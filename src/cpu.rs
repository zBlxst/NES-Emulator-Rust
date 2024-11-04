#[derive(Debug)]
pub struct CPU {
    pub reg_pc : u16,
    pub reg_sp : u8,
    pub reg_a  : u8,
    pub reg_x  : u8,
    pub reg_y  : u8,
    pub status : u8
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
        }
    }

    fn update_z_flag (&mut self, reg: u8){
        if reg == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= !0b0000_0010;
        }
    }

    fn update_n_flag (&mut self, reg: u8){
        if reg & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= !0b1000_0000;
        }
    }


    // Loads operand into Accumulator
    fn lda(&mut self, operand: u8) {
        self.reg_a = operand;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Loads operand into X register
    fn ldx(&mut self, operand: u8) {
        self.reg_x = operand;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Loads operand into Y register
    fn ldy(&mut self, operand: u8) {
        self.reg_y = operand;
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
    fn and(&mut self, operand: u8) {
        self.reg_a &= operand;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Logical xor between a value and Accumulator
    fn eor(&mut self, operand: u8) {
        self.reg_a ^= operand;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Logical or between a value and Accumulator
    fn ora(&mut self, operand: u8) {
        self.reg_a |= operand;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Increment X register
    fn inx(&mut self) {
        self.reg_x += 1;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Increment Y register
    fn iny(&mut self) {
        self.reg_y += 1;
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Set carry flag
    fn sec(&mut self) {
        self.status |= 0b0000_0001;
    }

    // Set decimal flag
    fn sed(&mut self) {
        self.status |= 0b0000_1000;
    }

    // Set interruption disable flag
    fn sei(&mut self) {
        self.status |= 0b0000_0100;
    }

    // Jump to a spectified address
    fn jmp(&mut self, operand: u16) {
        self.reg_pc = operand;
    }





    pub fn interpret(&mut self, program: &Vec<u8>) {
        self.reg_pc = 0;
        loop {
            let opcode: u8 = program[self.reg_pc as usize];
            self.reg_pc += 1;
        
            println!("{:?}", opcode);
            match opcode {
                0xa9 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.lda(operand);
                }

                0xa2 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.ldx(operand);
                }

                0xa0 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.ldy(operand);
                }

                0x29 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.and(operand);
                }

                0x49 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.eor(operand);
                }

                0x09 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.ora(operand);
                }

                0xAA => self.tax(),
                0x8A => self.txa(),

                0xA8 => self.tay(),
                0x98 => self.tya(),

                0xBA => self.tsx(),
                0x9A => self.txs(),

                0xE8 => self.inx(),
                0xC8 => self.iny(),

                0x38 => self.sec(),

                0xF8 => self.sed(),

                0x78 => self.sei(),

                // NOP
                0xEA => (),


                0x4C => {
                    let low_operand: u16 = program[self.reg_pc as usize].into();
                    self.reg_pc += 1;
                    let high_operand: u16 = program[self.reg_pc as usize].into();
                    self.reg_pc += 1;
                    self.jmp(high_operand << 8 | low_operand);
                }




                // BRK
                0x00 => {
                    self.status |= 0b0010_0000;
                    return
                }
                _ => todo!("Opcode not implemented")
            }
        }
    }
    
}