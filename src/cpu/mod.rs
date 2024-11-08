use cast::u8;

pub mod opcode;
use opcode::{OPCODES, AddressingMode};


#[derive(Debug)]
pub struct CPU {
    pub reg_pc          : u16,
    pub reg_sp          : u8,
    pub reg_a           : u8,
    pub reg_x           : u8,
    pub reg_y           : u8,
    pub status          : u8,
    pub memory          : [u8; 0xffff],
    pub stack_base      : u16, 
    pub program_base    : u16
}

#[derive(Debug)]
pub enum CPUFlag {
    Negative,
    Overflow,
    Break,
    Decimal,
    Interrupt,
    Zero,
    Carry,
}

impl CPU {

    // ===================================================================
    // ============================= API =================================
    // ===================================================================

    pub fn new() -> Self {
        CPU {
            reg_pc : 0,
            reg_sp : 0,
            reg_a  : 0,
            reg_x  : 0,
            reg_y  : 0,
            status : 0,
            memory : [0; 0xffff],
            stack_base : 0x0100,
            program_base : 0x8000
        }
    }

    pub fn mem_read_u8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    // Handles little endian
    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        (self.memory[addr.wrapping_add(1) as usize] as u16) << 8 | (self.memory[addr as usize] as u16)
    }

    pub fn mem_write_u8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    // Handles little endian
    pub fn mem_write_u16(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = u8(value & 0xff).expect("The logical and of the value and 0xff didn't work for cast (this should never happend)");
        self.memory[addr.wrapping_add(1) as usize] = u8(value >> 8).expect("The logical right shift of the value and 8 didn't work for cast (this should never happend)");
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.reg_sp = 0xff;
        self.status = 0;

        self.reg_pc = self.mem_read_u16(0xfffc);
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        self.memory[(self.program_base as usize) .. ((self.program_base as usize) + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xfffc, self.program_base);
    }

    pub fn load_and_run(&mut self, program: &Vec<u8>) {
        self.load_program(program);
        self.reset();
        // println!("{:?}", self);
        self.run();
    }

    pub fn run(&mut self) {
       self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where F: FnMut(&mut CPU) {
        loop {
            callback(self);

            let opcode: u8 = self.mem_read_u8(self.reg_pc);
            // println!("opcode {:?} at {:x}", opcode, self.reg_pc);
            OPCODES[opcode as usize].exec(self);
            if self.status & CPU::mask_from_flag(CPUFlag::Break) != 0 {
                break;
            }
         }
         println!("Execution is over !\n");
     }
    
    // ===================================================================
    // ======================== FLAG MANIPULATION ========================
    // ===================================================================

    pub fn mask_from_flag(flag : CPUFlag) -> u8 {
        match flag {
            CPUFlag::Negative  => 0b1000_0000,
            CPUFlag::Overflow  => 0b0100_0000,
            CPUFlag::Break     => 0b0001_0000,
            CPUFlag::Decimal   => 0b0000_1000,
            CPUFlag::Interrupt => 0b0000_0100,
            CPUFlag::Zero      => 0b0000_0010,
            CPUFlag::Carry     => 0b0000_0001,
        }
    }


    fn update_z_flag (&mut self, value: u8) {
        self.put_flag(CPUFlag::Zero, value == 0);
    }

    fn update_n_flag (&mut self, value: u8) {
        self.put_flag(CPUFlag::Negative, value & CPU::mask_from_flag(CPUFlag::Negative) != 0);
    }


    fn put_flag(&mut self, flag: CPUFlag, value: bool) {
        match value {
            true => self.set_flag(flag),
            false => self.unset_flag(flag)
        }
    }

    fn set_flag(&mut self, flag: CPUFlag) {
        self.status |= CPU::mask_from_flag(flag);
    }

    fn unset_flag(&mut self, flag: CPUFlag) {
        self.status &= !CPU::mask_from_flag(flag);
    }

    fn get_flag(&self, flag: CPUFlag) -> bool {
        self.status & CPU::mask_from_flag(flag) != 0
    }

    // ===================================================================
    // ======================== INSTRUCTION SET ==========================
    // ===================================================================

    // Implementation of addressing modes
    fn get_address_from_mode(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate | AddressingMode::Absolute | AddressingMode::Relative => self.reg_pc.wrapping_add(1),
            AddressingMode::ZeroPage => self.mem_read_u8(self.reg_pc.wrapping_add(1)) as u16,
            AddressingMode::ZeroPageX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_y) as u16
            }
            AddressingMode::AbsoluteX => {
                let pos: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let pos: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::IndirectX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                let addr: u16 = pos.wrapping_add(self.reg_x) as u16;
                self.mem_read_u16(addr)
            }
            AddressingMode::IndirectY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                let addr: u16 = self.mem_read_u16(pos as u16);
                addr.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::Indirect => {
                self.mem_read_u16(self.reg_pc.wrapping_add(1))
            }

            AddressingMode::Accumulator | AddressingMode::Implied | AddressingMode::NoneAddressing => {
                panic!("Mode : {:?} is not supported", mode);
            }

        }
    }

    fn jump_rel(&mut self, offset: u8) {
        if offset & CPU::mask_from_flag(CPUFlag::Negative) != 0 {
            self.reg_pc = self.reg_pc.wrapping_sub(256 - offset as u16);
        } else {
            self.reg_pc = self.reg_pc.wrapping_add(offset as u16);
        }
    }

    fn stack_push_u8(&mut self, value: u8) {
        self.mem_write_u8(self.stack_base + self.reg_sp as u16, value);
        self.reg_sp = self.reg_sp.wrapping_sub(1);
    }

    fn stack_pop_u8(&mut self) -> u8 {
        self.reg_sp = self.reg_sp.wrapping_add(1);
        self.mem_read_u8(self.stack_base + self.reg_sp as u16)   
    }

    fn stack_push_u16(&mut self, value: u16) {
        self.stack_push_u8(u8(value & 0xff).expect("The logical and of the value and 0xff didn't work for cast (this should never happend)"));
        self.stack_push_u8(u8(value >> 8).expect("The logical right shift of the value and 8 didn't work for cast (this should never happend)"));
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let high = self.stack_pop_u8();
        let low = self.stack_pop_u8();
        (high as u16) << 8 | low as u16
    }

    pub fn show_stack(&self) {
        for i in 0x00..0x100 {
            println!("0x{:02x}: {:02x}", i, self.memory[(self.stack_base + i) as usize]);
        }
    }
    
    fn no_bind_yet(&mut self, _addressmode: AddressingMode) {
       panic!("This opcode is not binded yet !")
    }

    // Add with carry
    fn adc(&mut self, addressmode: AddressingMode) {
        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {1} else {0} };
        let pos: u16 = self.get_address_from_mode(addressmode);
        let overflowed: bool;
        let overflowed2: bool;

        let base_a: u8 = self.reg_a;
        let to_add: u8 = self.mem_read_u8(pos);

        (self.reg_a, overflowed) = self.reg_a.overflowing_add(to_add);
        (self.reg_a, overflowed2) = self.reg_a.overflowing_add(carry);
        
        self.put_flag(CPUFlag::Carry, overflowed | overflowed2);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);

        // Set overflow if we add two positive (negative) integers which result to a negative (positive) integer
        // First parenthesis (with negation) has MSB set if base_a and to_add have the same MSB
        // Second parenthesis has MSB set if base_a and the result have different MSB 
        self.put_flag(CPUFlag::Overflow, (!(base_a ^ to_add) & (base_a ^ self.reg_a) & 0b1000_0000) != 0);
        
    }

    // Logical and between a value and Accumulator
    fn and(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a &= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Arithmetic shift left
    fn asl(&mut self, addressmode: AddressingMode) {
        let overflowing: bool;
        let res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                (res, overflowing) = self.reg_a.overflowing_mul(2);
                self.reg_a = res;
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                (res, overflowing) = self.mem_read_u8(pos).overflowing_mul(2);
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, overflowing);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    // Branch on carry clear
    fn bcc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Carry) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on carry set
    fn bcs(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Carry) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on equal
    fn beq(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Zero) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Bit test
    fn bit(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Zero, value & self.reg_a == 0);
        self.put_flag(CPUFlag::Overflow, value & CPU::mask_from_flag(CPUFlag::Overflow) == 1);
        self.put_flag(CPUFlag::Negative, value & CPU::mask_from_flag(CPUFlag::Negative) == 1);
    }

    // Branch on minus
    fn bmi(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Negative) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on not equal
    fn bne(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Zero) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on plus
    fn bpl(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Negative) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Force break
    fn brk(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Break);
    }

    // Branch on overflow clear
    fn bvc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Overflow) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on overflow set
    fn bvs(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Overflow) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Clear carry flag
    fn clc(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Carry);
    }

    // Clear decimal mode
    fn cld(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Decimal);
    }

    // Clear interrupt disable
    fn cli(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Interrupt);
    }

    // Clear overflow flag
    fn clv(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Overflow);
    }

    // Compare
    fn cmp(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_a >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_a == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_a <= to_compare);
    }

    // Compare X register
    fn cpx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_x >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_x == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_x <= to_compare);
    }

    // Compare Y register
    fn cpy(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_y >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_y == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_y <= to_compare);
    }

    // Decrement memory
    fn dec(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos).wrapping_sub(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
    }

    // Decrement X register
    fn dex(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_x.wrapping_sub(1);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);

    }

    // Decrement Y register
    fn dey(&mut self, _addressmode: AddressingMode) {
        self.reg_y = self.reg_y.wrapping_sub(1);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Logical xor between a value and Accumulator
    fn eor(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a ^= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Increment memory
    fn inc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos).wrapping_add(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
    }

    // Increment X register
    fn inx(&mut self, _addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_x, overflowed) = self.reg_x.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Increment Y register
    fn iny(&mut self, _addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_y, overflowed) = self.reg_y.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Jump to a spectified address
    fn jmp(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        // Substracts 3 to balance the +3 after the instruction
        self.reg_pc = self.mem_read_u16(pos).wrapping_sub(3);
    }

    // Jump to a subroutine
    fn jsr(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);

        // We add two to handle the 3-bit sized instruction 
        self.stack_push_u16(self.reg_pc + 3);

        // Substracts 3 to balance the +3 after the instruction
        self.reg_pc = self.mem_read_u16(pos).wrapping_sub(3);
    }

    // Loads operand into Accumulator
    fn lda(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Loads operand into X register
    fn ldx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_x = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Loads operand into Y register
    fn ldy(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_y = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Logical shift right
    fn lsr(&mut self, addressmode: AddressingMode) {
        let old_value: u8;
        let res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                old_value = self.reg_a;
                res = old_value / 2;
                self.reg_a = res;
                
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                old_value = self.mem_read_u8(pos);
                res = old_value / 2;
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, old_value & 0b0000_0001 != 0);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    fn nop(&mut self, _addressmode: AddressingMode) {}

    // Logical or between a value and Accumulator
    fn ora(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a |= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Push Accumulator on stack
    fn pha(&mut self, _addressmode: AddressingMode) {
        self.stack_push_u8(self.reg_a);
    }

    // Push Processor status on stack
    fn php(&mut self, _addressmode: AddressingMode) {
        self.stack_push_u8(self.status);
    }

    // Pull Accumulator from stack
    fn pla(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.stack_pop_u8();
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Pull Processor status from stack
    fn plp(&mut self, _addressmode: AddressingMode) {
        self.status = self.stack_pop_u8();
    }

    // Rotate left
    fn rol(&mut self, addressmode: AddressingMode) {
        let overflowing: bool;
        let mut res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                (res, overflowing) = self.reg_a.overflowing_mul(2);
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b0000_0001;
                }
                self.reg_a = res;
                
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                (res, overflowing) = self.mem_read_u8(pos).overflowing_mul(2);
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b0000_0001;
                }
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, overflowing);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    // Rotate right
    fn ror(&mut self, addressmode: AddressingMode) {
        let old_value: u8;
        let mut res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                old_value = self.reg_a;
                res = old_value / 2;
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b1000_0000;
                }
                self.reg_a = res;
                
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                old_value = self.mem_read_u8(pos);
                res = old_value / 2;
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b1000_0000;
                }
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, old_value & 0b0000_0001 != 0);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    // Return from interrupt
    fn rti(&mut self, _addressmode: AddressingMode) {
        self.status = self.stack_pop_u8();

        // Substracts 1 to balance the +1 after the instruction
        self.reg_pc = self.stack_pop_u16().wrapping_sub(1);
    }

    // Return from subroutine
    fn rts(&mut self, _addressmode: AddressingMode) {

        // Substracts 1 to balance the +1 after the instruction
        self.reg_pc = self.stack_pop_u16().wrapping_sub(1);
    }

    // Subtract with carry
    fn sbc(&mut self, addressmode: AddressingMode) {
        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {0} else {1} };
        let pos: u16 = self.get_address_from_mode(addressmode);
        let overflowed: bool;
        let overflowed2: bool;

        let base_a: u8 = self.reg_a;
        let to_sub: u8 = self.mem_read_u8(pos);

        (self.reg_a, overflowed) = self.reg_a.overflowing_sub(to_sub);
        (self.reg_a, overflowed2) = self.reg_a.overflowing_sub(carry);
        
        self.put_flag(CPUFlag::Carry, !(overflowed | overflowed2));
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);

        // Set overflow if we add two positive (negative) integers which result to a negative (positive) integer
        // First parenthesis has MSB set if base_a and to_add have the different MSB (+/- or -/+)
        // Second parenthesis has MSB set if base_a and the result have different MSB (+/- or -/+)
        // They are both set if we substract a negative to a positive and result is negative (or the contrary)
        self.put_flag(CPUFlag::Overflow, ((base_a ^ to_sub) & (base_a ^ self.reg_a) & 0b1000_0000) != 0);
    }

    // Set carry flag
    fn sec(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Carry);
    }

    // Set decimal flag
    fn sed(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Decimal);
    }

    // Set interruption disable flag
    fn sei(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Interrupt);
    }

    // Store Accumulator in memory
    fn sta(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_a);
    }

    // Store X register in memory
    fn stx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_x);
    }

    // Store Y register in memory
    fn sty(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_y);
    }    

    // Transfer Accumulator to X register
    fn tax(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_a;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Transfer Accumulator to Y register
    fn tay(&mut self, _addressmode: AddressingMode) {
        self.reg_y = self.reg_a;
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Transfer SP register to X register
    fn tsx(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_sp;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }


    // Transfer X register to Accumulator
    fn txa(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.reg_x;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }


    // Transfer X register to SP register
    fn txs(&mut self, _addressmode: AddressingMode) {
        self.reg_sp = self.reg_x;
    }



    // Transfer Y register to Accumulator
    fn tya(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.reg_y;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    
    }
     
}







// ==================================================================================================
// ============================================ UNIT TESTS ==========================================
// ==================================================================================================


#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    impl CPU {
        pub fn test_prog(program: Vec<u8>) -> Self {
            let mut cpu = CPU::new();
            cpu.load_and_run(&program);
            cpu
        }
    }

    #[test]
    fn test_immediate_lda() {
        let cpu = CPU::test_prog(vec![0xa9, 0xc0, 0x00]);
        assert_eq!(cpu.reg_a, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa9, 0x12, 0x00]);
        assert_eq!(cpu.reg_a, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_immediate_ldx() {
        let cpu = CPU::test_prog(vec![0xa2, 0xc0, 0x00]);
        assert_eq!(cpu.reg_x, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa2, 0x12, 0x00]);
        assert_eq!(cpu.reg_x, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0x00]);
        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_immediate_ldy() {
        let cpu = CPU::test_prog(vec![0xa0, 0xc0, 0x00]);
        assert_eq!(cpu.reg_y, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa0, 0x12, 0x00]);
        assert_eq!(cpu.reg_y, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa0, 0x00, 0x00]);
        assert_eq!(cpu.reg_y, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_inx() {
        let cpu = CPU::test_prog(vec![0xa2, 0x13, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x14);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);

        let cpu = CPU::test_prog(vec![0xa2, 0xff, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x00);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);
    }

    #[test]
    fn test_iny() {
        let cpu = CPU::test_prog(vec![0xa0, 0x13, 0xc8, 0x00]);
        assert_eq!(cpu.reg_y, 0x14);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);

        let cpu = CPU::test_prog(vec![0xa0, 0xff, 0xc8, 0x00]);
        assert_eq!(cpu.reg_y, 0x00);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);
    }

    #[test]
    fn test_jmp_and_branches() {
        // Jump
        let cpu = CPU::test_prog(vec![0xe8, 0x4c, 0x06, 0x80, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x03);
        assert_eq!(cpu.reg_pc, 0x8009);

        let cpu = CPU::test_prog(vec![0xe8, 0x30, 0x03, 0x6c, 0xfc, 0xff, 0x00]);
        assert_eq!(cpu.reg_x, 0x80);
        assert_eq!(cpu.reg_pc, 0x8007);
        
        // Carry set
        let cpu = CPU::test_prog(vec![0x38, 0xb0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8008);

        let cpu = CPU::test_prog(vec![0xea, 0xb0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8005);

        // Carry clear
        let cpu = CPU::test_prog(vec![0x38, 0x90, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8005);

        let cpu = CPU::test_prog(vec![0xea, 0x90, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8008);

        // Negative set
        let cpu = CPU::test_prog(vec![0xa2, 0xff, 0x30, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8009);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0x30, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8006);

        // Negative clear
        let cpu = CPU::test_prog(vec![0xa2, 0xff, 0x10, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.reg_pc, 0x8006);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0x10, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8009);

        // Zero set
        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0xf0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8009);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xf0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8006);

        // Zero clear
        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0xd0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8006);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xd0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8009);

        // Overflow set
        let cpu = CPU::test_prog(vec![0xa9, 0x7f, 0x69, 0x7f, 0x70, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x800b);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xea, 0xea, 0x70, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8008);

        // Overflow clear
        let cpu = CPU::test_prog(vec![0xa9, 0x7f, 0x69, 0x7f, 0x50, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8008);
        
        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xea, 0xea, 0x50, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x800b);

        // Negative branching
        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0xf0, 0xfd, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.reg_pc, 0x8002);

    }

    #[test]
    fn test_shifts() {
        // ASL
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x0a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0101_0000);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b1110_1000, 0x0a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1101_0000);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        // LSR
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x4a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1001, 0x4a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        // ROL
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0101_0000);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b1110_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1101_0000);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b0010_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0101_0001);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b1110_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1101_0001);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        // ROR
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1001, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b0010_1000, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1001_0100);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b0010_1001, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1001_0100);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

    }

    #[test]
    fn test_adc_sbc() {
        // ADC
        let cpu = CPU::test_prog(vec![0xa9, 0x18, 0x69, 0x12, 0x00]);
        assert_eq!(cpu.reg_a, 0x2a);
        assert!(!cpu.get_flag(CPUFlag::Overflow));

        let cpu = CPU::test_prog(vec![0xa9, 0x81, 0x69, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0x49);
        assert!(cpu.get_flag(CPUFlag::Overflow));

        let cpu = CPU::test_prog(vec![0xa9, 0x51, 0x69, 0x7f, 0x00]);
        assert_eq!(cpu.reg_a, 0xd0);
        assert!(cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0x81, 0x69, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0x4a);
        assert!(cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Carry));

        // SBC
        let cpu = CPU::test_prog(vec![0xa9, 0x18, 0xe9, 0x12, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(!cpu.get_flag(CPUFlag::Overflow));

        let cpu = CPU::test_prog(vec![0xa9, 0x81, 0xe9, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0xb8);
        assert!(!cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0x51, 0xe9, 0x80, 0x00]);
        assert_eq!(cpu.reg_a, 0xd0);
        assert!(cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0x81, 0xe9, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0xb9);
        assert!(!cpu.get_flag(CPUFlag::Overflow));
        assert!(!cpu.get_flag(CPUFlag::Carry));
    }

    #[test]
    fn test_stack() {
        // PHA and PLA
        let cpu = CPU::test_prog(vec![0xa9, 0x18, 0x48, 0xa9, 0x12, 0x48, 0xa9, 0xff, 0x68, 0xaa, 0x68, 0xa8, 0x00]);
        assert_eq!(cpu.reg_x, 0x12);
        assert_eq!(cpu.reg_y, 0x18);

        // JSR and RTS
        let cpu = CPU::test_prog(vec![0xe8, 0x20, 0x05, 0x80, 0x00, 0xe8, 0xe8, 0x60, 0x00]);
        cpu.show_stack();
        assert_eq!(cpu.reg_x, 0x03);
        assert_eq!(cpu.reg_pc, 0x8005);
    }


    #[test]
    fn test_misc() {
        // Call a function that adds X and Y to A
        let cpu = CPU::test_prog(vec![0xa2, 0x12, 0xa0, 0x34, 0x20, 0x0a, 0x80, 0x00, 0xa9, 0x00, 0x86, 0x00, 0x98, 0x65, 0x00, 0x60, 0x00]);
        assert_eq!(cpu.reg_a, 0x46);
        assert_eq!(cpu.reg_pc, 0x8008);
    }
    

}