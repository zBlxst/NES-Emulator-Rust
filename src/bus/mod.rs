use sdl2::libc::MOD_FREQUENCY;

use crate::input::Joypad;
use crate::mem::Mem;
use crate::ppu::PPU;
use crate::rom::Rom;
use crate::screen::Screen;

pub const CPU_RAM_START: u16 = 0x0000;
pub const CPU_RAM_END: u16 = 0x1fff;

pub const PPU_REGISTERS_MIRRORING_START: u16 = 0x2000;  
pub const PPU_REGISTERS_MIRRORING_END: u16 = 0x3fff;

const PPU_CONTROLER_REGISTER: u16 = 0x2000;
const PPU_MASK_REGISTER: u16 = 0x2001;
const PPU_STATUS_REGISTER: u16 = 0x2002;
const PPU_OAM_ADDRESS_REGISTER: u16 = 0x2003;
const PPU_OAM_DATA_REGISTER: u16 = 0x2004;
const PPU_SCROLL_REGISTER: u16 = 0x2005;
const PPU_ADDRESS_REGISTER: u16 = 0x2006;
const PPU_DATA_REGISTER: u16 = 0x2007;
const PPU_OAM_DMA_REGISTER: u16 = 0x4014;

const JOYPAD1_ADDRESS: u16 = 0x4016;
const JOYPAD2_ADDRESS: u16 = 0x4017;

pub const PROGRAM_ROM_START: u16 = 0x8000;
pub const PROGRAM_ROM_END: u16 = 0xffff;

pub const PROGRAM_BASE_POINTER: u16 = 0xfffc;
pub const NMI_ADDRESS_POINTER: u16 = 0xfffa;



// Is it possible to replace some constant 0x by const values?

pub struct Bus {
    cpu_cycles: usize,
    cpu_vram: [u8; 0x800],
    program_rom: [u8; 0x8000],
    pub ppu: PPU,
    pub screen: Screen,
    gameloop_callback: fn(&PPU, &mut Screen)
}

impl Bus {
    pub fn new(rom: Rom, gameloop_callback: fn(&PPU, &mut Screen), joypad1: Joypad, joypad2: Joypad) -> Self {
        Bus {
            cpu_cycles: 0,
            cpu_vram: [0; 0x800],
            program_rom: rom.program_rom,
            ppu: PPU::new(rom.chr_rom, rom.screen_mirroring),
            screen: Screen::new(joypad1, joypad2),
            gameloop_callback: gameloop_callback
        }
    }

    fn read_program_rom(&self, mut addr: u16) -> u8 {
        addr -= PROGRAM_ROM_START;
        if self.program_rom.len() == 0x4000 && addr >= 0x4000 {
            addr %= 0x4000;
        }
        
        self.program_rom[addr as usize]
    }

    pub fn rom_write_program_base(&mut self, program_base: u16) {
        let pos: u16 = PROGRAM_BASE_POINTER - PROGRAM_ROM_START;
        self.program_rom[pos as usize] = (program_base & 0xff) as u8; 
        self.program_rom[(pos+1) as usize] = (program_base >> 8) as u8; 
    }

    pub fn tick(&mut self, op_cycles : usize){
        self.cpu_cycles += op_cycles;
        let nmi_before = self.ppu.nmi_interrupt.is_some();
        self.ppu.tick(op_cycles * 3); // PPU runs 3 times faster than CPU
        let nmi_after = self.ppu.nmi_interrupt.is_some();
        
        if !nmi_before && nmi_after {
            (self.gameloop_callback)(&self.ppu, &mut self.screen);
        } 
    }

    pub fn poll_interrupt_nmi(&mut self) -> Option<()> {
        self.ppu.poll_nmi_interrupt()
    }

    pub fn read_joypad1(&mut self) -> u8 {
        self.screen.joypad1.read()
    }

    pub fn read_joypad2(&mut self) -> u8 {
        self.screen.joypad2.read()
        // 0
    }

    pub fn write_joypad1(&mut self, value: u8) {
        self.screen.joypad1.write(value);
        self.screen.joypad2.write(value);
    }

