pub mod addressregister;
pub mod controlregister;
pub mod statusregister;
pub mod maskregister;
pub mod scrollregister;

use crate::rom::Mirroring;

use addressregister::AddressingRegister;
use controlregister::ControlRegister;
use maskregister::MaskRegister;
use scrollregister::ScrollRegister;
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
    pub reg_scroll: ScrollRegister,
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
            reg_scroll: ScrollRegister::new(),
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

    pub fn tick(&mut self, ppu_cycles : usize) -> bool {
        self.cycles += ppu_cycles;
        if self.cycles >= SCANLINE_DURATION_IN_PPU_CYCLES {
            if self.is_zero_sprite_hit(self.cycles) {
                self.reg_status.set_sprite_zero_hit(true);
            }
            self.cycles %= SCANLINE_DURATION_IN_PPU_CYCLES;
            self.scanline += 1;
            if self.scanline == SCANLINE_NMI_TRIGGER {
                self.reg_status.set_vblank_status(true);
                self.reg_status.set_sprite_zero_hit(false);
                if self.reg_control.generate_vblank_nmi() {
                    self.nmi_interrupt = Some(());
                }
                // todo!("trigger nmi intterupt")
            }

            if self.scanline >= SCANLINE_MAX {
                self.scanline = 0;
                self.reg_status.reset_vblank_status();
                self.reg_status.set_sprite_zero_hit(false);
                self.nmi_interrupt = None;
                return true;
            }
        }
        return false;
    }

    fn is_zero_sprite_hit(&self, cycle: usize) -> bool {
        let y = self.oam_data[0] as usize;
        let x = self.oam_data[3] as usize;
        (y == self.scanline as usize) && x <= cycle && self.reg_mask.show_sprites()
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
        let addr = self.reg_addr.get();
        match addr {
            0..=0x1fff => {
                println!("attempt to write to chr rom space {}", addr);
                // self.chr_rom[addr as usize] = value;
            }, 
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3eff => {
                println!("addr {} shouldn't be used in reallity", addr);
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }

            // Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
            0x3f00..=0x3fff =>
            {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => println!("unexpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }
    
    
    fn increment_vram_addr(&mut self) {
        self.reg_addr.increment(self.reg_control.vram_addr_increment());
    }

    pub fn write_to_oam_addr(&mut self, value: u8) {
        self.reg_oam_addr = value;
    }
    
    pub fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.reg_oam_addr as usize] = value;
        self.reg_oam_addr = self.reg_oam_addr.wrapping_add(1);
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.reg_oam_addr as usize]
    }

    pub fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.oam_data[self.reg_oam_addr as usize] = *x;
            self.reg_oam_addr = self.reg_oam_addr.wrapping_add(1);
        }
    }

    pub fn write_to_scroll(&mut self, data: u8) {
        self.reg_scroll.write(data);
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
        let res: u8 = self.reg_status.snapshot();
        self.reg_status.reset_vblank_status();
        self.reg_addr.reset_latch();
        res
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_addr: u16 = addr & 0x2fff; // From 0x3000-0x3eff to 0x2000-0x2eff
        let vram_index: u16 = mirrored_addr - VRAM_START;
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<()> {
        self.nmi_interrupt.take()
    }
}