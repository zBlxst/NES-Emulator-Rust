use crate::mem::Mem;
use crate::ppu::PPU;
use crate::rom::Rom;

pub const CPU_RAM_START: u16 = 0x0000;
pub const CPU_RAM_END: u16 = 0x1fff;

pub const PPU_REGISTERS_MIRRORING_START: u16 = 0x2000;  
pub const PPU_REGISTERS_MIRRORING_END: u16 = 0x3fff;

const PPU_CONTROLER_REGISTER: u16 = 0x2000;
const PPU_MASK_REGISTER: u16 = 0x2001;
const _PPU_STATUS_REGISTER: u16 = 0x2002;
const PPU_OAM_ADDRESS_REGISTER: u16 = 0x2003;
const _PPU_OAM_DATA_REGISTER: u16 = 0x2004;
const PPU_SCROLL_REGISTER: u16 = 0x2005;
const PPU_ADDRESS_REGISTER: u16 = 0x2006;
const PPU_DATA_REGISTER: u16 = 0x2007;
const PPU_OAM_DMA_REGISTER: u16 = 0x4014;

pub const PROGRAM_ROM_START: u16 = 0x8000;
pub const PROGRAM_ROM_END: u16 = 0xffff;

pub const PROGRAM_BASE_POINTER: u16 = 0xfffc;


// Is it possible to replace some constant 0x by const values?

#[derive(Debug)]
pub struct Bus {
    cpu_vram: [u8; 0x800],
    program_rom: [u8; 0x8000],
    ppu: PPU
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; 0x800],
            program_rom: rom.program_rom,
            ppu: PPU::new(rom.chr_rom, rom.screen_mirroring)
        }
    }

    // is the substraction safe? might need to return err
    fn read_program_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.program_rom.len() == 0x4000 && addr >= 0x4000 {
            addr %= 0x4000;
        }
        
        self.program_rom[addr as usize]
    }

    pub fn rom_write_program_base(&mut self, program_base: u16) {
        let pos: u16 = PROGRAM_BASE_POINTER - 0x8000;
        self.program_rom[pos as usize] = (program_base & 0xff) as u8; 
        self.program_rom[(pos+1) as usize] = (program_base >> 8) as u8; 
    }
}


impl Mem for Bus {
    fn mem_read_u8(&mut self, addr: u16) -> u8 {
        match addr {
            CPU_RAM_START..=CPU_RAM_END => {// from 0x0000 to 0x1fff
                let real_addr: u16 = addr & 0b00000111_11111111;
                self.cpu_vram[real_addr as usize]
            }

            // 0x2000, 0x2001, 0x2002, 0x2003, 0x2005, 0x2006, 0x4014
            PPU_CONTROLER_REGISTER | PPU_MASK_REGISTER | PPU_OAM_ADDRESS_REGISTER | 
            PPU_SCROLL_REGISTER | PPU_ADDRESS_REGISTER | PPU_OAM_DMA_REGISTER
                => panic!("Trying to read from write-only address {:04x}", addr),

            PPU_DATA_REGISTER => self.ppu.read_data(),// 0x2007
            
            PPU_REGISTERS_MIRRORING_START..=PPU_REGISTERS_MIRRORING_END => {// from 0x2000 to 0x3fff
                let mirrored_addr: u16 = addr & 0b00100000_00000111;
                self.mem_read_u8(mirrored_addr)
            }

            PROGRAM_ROM_START..=PROGRAM_ROM_END => {// from 0x8000 to 0xffff
                self.read_program_rom(addr)
            }

            _ => {
                println!("Ignoring access to address {:x}", addr);
                0
            }
        }
    }

    // turn panics into Errors later
    fn mem_write_u8(&mut self, addr: u16, value: u8) {
        match addr {
            CPU_RAM_START..=CPU_RAM_END => {// from 0x0000 to 0x1fff
                let real_addr: u16 = addr & 0x1fff;
                self.cpu_vram[real_addr as usize] = value;
            }

            PPU_CONTROLER_REGISTER => self.ppu.write_to_control(value),// 0x2000
            PPU_ADDRESS_REGISTER => self.ppu.write_to_ppu_addr(value),// 0x2006
            PPU_DATA_REGISTER => self.ppu.write_to_data(value),// 0x2007

            PPU_REGISTERS_MIRRORING_START..=PPU_REGISTERS_MIRRORING_END => {// from 0x2000 to 0x3fff
                let mirrored_addr: u16 = addr & 0x2007;
                self.mem_write_u8(mirrored_addr, value);
            }
            PROGRAM_ROM_START..=PROGRAM_ROM_END => {// from 0x8000 to 0xffff
                panic!("Attempting to write on program ROM (at {:x})", addr);
            }

            _ => {
                println!("Ignoring write-access to address {:x}", addr);
            }
        }
    }

}
