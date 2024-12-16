extern crate bitflags;
use bitflags::bitflags;

bitflags! {

    pub struct ControlRegister: u8 {
        const NAMETABLE1              = 0b0000_0001;
        const NAMETABLE2              = 0b0000_0010;
        const INCREMENT_VRAM_ADD      = 0b0000_0100;
        const SPRITE_PATTERN_ADDR     = 0b0000_1000;
        const BACKROUND_PATTERN_ADDR  = 0b0001_0000;
        const HEIGHT_SPRITE           = 0b0010_0000;
        const PPU_MASTER_SLAVE        = 0b0100_0000;
        const VBLANK_NMI_ENABLE       = 0b1000_0000;
    }

    // 7  bit  0
    // ---- ----
    // VPHB SINN
    // |||| ||||
    // |||| ||++- Base nametable address
    // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
    // |||| |     (0: add 1, going across; 1: add 32, going down)
    // |||| +---- Sprite pattern table address for 8x8 sprites
    // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
    // |||+------ Background pattern table address (0: $0000; 1: $1000)
    // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
    // |+-------- PPU master/slave select
    // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
    // +--------- Vblank NMI enable (0: off, 1: on)

    // taken from https://www.nesdev.org/wiki/PPU_registers

}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0b0000_0000)
    }

    pub fn nametable_addr(&self) -> u16 {
        match self.bits & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("Should not happen")
        }
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        if !self.contains(ControlRegister::SPRITE_PATTERN_ADDR) { 0 } else { 0x1000 }
    }

    pub fn bg_pattern_addr(&self) -> u16 {
        if !self.contains(ControlRegister::BACKROUND_PATTERN_ADDR) { 0 } else { 0x1000 }
    }

    pub fn sprite_size(&self) -> u8 {
        if !self.contains(ControlRegister::HEIGHT_SPRITE) { 8 } else { 16 }
    }

    pub fn master_slave_select(&self) -> u8 {
        if !self.contains(ControlRegister::PPU_MASTER_SLAVE) { 0 } else { 1 }
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(ControlRegister::INCREMENT_VRAM_ADD) { 1 } else { 32 }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        self.contains(ControlRegister::VBLANK_NMI_ENABLE)
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }

    
}