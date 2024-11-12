use crate::mem::Mem;
use crate::rom::Rom;

pub const RAM_START: u16 = 0x0000;
pub const RAM_END: u16 = 0x1fff;

pub const PPU_REGISTERS_START: u16 = 0x2000;  
pub const PPU_REGISTERS_END: u16 = 0x3fff;

pub const PROGRAM_ROM_START: u16 = 0x8000;
pub const PROGRAM_ROM_END: u16 = 0xffff;

pub const PROGRAM_BASE_POINTER: u16 = 0xfffc;


#[derive(Debug)]
pub struct Bus {
    cpu_vram: [u8; 0x800],
    rom: Rom
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
            PROGRAM_ROM_START..=PROGRAM_ROM_END => {
                self.read_program_rom(addr)
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
            PROGRAM_ROM_START..=PROGRAM_ROM_END => {
                panic!("Attempting to write on program ROM (at {:x})", addr);
            }

            _ => {
                println!("Ignoring write-access to address {:x}", addr);
            }
        }
    }

}

impl Bus {
    pub fn new(rom: Rom) -> Bus {
        Bus {
            cpu_vram: [0; 0x800],
            rom: rom
        }
    }
    
    fn read_program_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.program_rom.len() == 0x4000 && addr >= 0x4000 {
            addr %= 0x4000;
        }
        
        self.rom.program_rom[addr as usize]
    }

    pub fn rom_write_program_base(&mut self, program_base: u16) {
        let pos: u16 = PROGRAM_BASE_POINTER - 0x8000;
        self.rom.program_rom[pos as usize] = (program_base & 0xff) as u8; 
        self.rom.program_rom[(pos+1) as usize] = (program_base >> 8) as u8; 
    }
}