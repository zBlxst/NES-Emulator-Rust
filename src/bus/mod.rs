use crate::mem::Mem;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1fff;

const PPU_REGISTERS_START: u16 = 0x2000;  
const PPU_REGISTERS_END: u16 = 0x3fff;  

#[derive(Debug)]
pub struct Bus {
    cpu_vram: [u8; 0x800]
}

impl Mem for Bus {
    fn mem_read_u8(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                let real_addr: u16 = addr & 0b00000111_11111111;
                self.cpu_vram[real_addr as usize]
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                let _real_addr: u16 = addr & 0b00100000_00000111;
                panic!("Not implemented yet !");
            }
            _ => {
                println!("Ignoring access to address {:x}", addr);
                0
            }
        }
    }

    fn mem_write_u8(&mut self, addr: u16, value: u8) {
        match addr {
            RAM_START..=RAM_END => {
                let real_addr: u16 = addr & 0b00000111_11111111;
                self.cpu_vram[real_addr as usize] = value;
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                let _real_addr: u16 = addr & 0b00100000_00000111;
                panic!("Not implemented yet !");
            }
            _ => {
                println!("Ignoring write-access to address {:x}", addr);
            }
        }
    }
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpu_vram: [0; 0x800]
        }
    }
}