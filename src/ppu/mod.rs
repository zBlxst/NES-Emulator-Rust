pub mod addressregister;
pub mod controlregister;

use crate::rom::Mirroring;

use addressregister::AddressingRegister;
use controlregister::ControlRegister;




const CHR_ROM_START: u16 = 0x0000;
const CHR_ROM_END: u16 = 0x1fff;

const VRAM_START: u16 = 0x2000;
const VRAM_END: u16 = 0x2fff;

const FORBIDDEN_START: u16 = 0x3000;
const FORBIDDEN_END: u16 = 0x3eff;

const PALETTE_START: u16 = 0x3f00;
const PALETTE_END: u16 = 0x3fff;





#[derive(Debug)]
pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],

    pub mirroring: Mirroring,
    pub addr_reg: AddressingRegister,
    pub control_reg: ControlRegister,

    pub internal_buffer: u8,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],

            mirroring,
            addr_reg: AddressingRegister::new(),
            control_reg: ControlRegister::new(),

            internal_buffer: 0,
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr_reg.update(value);
    }

    pub fn write_to_control(&mut self, value: u8) {
        self.control_reg.update(value);
    }

    pub fn write_to_data(&mut self, value: u8) {
        self.vram[self.addr_reg.get() as usize] = value;
    }

    fn increment_vram_addr(&mut self) {
        self.addr_reg.increment(self.control_reg.vram_addr_increment());
    }

    pub fn read_data(&mut self) -> u8 {
        let addr_reg: u16 = self.addr_reg.get();
        self.increment_vram_addr();

        match addr_reg {
            CHR_ROM_START..=CHR_ROM_END => {// from 0x0000 to 0x1fff
                let result: u8 = self.internal_buffer;
                self.internal_buffer = self.chr_rom[(addr_reg - CHR_ROM_START) as usize];
                result 
            }
            VRAM_START..=VRAM_END => {// from 0x2000 to 0x2fff
                let result: u8 = self.internal_buffer;
                self.internal_buffer = self.vram[self.mirror_vram_addr(addr_reg) as usize];
                result
            }
            FORBIDDEN_START..=FORBIDDEN_END => panic!("The address {:04x} is not supposed to be read (between 0x3000 and 0x3eff", addr_reg),// from 0x3000 to 0x3eff
            PALETTE_START..=PALETTE_END => self.palette_table[(addr_reg - PALETTE_START) as usize],// from 0x3f00 to 0x3ff
            _ => panic!("The address {:04x} is not supposed to be read (above 0x4000)", addr_reg)// >= 0x4000
        }
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_addr: u16 = addr & 0b0010_1111_1111_1111; // From 0x3000-0x3eff to 0x2000-0x2eff
        let vram_index: u16 = mirrored_addr - VRAM_START;
        match self.mirroring {
            Mirroring::HORIZONTAL => vram_index & 0b0000_1011_1111_1111,
            Mirroring::VERTICAL => vram_index & 0b0000_0111_1111_1111,
            Mirroring::FOURSCREEN => vram_index
        }
    }
}