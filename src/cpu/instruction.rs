use super::{CPU, AddressingMode, CPUFlag, Mem};



impl CPU {

    //==================================================================================
    //================================ basic operations ================================
    //==================================================================================
 
    fn page_cross(base: u16, new_target: u16) -> bool {
        base & 0xff00 != new_target & 0xff00
    }

    // Implementation of addressing modes
    pub fn get_address_from_mode(&mut self, mode: AddressingMode, new_pc: u16) -> (bool, u16) {
        match mode {
            AddressingMode::Immediate => (false, self.reg_pc.wrapping_add(1)),
            AddressingMode::Absolute => (false, self.mem_read_u16(self.reg_pc.wrapping_add(1))),
            AddressingMode::Relative => {
                let offset: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                let value: u16 = if offset < 127 { new_pc.wrapping_add(offset as u16) } else { new_pc.wrapping_sub(256 - offset as u16) };
                (CPU::page_cross(new_pc, value), self.reg_pc.wrapping_add(1))
            }
            AddressingMode::ZeroPage => (false, self.mem_read_u8(self.reg_pc.wrapping_add(1)) as u16),
            AddressingMode::ZeroPageX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                (false, pos.wrapping_add(self.reg_x) as u16)
            }
            AddressingMode::ZeroPageY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                (false, pos.wrapping_add(self.reg_y) as u16)
            }
            AddressingMode::AbsoluteX => {
                let pos: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
                let addr: u16 = pos.wrapping_add(self.reg_x as u16);
                (CPU::page_cross(pos, addr), addr)
            }
            AddressingMode::AbsoluteY => {
                let pos: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
                let addr = pos.wrapping_add(self.reg_y as u16);
                (CPU::page_cross(pos, addr), addr)
            }
            AddressingMode::IndirectX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                let addr: u8 = pos.wrapping_add(self.reg_x);
                let low: u8 = self.mem_read_u8(addr as u16);
                let high: u8 = self.mem_read_u8(addr.wrapping_add(1) as u16);
                (false, (high as u16) << 8 | (low as u16)) 
            }
            AddressingMode::IndirectY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                let low: u8 = self.mem_read_u8(pos as u16);
                let high: u8 = self.mem_read_u8(pos.wrapping_add(1) as u16);
                let addr_base: u16 = (high as u16) << 8 | (low as u16);
                let addr: u16 = addr_base.wrapping_add(self.reg_y as u16);
                (CPU::page_cross(addr_base, addr), addr)
            }
            AddressingMode::Indirect => {
                let pos: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
                let low: u8 = (pos & 0xff) as u8;
                let high: u8 = (pos >> 8) as u8;
                let pos2: u16 = (high as u16) << 8 | (low.wrapping_add(1) as u16);  
                let low: u8 = self.mem_read_u8(pos as u16);
                let high: u8 = self.mem_read_u8(pos2 as u16);
                (false, (high as u16) << 8 | (low as u16))
                
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

    pub(super) fn stack_push_u8(&mut self, value: u8) {
        self.mem_write_u8(self.stack_base + self.reg_sp as u16, value);
        self.reg_sp = self.reg_sp.wrapping_sub(1);
    }

    pub(super) fn stack_pop_u8(&mut self) -> u8 {
        self.reg_sp = self.reg_sp.wrapping_add(1);
        self.mem_read_u8(self.stack_base + self.reg_sp as u16)   
    }

    pub(super) fn stack_push_u16(&mut self, value: u16) {
        self.stack_push_u8((value >> 8) as u8);
        self.stack_push_u8((value & 0xff) as u8);
    }

    pub(super) fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop_u8();
        let high = self.stack_pop_u8();
        (high as u16) << 8 | low as u16
    }
    
    // ======================== FLAG MANIPULATION ========================

    pub(super) fn mask_from_flag(flag : CPUFlag) -> u8 {
        match flag {
            CPUFlag::Negative  => 0b1000_0000,
            CPUFlag::Overflow  => 0b0100_0000,
            CPUFlag::Break2    => 0b0010_0000,
            CPUFlag::Break     => 0b0001_0000,
            CPUFlag::Decimal   => 0b0000_1000,
            CPUFlag::InterruptDisabled => 0b0000_0100,
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


    pub(super) fn put_flag(&mut self, flag: CPUFlag, value: bool) {
        match value {
            true => self.set_flag(flag),
            false => self.unset_flag(flag)
        }
    }

    pub(super) fn set_flag(&mut self, flag: CPUFlag) {
        self.status |= CPU::mask_from_flag(flag);
    }

    pub(super) fn unset_flag(&mut self, flag: CPUFlag) {
        self.status &= !CPU::mask_from_flag(flag);
    }

    pub(super) fn get_flag(&self, flag: CPUFlag) -> bool {
        self.status & CPU::mask_from_flag(flag) != 0
    }




    //==================================================================================
    //================================ Official Instructions ===========================
    //==================================================================================
 
    pub(super) fn no_bind_yet(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
       panic!("This opcode is not binded yet !")
    }
   
    // Add with carry
    pub(super) fn adc(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {1} else {0} };
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
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
        if page_cross { 1 } else { 0 }
        
    }

    // Logical and between a value and Accumulator
    pub(super) fn and(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a &= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        if page_cross { 1 } else { 0 }
    }

    // Arithmetic shift left
    pub(super) fn asl(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let overflowing: bool;
        let res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                (res, overflowing) = self.reg_a.overflowing_mul(2);
                self.reg_a = res;
            }
            _ => {
                let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
                (res, overflowing) = self.mem_read_u8(pos).overflowing_mul(2);
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, overflowing);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
        0
    }

    // Branch on carry clear
    pub(super) fn bcc(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if !self.get_flag(CPUFlag::Carry) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Branch on carry set
    pub(super) fn bcs(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if self.get_flag(CPUFlag::Carry) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Branch on equal
    pub(super) fn beq(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if self.get_flag(CPUFlag::Zero) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Bit test
    pub(super) fn bit(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let value: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Zero, value & self.reg_a == 0);
        self.put_flag(CPUFlag::Overflow, value & CPU::mask_from_flag(CPUFlag::Overflow) != 0);
        self.put_flag(CPUFlag::Negative, value & CPU::mask_from_flag(CPUFlag::Negative) != 0);
        0
    }

    // Branch on minus
    pub(super) fn bmi(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if self.get_flag(CPUFlag::Negative) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Branch on not equal
    pub(super) fn bne(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if !self.get_flag(CPUFlag::Zero) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Branch on plus
    pub(super) fn bpl(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if !self.get_flag(CPUFlag::Negative) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Force break
    pub(super) fn brk(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.running = false;
        0
    }

    // Branch on overflow clear
    pub(super) fn bvc(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if !self.get_flag(CPUFlag::Overflow) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Branch on overflow set
    pub(super) fn bvs(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if self.get_flag(CPUFlag::Overflow) {
            let offset: u8 = self.mem_read_u8(pos);
            self.jump_rel(offset);
            if page_cross { 2 } else { 1 }
        } else { 0 }
    }

    // Clear carry flag
    pub(super) fn clc(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.unset_flag(CPUFlag::Carry);
        0
    }

    // Clear decimal mode
    pub(super) fn cld(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.unset_flag(CPUFlag::Decimal);
        0
    }

    // Clear interrupt disable
    pub(super) fn cli(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.unset_flag(CPUFlag::InterruptDisabled);
        0
    }

    // Clear overflow flag
    pub(super) fn clv(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.unset_flag(CPUFlag::Overflow);
        0
    }

    // Compare
    pub(super) fn cmp(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_a >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_a == to_compare);
        self.put_flag(CPUFlag::Negative, ((self.reg_a.wrapping_sub(to_compare)) as i8) < 0);
        if page_cross { 1 } else { 0 }
    }

    // Compare X register
    pub(super) fn cpx(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_x >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_x == to_compare);
        self.put_flag(CPUFlag::Negative, ((self.reg_x.wrapping_sub(to_compare)) as i8) < 0);
        0
    }

    // Compare Y register
    pub(super) fn cpy(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_y >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_y == to_compare);
        self.put_flag(CPUFlag::Negative, ((self.reg_y.wrapping_sub(to_compare)) as i8) < 0);
        0
    }

    // Decrement memory
    pub(super) fn dec(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let value: u8 = self.mem_read_u8(pos).wrapping_sub(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
        0
    }

    // Decrement X register
    pub(super) fn dex(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_x = self.reg_x.wrapping_sub(1);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
        0

    }

    // Decrement Y register
    pub(super) fn dey(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_y = self.reg_y.wrapping_sub(1);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
        0
    }

    // Logical xor between a value and Accumulator
    pub(super) fn eor(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a ^= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        if page_cross { 1 } else { 0 }
    }

    // Increment memory
    pub(super) fn inc(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let value: u8 = self.mem_read_u8(pos).wrapping_add(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
        0
    }

    // Increment X register
    pub(super) fn inx(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_x = self.reg_x.wrapping_add(1);

        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
        0
    }

    // Increment Y register
    pub(super) fn iny(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_y = self.reg_y.wrapping_add(1);

        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
        0
    }

    // Jump to a spectified address
    pub(super) fn jmp(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        // Substracts 3 to balance the +3 after the instruction
        self.reg_pc = pos.wrapping_sub(3);
        0
    }

    // Jump to a subroutine
    pub(super) fn jsr(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);

        // We add two to handle the 3-bit sized instruction 
        self.stack_push_u16(self.reg_pc + 2);

        // Substracts 3 to balance the +3 after the instruction
        self.reg_pc = pos.wrapping_sub(3);
        0
    }

    // Loads operand into Accumulator
    pub(super) fn lda(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        if page_cross { 1 } else { 0 }
    }

    // Loads operand into X register
    pub(super) fn ldx(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_x = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
        if page_cross { 1 } else { 0 }
    }

    // Loads operand into Y register
    pub(super) fn ldy(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_y = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
        if page_cross { 1 } else { 0 }
    }

    // Logical shift right
    pub(super) fn lsr(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let old_value: u8;
        let res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                old_value = self.reg_a;
                res = old_value / 2;
                self.reg_a = res;
                
            }
            _ => {
                let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
                old_value = self.mem_read_u8(pos);
                res = old_value / 2;
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, old_value & 0b0000_0001 != 0);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
        0
    }

    pub(super) fn nop(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize { 0 }

    // Logical or between a value and Accumulator
    pub(super) fn ora(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a |= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        if page_cross { 1 } else { 0 }
    }

    // Push Accumulator on stack
    pub(super) fn pha(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.stack_push_u8(self.reg_a);
        0
    }

    // Push Processor status on stack
    pub(super) fn php(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        let mut flags: u8 = self.status.clone();
        flags |= CPU::mask_from_flag(CPUFlag::Break);
        flags |= CPU::mask_from_flag(CPUFlag::Break2);
        self.stack_push_u8(flags);
        0
    }

    // Pull Accumulator from stack
    pub(super) fn pla(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_a = self.stack_pop_u8();
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        0
    }

    // Pull Processor status from stack
    pub(super) fn plp(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.status = self.stack_pop_u8();
        self.unset_flag(CPUFlag::Break);
        self.set_flag(CPUFlag::Break2);
        0
    }

    // Rotate left
    pub(super) fn rol(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
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
                let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
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
        0
    }

    // Rotate right
    pub(super) fn ror(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
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
                let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
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
        0
    }

    // Return from interrupt
    pub(super) fn rti(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.status = self.stack_pop_u8();
        self.unset_flag(CPUFlag::Break);
        self.set_flag(CPUFlag::Break2);

        // Substracts 1 to balance the +1 after the instruction
        self.reg_pc = self.stack_pop_u16().wrapping_sub(1);
        0
    }

    // Return from subroutine
    pub(super) fn rts(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_pc = self.stack_pop_u16();
        0
    }

    // Subtract with carry
    pub(super) fn sbc(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {0} else {1} };
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
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
        if page_cross { 1 } else { 0 }
    }

    // Set carry flag
    pub(super) fn sec(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.set_flag(CPUFlag::Carry);
        0
    }

    // Set decimal flag
    pub(super) fn sed(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.set_flag(CPUFlag::Decimal);
        0
    }

    // Set interruption disable flag
    pub(super) fn sei(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.set_flag(CPUFlag::InterruptDisabled);
        0
    }

    // Store Accumulator in memory
    pub(super) fn sta(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.mem_write_u8(pos, self.reg_a);
        0
    }

    // Store X register in memory
    pub(super) fn stx(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.mem_write_u8(pos, self.reg_x);
        0
    }

    // Store Y register in memory
    pub(super) fn sty(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.mem_write_u8(pos, self.reg_y);
        0
    }    

    // Transfer Accumulator to X register
    pub(super) fn tax(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_x = self.reg_a;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
        0
    }

    // Transfer Accumulator to Y register
    pub(super) fn tay(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_y = self.reg_a;
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
        0
    }

    // Transfer SP register to X register
    pub(super) fn tsx(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_x = self.reg_sp;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
        0
    }


    // Transfer X register to Accumulator
    pub(super) fn txa(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_a = self.reg_x;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        0
    }


    // Transfer X register to SP register
    pub(super) fn txs(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_sp = self.reg_x;
        0
    }



    // Transfer Y register to Accumulator
    pub(super) fn tya(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {
        self.reg_a = self.reg_y;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        0
    
    }

    // ===============================================================================
    // ============================== Unofficial Instructions ========================
    // ===============================================================================

    // And with Accumulator and update carry flag
    pub(super) fn aac(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        self.and(addressmode, new_pc);
        self.put_flag(CPUFlag::Carry, self.get_flag(CPUFlag::Negative));
        0
    }

    // And with Accumulator and transfer to X register
    pub(super) fn aax(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let mut x: u8 = self.reg_x;
        x &= self.reg_a;
        self.mem_write_u8(pos, x);
        0
    }

    // And with Accumulator, Rotate 1 bit right and set specific Carry/Overflow flags
    pub(super) fn arr(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a &= self.mem_read_u8(pos);

        //rotation
        self.reg_a = self.reg_a >> 1;
        if self.get_flag(CPUFlag::Carry) {
            self.reg_a |= 0b1000_0000;
        }

        // update flags
        match self.reg_a & 0b0110_0000 {
            0b0110_0000 =>{
                self.set_flag(CPUFlag::Carry);
                self.unset_flag(CPUFlag::Overflow);
            }
            0b0000_0000 =>{
                self.unset_flag(CPUFlag::Carry);
                self.unset_flag(CPUFlag::Overflow);
            }
            0b0010_0000 =>{
                self.unset_flag(CPUFlag::Carry);
                self.set_flag(CPUFlag::Overflow);
            }
            0b0100_0000 =>{
                self.set_flag(CPUFlag::Carry);
                self.set_flag(CPUFlag::Overflow);
            }
            _ => panic!("Should not happen")
        }
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a); 
        0
    }

    // And with Accumulator, Rotate 1 bit right
    pub(super) fn asr(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a &= self.mem_read_u8(pos);
        let new_carry = self.reg_a & 0b0000_0001 == 0b0000_0001;

        //rotation
        self.reg_a = self.reg_a >> 1;
        if self.get_flag(CPUFlag::Carry) {
            self.reg_a |= 0b1000_0000;
        }

        self.put_flag(CPUFlag::Carry, new_carry); 
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        0
    }

    // And with Accumulator, transfer Accumulator to X
    pub(super) fn atx(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        self.and(addressmode, new_pc);
        self.reg_x = self.reg_a;
        0
    }

    // write (X and Accumulator and 7)  in memory 
    pub(super) fn axa(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.mem_write_u8(pos, (self.reg_a & self.reg_x) & 7 );
        0
    }

    // write (X & A - addrValue) in X
    pub(super) fn axs(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize { 
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_x = (self.reg_a & self.reg_x).wrapping_sub(self.mem_read_u8(pos));

        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
        self.put_flag(CPUFlag::Carry, !self.get_flag(CPUFlag::Negative));
        0
    }

    // Substract 1 from mem
    pub(super) fn dcp(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let value: u8 = self.mem_read_u8(pos);
        let to_write: u8 = value.wrapping_sub(1);
        self.mem_write_u8(pos, to_write);

        // let value: bool = (to_write & 0b0000_0001) == 0;
        // self.put_flag(CPUFlag::Carry, value);
        self.put_flag(CPUFlag::Carry, self.reg_a >= to_write);
        self.put_flag(CPUFlag::Zero, self.reg_a == to_write);
        self.put_flag(CPUFlag::Negative, ((self.reg_a.wrapping_sub(to_write)) as i8) < 0);
        0
    }

    // Do nothing
    pub(super) fn dop(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize { 0 }

    // Increment mem then substract it from Accumulator
    pub(super) fn isc(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let to_write: u8 = self.mem_read_u8(pos).wrapping_add(1);
        self.mem_write_u8(pos, to_write);

        self.update_n_flag(to_write);
        self.update_z_flag(to_write);




        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {0} else {1} };
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
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
        0
    }

    // Stop Program Counter (kill)
    // implement it in the loop?
    pub(super) fn kil(&mut self, _addressmode: AddressingMode, _new_pc: u16) -> usize {self.running = false; 0 }

    // And with stack pointer, copy in sp, registers A and X
    pub(super) fn lar(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_sp &= self.mem_read_u8(pos);
        self.reg_a = self.reg_sp;
        self.reg_x = self.reg_sp;

        self.update_n_flag(self.reg_sp);
        self.update_z_flag(self.reg_sp);
        if page_cross { 1 } else { 0 }    
    }

    // Load mem to Accumulator and X
    pub(super) fn lax(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a = self.mem_read_u8(pos);
        self.reg_x = self.mem_read_u8(pos);

        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        if page_cross { 1 } else { 0 }    
    }

    // Rotate 1 bit let, And and store in accumulator 
    pub(super) fn rla(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let new_carry = self.mem_read_u8(pos) & 0b1000_0000 == 0b1000_0000;
        
        let mut tmp_mem = self.mem_read_u8(pos) << 1;
        if self.get_flag(CPUFlag::Carry) {tmp_mem |= 0b0000_0001;}
        self.mem_write_u8(pos, tmp_mem);

        self.reg_a &= tmp_mem;

        self.put_flag(CPUFlag::Carry, new_carry);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        0
    }

    // Rotate 1 bit right in mem, then add to Accumulator
    pub(super) fn rra(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
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
                let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
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

        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {1} else {0} };
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
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
        0



    }
    
    // Shift 1 bit left in mem, then OR with Accumulator 
    pub(super) fn slo(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let new_carry = self.mem_read_u8(pos) & 0b1000_0000 == 0b1000_0000;

        let value: u8 = self.mem_read_u8(pos) << 1;
        self.mem_write_u8(pos, value);

        self.reg_a |= self.mem_read_u8(pos);

        self.put_flag(CPUFlag::Carry, new_carry);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        0
    }

    // Shift 1 bit right in mem, then XOR with Accumulator
    pub(super) fn sre(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        let new_carry = self.mem_read_u8(pos) & 0b0000_0001 == 0b0000_0001;

        let value: u8 = self.mem_read_u8(pos) >> 1;
        self.mem_write_u8(pos, value);

        self.reg_a ^= self.mem_read_u8(pos);

        self.put_flag(CPUFlag::Carry, new_carry);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);   
        0    
    }

    // And the high byte of addr with X, add 1 and store in mem
    pub(super) fn sxa(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
       let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
       self.mem_write_u8(pos, (self.reg_x & (pos >> 8) as u8) + 1 ); 
       0
    }

    // And the high byte of addr with Y, add 1 and store in mem
    pub(super) fn sya(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.mem_write_u8(pos, (self.reg_y & (pos >> 8) as u8) + 1 );
        0
    }

    // Do nothing
    pub(super) fn top(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (page_cross, _): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        if page_cross { 1 } else { 0 }
    }

    // And mem And X And Accumulator, store in Accumulator
    pub(super) fn xaa(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);
        self.reg_a &= self.reg_x & self.mem_read_u8(pos);

        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        0
    }

    // And X with Accumulator, store in Sp, And result with high byte of addr, store in mem
    pub(super) fn xas(&mut self, addressmode: AddressingMode, new_pc: u16) -> usize {
        let (_page_cross, pos): (bool, u16) = self.get_address_from_mode(addressmode, new_pc);

        self.reg_sp = self.reg_a & self.reg_x;

        self.mem_write_u8(pos, (self.reg_sp & (pos >> 8) as u8 ) + 1 );

        self.update_n_flag(self.reg_sp);
        self.update_z_flag(self.reg_sp);
        0
    }
 
}