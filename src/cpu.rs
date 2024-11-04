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


    pub fn interpret(&mut self, program: &Vec<u8>) {
        self.reg_pc = 0;
        loop {
            let opcode: u8 = program[self.reg_pc as usize];
            self.reg_pc += 1;
        
            println!("{:?}", opcode);
            match opcode {
                // LDA 
                // Loads operand into accumulator
                0xa9 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.reg_a = operand;

                    self.update_n_flag(self.reg_a);
                    self.update_z_flag(self.reg_a);
                }

                // AND
                // logical and between a Memory adress and Accumulator
                0x29 => {
                    let operand: u8 = program[self.reg_pc as usize];
                    self.reg_pc += 1;
                    self.reg_a &= operand;

                    self.update_n_flag(self.reg_a);
                    self.update_z_flag(self.reg_a);
                }

                // TAX
                // Transfer Accumulator to X register
                0xAA => {
                    self.reg_pc += 1;
                    self.reg_x = self.reg_a;

                    self.update_n_flag(self.reg_x);
                    self.update_z_flag(self.reg_x);
                }

                // INX
                // Transfer Accumulator to X register
                0xE8 => {
                    self.reg_pc += 1;
                    self.reg_x +=1;

                    self.update_n_flag(self.reg_x);
                    self.update_z_flag(self.reg_x);
                }

                // BRK
                0x00 => {
                    self.status |= 0b0010_0000;
                    return
                }
                // Return 
                // Just for the tests
                0xff => {
                    println!("{:?}", self);
                    return;
                }
                _ => todo!("Opcode not implemented")
            }
        }
    }
    
}