    pub fn write_joypad2(&mut self, value: u8) {
        // println!("WOOOOOOOOOOOOOOOOOOO");
        self.screen.joypad1.write(value);
        self.screen.joypad2.write(value);
    }

    
}


impl Mem for Bus {
    fn mem_read_u8_no_fail(&mut self, addr: u16, no_fail: bool) -> u8 {
        match addr {
            CPU_RAM_START..=CPU_RAM_END => {// from 0x0000 to 0x1fff
                let real_addr: u16 = addr & 0x7ff;
                self.cpu_vram[real_addr as usize]
            }

            // 0x2000, 0x2001, 0x2003, 0x2005, 0x2006, 0x4014
            PPU_CONTROLER_REGISTER | PPU_MASK_REGISTER | PPU_OAM_ADDRESS_REGISTER | 
            PPU_SCROLL_REGISTER | PPU_ADDRESS_REGISTER | PPU_OAM_DMA_REGISTER => {
                    if no_fail {
                        println!("Trying to read from write-only address {:04x}", addr); 0
                    } else {
                        panic!("Trying to read from write-only address")
                    }
            }

            PPU_STATUS_REGISTER => self.ppu.read_status(), // 0x2002 
            PPU_DATA_REGISTER => self.ppu.read_data(),// 0x2007
            
            PPU_REGISTERS_MIRRORING_START..=PPU_REGISTERS_MIRRORING_END => {// from 0x2000 to 0x3fff
                let mirrored_addr: u16 = addr & 0x2007;
                println!("{:04x}", mirrored_addr);
                self.mem_read_u8(mirrored_addr)
            }

            JOYPAD1_ADDRESS => self.read_joypad1(), // 0x4016
            JOYPAD2_ADDRESS => self.read_joypad2(), // 0x4017

            PROGRAM_ROM_START..=PROGRAM_ROM_END => {// from 0x8000 to 0xffff
                self.read_program_rom(addr)
            }

            _ => {
                // println!("Ignoring access to address {:x}", addr);
                0
            }
        }
    }

    // turn panics into Errors later
    fn mem_write_u8(&mut self, addr: u16, value: u8) {
        match addr {
            CPU_RAM_START..=CPU_RAM_END => {// from 0x0000 to 0x1fff
                let real_addr: u16 = addr & 0x7ff;
                self.cpu_vram[real_addr as usize] = value;
            }

            PPU_CONTROLER_REGISTER => self.ppu.write_to_control(value),// 0x2000
            PPU_MASK_REGISTER => self.ppu.write_to_mask(value), // 0x2001
            PPU_ADDRESS_REGISTER => self.ppu.write_to_ppu_addr(value),// 0x2006
            PPU_DATA_REGISTER => self.ppu.write_to_data(value),// 0x2007

            PPU_OAM_ADDRESS_REGISTER => self.ppu.write_to_oam_addr(value), // 0x2003
            PPU_OAM_DATA_REGISTER => self.ppu.write_to_oam_data(value), // 0x2004

            PPU_SCROLL_REGISTER => {
                self.ppu.write_to_scroll(value); // 0x2005
            }
            
            PPU_STATUS_REGISTER => panic!("Trying to write to PPU Status !"), // 0x2002

            PPU_OAM_DMA_REGISTER => {
                let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (value as u16) << 8;
                for i in 0..256 {
                    buffer[i as usize] = self.mem_read_u8(hi+i);
                }
                self.ppu.write_oam_dma(&buffer);
            }

            PPU_REGISTERS_MIRRORING_START..=PPU_REGISTERS_MIRRORING_END => {// from 0x2000 to 0x3fff
                let mirrored_addr: u16 = addr & 0x2007;
                self.mem_write_u8(mirrored_addr, value);
            }

            JOYPAD1_ADDRESS => self.write_joypad1(value), // 0x4016
            JOYPAD2_ADDRESS => self.write_joypad2(value), // 0x4017

            PROGRAM_ROM_START..=PROGRAM_ROM_END => {// from 0x8000 to 0xffff
                panic!("Attempting to write on program ROM (at {:x})", addr);
            }

            _ => {
                // println!("Ignoring write-access to address {:x}", addr);
            }
        }
    }

}
