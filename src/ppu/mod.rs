pub mod addressregister;
pub mod controlregister;
pub mod statusregister;
pub mod maskregister;

use crate::rom::Mirroring;

use addressregister::AddressingRegister;
use controlregister::ControlRegister;
use maskregister::MaskRegister;
use statusregister::StatusRegister;


const CHR_ROM_START: u16 = 0x0000;
const CHR_ROM_END: u16 = 0x1fff;

const VRAM_START: u16 = 0x2000;
const VRAM_END: u16 = 0x2fff;

const FORBIDDEN_START: u16 = 0x3000;
const FORBIDDEN_END: u16 = 0x3eff;

const PALETTE_START: u16 = 0x3f00;
const PALETTE_END: u16 = 0x3fff;


const SCANLINE_NMI_TRIGGER : usize = 241;
const SCANLINE_MAX : usize = 262;
const SCANLINE_DURATION_IN_PPU_CYCLES : usize = 341;



#[derive(Debug)]
pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],

    pub mirroring: Mirroring,
    // writes to registers $2000, $2001, $2005 and $2006 are ignored before the 1st pre-render scanline
    pub reg_addr: AddressingRegister,
    pub reg_control: ControlRegister,
    pub reg_mask : MaskRegister,
    pub reg_oam_addr: u8,
    pub reg_oam_data: u8,
    pub reg_status: StatusRegister,
    pub nmi_interrupt: Option<()>,

    //internal registers
    pub reg_v : u16, // 15 bits
    pub reg_t : u16, // 15 bits
    pub reg_x : u8, // 3 bits
    pub reg_w : bool, // 1 bit

    pub internal_buffer: u8,
    pub cycles: usize,
    pub scanline: usize,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],

            mirroring,
            reg_addr: AddressingRegister::new(),
            reg_control: ControlRegister::new(),
            reg_mask : MaskRegister::new(),
            reg_oam_addr: 0,
            reg_oam_data: 0,
            reg_status: StatusRegister::new(),
            nmi_interrupt: None,


            reg_v : 0,
            reg_t : 0,
            reg_x : 0,
            reg_w : false,

            internal_buffer: 0,
            cycles: 21,
            scanline: 0,
        }
    }

    pub fn tick(&mut self, ppu_cycles : usize) {
        self.cycles += ppu_cycles;
        if self.cycles >= SCANLINE_DURATION_IN_PPU_CYCLES {
            self.cycles %= SCANLINE_DURATION_IN_PPU_CYCLES;
            self.scanline += 1;

            if self.scanline == SCANLINE_NMI_TRIGGER && self.reg_control.generate_vblank_nmi() {
                self.reg_status.set_vblank_status(true);
                self.nmi_interrupt = Some(());
                // todo!("trigger nmi intterupt")
            }

            if self.scanline >= SCANLINE_MAX{
                self.scanline = 0;
                self.reg_status.reset_vblank_status();
            }
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.reg_addr.update(value);
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.reg_mask.update(value);
    }

    pub fn write_to_control(&mut self, value: u8) {
        let before_nmi_status: bool = self.reg_control.generate_vblank_nmi(); 
        self.reg_control.update(value);
        let after_nmi_status: bool = self.reg_control.generate_vblank_nmi(); 
        if !before_nmi_status && after_nmi_status && self.reg_status.is_in_vblank() {
            self.nmi_interrupt = Some(());
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        self.vram[self.reg_addr.get() as usize] = value;
    }

    pub fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.reg_oam_addr as usize] = value;
        self.reg_oam_addr = self.reg_oam_addr.wrapping_add(1);
    }

    fn increment_vram_addr(&mut self) {
        self.reg_addr.increment(self.reg_control.vram_addr_increment());
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.reg_oam_addr as usize]
    }

    pub fn read_data(&mut self) -> u8 {
        let reg_addr: u16 = self.reg_addr.get();
        self.increment_vram_addr();

        match reg_addr {
            CHR_ROM_START..=CHR_ROM_END => {// from 0x0000 to 0x1fff
                let result: u8 = self.internal_buffer;
                self.internal_buffer = self.chr_rom[(reg_addr - CHR_ROM_START) as usize];
                result 
            }
            VRAM_START..=VRAM_END => {// from 0x2000 to 0x2fff
                let result: u8 = self.internal_buffer;
                self.internal_buffer = self.vram[self.mirror_vram_addr(reg_addr) as usize];
                result
            }
            FORBIDDEN_START..=FORBIDDEN_END => panic!("The address {:04x} is not supposed to be read (between 0x3000 and 0x3eff", reg_addr),// from 0x3000 to 0x3eff
            PALETTE_START..=PALETTE_END => self.palette_table[(reg_addr - PALETTE_START) as usize],// from 0x3f00 to 0x3ff
            _ => panic!("The address {:04x} is not supposed to be read (above 0x4000)", reg_addr)// >= 0x4000
        }
    }

    pub fn read_status(&mut self) -> u8 {
        self.reg_status.bits()
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_addr: u16 = addr & 0x2fff; // From 0x3000-0x3eff to 0x2000-0x2eff
        let vram_index: u16 = mirrored_addr - VRAM_START;
        match self.mirroring {
            Mirroring::HORIZONTAL => vram_index & 0x0bff,
            Mirroring::VERTICAL => vram_index & 0x07ff,
            Mirroring::FOURSCREEN => vram_index
        }
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<()> {
        self.nmi_interrupt.take()
    }
}