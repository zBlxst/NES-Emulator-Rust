use super::{CPU, AddressingMode, CPUFlag, Mem};



impl CPU {

    //==================================================================================
    //================================ basic operations ================================
    //==================================================================================
 
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
        self.stack_push_u8((value & 0xff) as u8);
        self.stack_push_u8((value >> 8) as u8);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let high = self.stack_pop_u8();
        let low = self.stack_pop_u8();
        (high as u16) << 8 | low as u16
    }

    pub fn show_stack(&self) {
        for i in 0x00..0x100 {
            println!("0x{:02x}: {:02x}", i, self.mem_read_u8(self.stack_base + i))
        }
       
    }
    
    // ======================== FLAG MANIPULATION ========================

    pub(super) fn mask_from_flag(flag : CPUFlag) -> u8 {
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

    pub(super) fn get_flag(&self, flag: CPUFlag) -> bool {
        self.status & CPU::mask_from_flag(flag) != 0
    }




    //==================================================================================
    //================================ Official Instructions ===========================
    //==================================================================================
 
    pub(super) fn no_bind_yet(&mut self, _addressmode: AddressingMode) {
       panic!("This opcode is not binded yet !")
    }
   
    // Add with carry
    pub(super) fn adc(&mut self, addressmode: AddressingMode) {
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
    pub(super) fn and(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a &= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Arithmetic shift left
    pub(super) fn asl(&mut self, addressmode: AddressingMode) {
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
    pub(super) fn bcc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Carry) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on carry set
    pub(super) fn bcs(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Carry) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on equal
    pub(super) fn beq(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Zero) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Bit test
    pub(super) fn bit(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Zero, value & self.reg_a == 0);
        self.put_flag(CPUFlag::Overflow, value & CPU::mask_from_flag(CPUFlag::Overflow) == 1);
        self.put_flag(CPUFlag::Negative, value & CPU::mask_from_flag(CPUFlag::Negative) == 1);
    }

    // Branch on minus
    pub(super) fn bmi(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Negative) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on not equal
    pub(super) fn bne(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Zero) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on plus
    pub(super) fn bpl(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Negative) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Force break
    pub(super) fn brk(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Break);
    }

    // Branch on overflow clear
    pub(super) fn bvc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Overflow) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on overflow set
    pub(super) fn bvs(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Overflow) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Clear carry flag
    pub(super) fn clc(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Carry);
    }

    // Clear decimal mode
    pub(super) fn cld(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Decimal);
    }

    // Clear interrupt disable
    pub(super) fn cli(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Interrupt);
    }

    // Clear overflow flag
    pub(super) fn clv(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Overflow);
    }

    // Compare
    pub(super) fn cmp(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_a >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_a == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_a <= to_compare);
    }

    // Compare X register
    pub(super) fn cpx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_x >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_x == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_x <= to_compare);
    }

    // Compare Y register
    pub(super) fn cpy(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_y >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_y == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_y <= to_compare);
    }

    // Decrement memory
    pub(super) fn dec(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos).wrapping_sub(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
    }

    // Decrement X register
    pub(super) fn dex(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_x.wrapping_sub(1);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);

    }

    // Decrement Y register
    pub(super) fn dey(&mut self, _addressmode: AddressingMode) {
        self.reg_y = self.reg_y.wrapping_sub(1);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Logical xor between a value and Accumulator
    pub(super) fn eor(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a ^= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Increment memory
    pub(super) fn inc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos).wrapping_add(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
    }

    // Increment X register
    pub(super) fn inx(&mut self, _addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_x, overflowed) = self.reg_x.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Increment Y register
    pub(super) fn iny(&mut self, _addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_y, overflowed) = self.reg_y.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Jump to a spectified address
    pub(super) fn jmp(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        // Substracts 3 to balance the +3 after the instruction
        self.reg_pc = self.mem_read_u16(pos).wrapping_sub(3);
    }

    // Jump to a subroutine
    pub(super) fn jsr(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);

        // We add two to handle the 3-bit sized instruction 
        self.stack_push_u16(self.reg_pc + 3);

        // Substracts 3 to balance the +3 after the instruction
        self.reg_pc = self.mem_read_u16(pos).wrapping_sub(3);
    }

    // Loads operand into Accumulator
    pub(super) fn lda(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Loads operand into X register
    pub(super) fn ldx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_x = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Loads operand into Y register
    pub(super) fn ldy(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_y = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Logical shift right
    pub(super) fn lsr(&mut self, addressmode: AddressingMode) {
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

    pub(super) fn nop(&mut self, _addressmode: AddressingMode) {}

    // Logical or between a value and Accumulator
    pub(super) fn ora(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a |= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Push Accumulator on stack
    pub(super) fn pha(&mut self, _addressmode: AddressingMode) {
        self.stack_push_u8(self.reg_a);
    }

    // Push Processor status on stack
    pub(super) fn php(&mut self, _addressmode: AddressingMode) {
        self.stack_push_u8(self.status);
    }

    // Pull Accumulator from stack
    pub(super) fn pla(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.stack_pop_u8();
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Pull Processor status from stack
    pub(super) fn plp(&mut self, _addressmode: AddressingMode) {
        self.status = self.stack_pop_u8();
    }

    // Rotate left
    pub(super) fn rol(&mut self, addressmode: AddressingMode) {
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
    pub(super) fn ror(&mut self, addressmode: AddressingMode) {
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
    pub(super) fn rti(&mut self, _addressmode: AddressingMode) {
        self.status = self.stack_pop_u8();

        // Substracts 1 to balance the +1 after the instruction
        self.reg_pc = self.stack_pop_u16().wrapping_sub(1);
    }

    // Return from subroutine
    pub(super) fn rts(&mut self, _addressmode: AddressingMode) {

        // Substracts 1 to balance the +1 after the instruction
        self.reg_pc = self.stack_pop_u16().wrapping_sub(1);
    }

    // Subtract with carry
    pub(super) fn sbc(&mut self, addressmode: AddressingMode) {
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
    pub(super) fn sec(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Carry);
    }

    // Set decimal flag
    pub(super) fn sed(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Decimal);
    }

    // Set interruption disable flag
    pub(super) fn sei(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Interrupt);
    }

    // Store Accumulator in memory
    pub(super) fn sta(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_a);
    }

    // Store X register in memory
    pub(super) fn stx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_x);
    }

    // Store Y register in memory
    pub(super) fn sty(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_y);
    }    

    // Transfer Accumulator to X register
    pub(super) fn tax(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_a;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Transfer Accumulator to Y register
    pub(super) fn tay(&mut self, _addressmode: AddressingMode) {
        self.reg_y = self.reg_a;
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Transfer SP register to X register
    pub(super) fn tsx(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_sp;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }


    // Transfer X register to Accumulator
    pub(super) fn txa(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.reg_x;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }


    // Transfer X register to SP register
    pub(super) fn txs(&mut self, _addressmode: AddressingMode) {
        self.reg_sp = self.reg_x;
    }



    // Transfer Y register to Accumulator
    pub(super) fn tya(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.reg_y;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    
    }

    // ===============================================================================
    // ============================== Unofficial Instructions ========================
    // ===============================================================================

    // And with Accumulator and update carry flag
    pub(super) fn aac(&mut self, _addressmode: AddressingMode) {
        self.and(_addressmode);
        self.put_flag(CPUFlag::Carry, self.get_flag(CPUFlag::Negative));
    }

    // And with Accumulator and transfer to X register
    pub(super) fn aax(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        self.reg_x &= self.reg_a;
        self.mem_write_u8(pos, self.reg_x);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a)
    }

    // And with Accumulator, Rotate 1 bit right and set specific Carry/Overflow flags
    pub(super) fn arr(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
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
    }

    // And with Accumulator, Rotate 1 bit right
    pub(super) fn asr(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
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
    }

    // And with Accumulator, transfer Accumulator to X
    pub(super) fn atx(&mut self, _addressmode: AddressingMode) {
        self.and(_addressmode);
        self.reg_x = self.reg_a;
    }

    // write (X and Accumulator and 7)  in memory 
    pub(super) fn axa(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        self.mem_write_u8(pos, (self.reg_a & self.reg_x) & 7 );
    }

    // write (X & A - addrValue) in X
    // PROBABLY WRONG
    pub(super) fn axs(&mut self, _addressmode: AddressingMode) { 
        let pos: u16 = self.get_address_from_mode(_addressmode);
        self.reg_x = (self.reg_a & self.reg_x).wrapping_sub(self.mem_read_u8(pos));

        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
        self.put_flag(CPUFlag::Carry, !self.get_flag(CPUFlag::Negative));
    }

    // Substract 1 from mem
    pub(super) fn dcp(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        self.mem_write_u8(pos, self.mem_read_u8(pos).wrapping_sub(1));

        self.put_flag(CPUFlag::Carry, (self.mem_read_u8(pos) & 0b0000_0001) == 0);
    }

    // Do nothing
    pub(super) fn dop(&mut self, _addressmode: AddressingMode) {}

    // Increment mem then substract it from Accumulator
    // PROBABLY WRONG
    pub(super) fn isc(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        self.mem_write_u8(pos, self.mem_read_u8(pos).wrapping_add(1));

        let overflow:bool;
        (self.reg_a , overflow) = self.reg_a.overflowing_sub(self.mem_read_u8(pos));

        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
        self.put_flag(CPUFlag::Overflow, overflow);
        self.put_flag(CPUFlag::Carry, !self.get_flag(CPUFlag::Negative));
    }

    // Stop Program Counter (kill)
    // implement it in the loop?
    pub(super) fn kil(&mut self, _addressmode: AddressingMode) {self.set_flag(CPUFlag::Break);}

    // And with stack pointer, copy in sp, registers A and X
    pub(super) fn lar(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        self.reg_sp &= self.mem_read_u8(pos);
        self.reg_a = self.reg_sp;
        self.reg_x = self.reg_sp;

        self.update_n_flag(self.reg_sp);
        self.update_z_flag(self.reg_sp);
    }

    // Load mem to Accumulator and X
    pub(super) fn lax(&mut self, _addressmode: AddressingMode) {
        let pos : u16 = self.get_address_from_mode(_addressmode);
        self.reg_a = self.mem_read_u8(pos);
        self.reg_x = self.mem_read_u8(pos);

        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // fn nop(&mut self, _addressmode: AddressingMode) {} // more opcodes match with it

    // Rotate 1 bit let, And and store in accumulator 
    pub(super) fn rla(&mut self, _addressmode: AddressingMode) {
        let pos : u16 = self.get_address_from_mode(_addressmode);
        let new_carry = self.mem_read_u8(pos) & 0b1000_0000 == 0b1000_0000;
        
        let mut tmp_mem = self.mem_read_u8(pos) << 1;
        if self.get_flag(CPUFlag::Carry) {tmp_mem |= 0b0000_0001;}
        self.mem_write_u8(pos, tmp_mem);

        self.reg_a &= tmp_mem;

        self.put_flag(CPUFlag::Carry, new_carry);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Rotate 1 bit right in mem, then add to Accumulator
    pub(super) fn rra(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        let new_carry = self.mem_read_u8(pos) & 0b0000_0001 == 0b0000_0001;
        
        //rotation
        let mut tmp_mem = self.mem_read_u8(pos) >> 1;
        if self.get_flag(CPUFlag::Carry) {
            tmp_mem |= 0b1000_0000;
        }
        self.mem_write_u8(pos, tmp_mem);

        //add to accumulator
        let overflow : bool;
        (self.reg_a, overflow) = self.reg_a.overflowing_add(tmp_mem);

        self.put_flag(CPUFlag::Overflow, overflow);
        self.put_flag(CPUFlag::Carry, new_carry); 
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }
    
    // fn sbc(&mut self, _addressmode: AddressingMode) {} // more opcodes match with it

    // Shift 1 bit left in mem, then OR with Accumulator 
    pub(super) fn slo(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        let new_carry = self.mem_read_u8(pos) & 0b1000_0000 == 0b1000_0000;

        self.mem_write_u8(pos, self.mem_read_u8(pos) << 1);

        self.reg_a |= self.mem_read_u8(pos);

        self.put_flag(CPUFlag::Carry, new_carry);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Shift 1 bit right in mem, then XOR with Accumulator
    pub(super) fn sre(&mut self, _addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(_addressmode);
        let new_carry = self.mem_read_u8(pos) & 0b0000_0001 == 0b0000_0001;

        self.mem_write_u8(pos, self.mem_read_u8(pos) >> 1);

        self.reg_a ^= self.mem_read_u8(pos);

        self.put_flag(CPUFlag::Carry, new_carry);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);       
    }

    // And the high byte of addr with X, add 1 and store in mem
    pub(super) fn sxa(&mut self, _addressmode: AddressingMode) {
       let pos: u16 = self.get_address_from_mode(_addressmode);
       self.mem_write_u8(pos, (self.reg_x & (pos >> 8) as u8) + 1 ); 
    }

    // And the high byte of addr with Y, add 1 and store in mem
    pub(super) fn sya(&mut self, _addressmode: AddressingMode) {
        let pos : u16 = self.get_address_from_mode(_addressmode);
        self.mem_write_u8(pos, (self.reg_y & (pos >> 8) as u8) + 1 );
    }

    // Do nothing
    pub(super) fn top(&mut self, _addressmode: AddressingMode) {}

    // And mem And X And Accumulator, store in Accumulator
    pub(super) fn xaa(&mut self, _addressmode: AddressingMode) {
        let pos : u16 = self.get_address_from_mode(_addressmode);
        self.reg_a &= self.reg_x & self.mem_read_u8(pos);

        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // And X with Accumulator, store in Sp, And result with high byte of addr, store in mem
    pub(super) fn xas(&mut self, _addressmode: AddressingMode) {
        let pos : u16 = self.get_address_from_mode(_addressmode);

        self.reg_sp = self.reg_a & self.reg_x;

        self.mem_write_u8(pos, (self.reg_sp & (pos >> 8) as u8 ) + 1 );

        self.update_n_flag(self.reg_sp);
        self.update_z_flag(self.reg_sp);
    }
 
}