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

                    if operand == 0 {
                        self.status |= 0b0000_0010;
                    } else {
                        self.status &= !0b0000_0010;
                    }

                    if operand & 0b1000_0000 != 0 {
                        self.status |= 0b1000_0000;
                    } else {
                        self.status &= !0b1000_0000;
                    }
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