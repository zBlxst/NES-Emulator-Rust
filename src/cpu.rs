use cast::{u8, u16};

#[derive(Debug)]
pub struct CPU {
    pub reg_pc : u16,
    pub reg_sp : u8,
    pub reg_a  : u8,
    pub reg_x  : u8,
    pub reg_y  : u8,
    pub status : u8,
    pub memory : [u8; 0xffff]
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

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

impl CPU {


    pub fn new() -> Self {
        CPU {
            reg_pc : 0,
            reg_sp : 0,
            reg_a  : 0,
            reg_x  : 0,
            reg_y  : 0,
            status : 0,
            memory : [0; 0xffff]
        }
    }

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

    pub fn get_address_from_mode(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.reg_pc,
            AddressingMode::ZeroPage => self.mem_read_u8(self.reg_pc) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.reg_pc),
            AddressingMode::ZeroPageX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                pos.wrapping_add(self.reg_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                pos.wrapping_add(self.reg_y) as u16
            }
            AddressingMode::AbsoluteX => {
                let pos: u16 = self.mem_read_u16(self.reg_pc);
                pos.wrapping_add(self.reg_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let pos: u16 = self.mem_read_u16(self.reg_pc);
                pos.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::IndirectX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                let addr: u16 = pos.wrapping_add(self.reg_x) as u16;
                self.mem_read_u16(addr)
            }
            AddressingMode::IndirectY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                let addr: u16 = self.mem_read_u16(pos as u16);
                addr.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("Mode : {:?} is not supported", mode);
            }

        }
    }

    fn mem_read_u8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    // Handles little endian
    fn mem_read_u16(&self, addr: u16) -> u16 {
        u16(self.memory[addr.wrapping_add(1) as usize]) << 8 | u16(self.memory[addr as usize])
    }

    fn mem_write_u8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    // Handles little endian
    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = u8(value & 0xff).expect("The logical and of the value and 0xff didn't work for cast (this should never happend)");
        self.memory[addr.wrapping_add(1) as usize] = u8(value >> 8).expect("The logical right shift of the value and 0xff didn't work for cast (this should never happend)");
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.reg_sp = 0;
        self.status = 0;

        self.reg_pc = self.mem_read_u16(0xfffc);
    }

    fn load_program(&mut self, program: &Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xfffc, 0x8000);
    }

    pub fn load_and_run(&mut self, program: &Vec<u8>) {
        self.load_program(program);
        self.reset();
        // println!("{:?}", self);
        self.run();
    }

    fn update_z_flag (&mut self, reg: u8){
        if reg == 0 {
            self.set_flag(CPUFlag::Zero);
        } else {
            self.unset_flag(CPUFlag::Zero);
        }
    }

    fn update_n_flag (&mut self, reg: u8){
        if reg & CPU::mask_from_flag(CPUFlag::Negative) != 0 {
            self.set_flag(CPUFlag::Negative);
        } else {
            self.unset_flag(CPUFlag::Negative);
        }
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

    // Transfer Accumulator to X register
    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Transfer X register to Accumulator
    fn txa(&mut self) {
        self.reg_a = self.reg_x;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Transfer SP register to X register
    fn tsx(&mut self) {
        self.reg_x = self.reg_sp;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Transfer X register to SP register
    fn txs(&mut self) {
        self.reg_sp = self.reg_x;
    }

    // Transfer Accumulator to Y register
    fn tay(&mut self) {
        self.reg_y = self.reg_a;
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Transfer Y register to Accumulator
    fn tya(&mut self) {
        self.reg_a = self.reg_y;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Logical and between a value and Accumulator
    fn and(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a &= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Logical xor between a value and Accumulator
    fn eor(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a ^= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Logical or between a value and Accumulator
    fn ora(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a |= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Increment X register
    fn inx(&mut self) {
        let overflowed : bool;
        (self.reg_x, overflowed) = self.reg_x.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Increment Y register
    fn iny(&mut self) {
        let overflowed : bool;
        (self.reg_y, overflowed) = self.reg_y.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Set carry flag
    fn sec(&mut self) {
        self.set_flag(CPUFlag::Carry);
    }

    // Set decimal flag
    fn sed(&mut self) {
        self.set_flag(CPUFlag::Decimal);
    }

    // Set interruption disable flag
    fn sei(&mut self) {
        self.set_flag(CPUFlag::Interrupt);
    }

    // Jump to a spectified address
    fn jmp(&mut self, operand: u16) {
        self.reg_pc = operand;
    }





    pub fn run(&mut self) {
       loop {
            let opcode: u8 = self.mem_read_u8(self.reg_pc);
            self.reg_pc = self.reg_pc.wrapping_add(1);
        
            println!("{:?}", opcode);
            match opcode {
                // LDA
                0xa9 => {
                    self.lda(AddressingMode::Immediate);
                    self.reg_pc += 1;
                }
                0xa5 => {
                    self.lda(AddressingMode::ZeroPage);
                    self.reg_pc += 1;
                }
                0xb5 => {
                    self.lda(AddressingMode::ZeroPageX);
                    self.reg_pc += 1;
                }
                0xad => {
                    self.lda(AddressingMode::Absolute);
                    self.reg_pc += 2;
                }
                0xbd => {
                    self.lda(AddressingMode::AbsoluteX);
                    self.reg_pc += 2;
                }
                0xb9 => {
                    self.lda(AddressingMode::AbsoluteY);
                    self.reg_pc += 2;
                }
                0xa1 => {
                    self.lda(AddressingMode::IndirectX);
                    self.reg_pc += 1;
                }
                0xb1 => {
                    self.lda(AddressingMode::IndirectY);
                    self.reg_pc += 1;
                }
                
                // LDX
                0xa2 => {
                    self.ldx(AddressingMode::Immediate);
                    self.reg_pc += 1;
                }
                0xa6 => {
                    self.ldx(AddressingMode::ZeroPage);
                    self.reg_pc += 1;
                }
                0xb2 => {
                    self.ldx(AddressingMode::ZeroPageY);
                    self.reg_pc += 1;
                }
                0xae => {
                    self.ldx(AddressingMode::Absolute);
                    self.reg_pc += 2;
                }
                0xbe => {
                    self.ldx(AddressingMode::AbsoluteY);
                    self.reg_pc += 2;
                }

                // LDY
                0xa0 => {
                    self.ldy(AddressingMode::Immediate);
                    self.reg_pc += 1;
                }
                0xa4 => {
                    self.ldy(AddressingMode::ZeroPage);
                    self.reg_pc += 1;
                }
                0xb4 => {
                    self.ldy(AddressingMode::ZeroPageX);
                    self.reg_pc += 1;
                }
                0xac => {
                    self.ldy(AddressingMode::Absolute);
                    self.reg_pc += 2;
                }
                0xbc => {
                    self.ldy(AddressingMode::AbsoluteX);
                    self.reg_pc += 2;
                }

                // ORA
                0x09 => {
                    self.ora(AddressingMode::Immediate);
                    self.reg_pc += 1;
                }
                0x05 => {
                    self.ora(AddressingMode::ZeroPage);
                    self.reg_pc += 1;
                }
                0x15 => {
                    self.ora(AddressingMode::ZeroPageX);
                    self.reg_pc += 1;
                }
                0x0d => {
                    self.ora(AddressingMode::Absolute);
                    self.reg_pc += 2;
                }
                0x1d => {
                    self.ora(AddressingMode::AbsoluteX);
                    self.reg_pc += 2;
                }
                0x19 => {
                    self.ora(AddressingMode::AbsoluteY);
                    self.reg_pc += 2;
                }
                0x01 => {
                    self.ora(AddressingMode::IndirectX);
                    self.reg_pc += 1;
                }
                0x11 => {
                    self.ora(AddressingMode::IndirectY);
                    self.reg_pc += 1;
                }
                
                // AND
                0x29 => {
                    self.and(AddressingMode::Immediate);
                    self.reg_pc += 1;
                }
                0x25 => {
                    self.and(AddressingMode::ZeroPage);
                    self.reg_pc += 1;
                }
                0x35 => {
                    self.and(AddressingMode::ZeroPageX);
                    self.reg_pc += 1;
                }
                0x2d => {
                    self.and(AddressingMode::Absolute);
                    self.reg_pc += 2;
                }
                0x3d => {
                    self.and(AddressingMode::AbsoluteX);
                    self.reg_pc += 2;
                }
                0x39 => {
                    self.and(AddressingMode::AbsoluteY);
                    self.reg_pc += 2;
                }
                0x21 => {
                    self.and(AddressingMode::IndirectX);
                    self.reg_pc += 1;
                }
                0x31 => {
                    self.and(AddressingMode::IndirectY);
                    self.reg_pc += 1;
                }

                // EOR
                0x49 => {
                    self.eor(AddressingMode::Immediate);
                    self.reg_pc += 1;
                }
                0x45 => {
                    self.eor(AddressingMode::ZeroPage);
                    self.reg_pc += 1;
                }
                0x55 => {
                    self.eor(AddressingMode::ZeroPageX);
                    self.reg_pc += 1;
                }
                0x4d => {
                    self.eor(AddressingMode::Absolute);
                    self.reg_pc += 2;
                }
                0x5d => {
                    self.eor(AddressingMode::AbsoluteX);
                    self.reg_pc += 2;
                }
                0x59 => {
                    self.eor(AddressingMode::AbsoluteY);
                    self.reg_pc += 2;
                }
                0x41 => {
                    self.eor(AddressingMode::IndirectX);
                    self.reg_pc += 1;
                }
                0x51 => {
                    self.eor(AddressingMode::IndirectY);
                    self.reg_pc += 1;
                }



                0xaa => self.tax(),
                0x8a => self.txa(),

                0xa8 => self.tay(),
                0x98 => self.tya(),

                0xba => self.tsx(),
                0x9a => self.txs(),

                0xe8 => self.inx(),
                0xc8 => self.iny(),

                0x38 => self.sec(),

                0xf8 => self.sed(),

                0x78 => self.sei(),

                // NOP
                0xea => (),


                0x4c => {
                    let operand: u16 = self.mem_read_u16(self.reg_pc);
                    self.jmp(operand);
                }




                // BRK
                0x00 => {
                    self.status |= CPU::mask_from_flag(CPUFlag::Break);
                    return
                }
                _ => todo!("Opcode not implemented")
            }
        }
    }
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_immediate_lda() {
        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa9, 0xc0, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_a, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa9, 0x12, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_a, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa9, 0x00, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_immediate_ldx() {
        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa2, 0xc0, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_x, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa2, 0x12, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_x, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa2, 0x00, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_immediate_ldy() {
        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa0, 0xc0, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_y, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);


        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa0, 0x12, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_y, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);


        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa0, 0x00, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_y, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_inx() {
        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa2, 0x13, 0xe8, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_x, 0x14);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);


        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa2, 0xff, 0xe8, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_x, 0x00);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);
    }

    #[test]
    fn test_iny() {
        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa0, 0x13, 0xc8, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_y, 0x14);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);


        let mut cpu : CPU = CPU::new();
        let program : Vec<u8> = vec![0xa0, 0xff, 0xc8, 0x00];
        cpu.load_and_run(&program);

        assert_eq!(cpu.reg_y, 0x00);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);
    }


}