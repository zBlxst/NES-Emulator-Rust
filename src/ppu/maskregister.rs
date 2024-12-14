extern crate bitflags;
use bitflags::bitflags;

// We could use bitflags as well for the status flags of CPU?
bitflags! {

    pub struct MaskRegister: u8 {
        const GREYSCALE              = 0b0000_0001;
        const LEFT_PX_BACKGROUND        = 0b0000_0010;
        const LEFT_PX_SPRITES           = 0b0000_0100;
        const BACKGROUND_RENDERING   = 0b0000_1000;
        const SPRITE_RENDERING       = 0b0001_0000;
        const RED                    = 0b0010_0000;
        const GREEN                  = 0b0100_0000;
        const BLUE                   = 0b1000_0000;
    }

    // 7  bit  0
    // ---- ----
    // BGRs bMmG
    // |||| ||||
    // |||| |||+- Greyscale (0: normal color, 1: greyscale)
    // |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
    // |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
    // |||| +---- 1: Enable background rendering
    // |||+------ 1: Enable sprite rendering
    // ||+------- Emphasize red (green on PAL/Dendy)
    // |+-------- Emphasize green (red on PAL/Dendy)
    // +--------- Emphasize blue


    // taken from https://www.nesdev.org/wiki/PPU_registers
    
}

pub enum Color {
    Red,
    Green,
    Blue
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from_bits_truncate(0b0000_0000)
    }

    pub fn is_greyscale(&self) -> bool {
        self.contains(MaskRegister::GREYSCALE)
    }

    pub fn show_leftpixels_bg(&self) -> bool {
        self.contains(MaskRegister::LEFT_PX_BACKGROUND)
    }

    pub fn show_leftpixels_sprite(&self) -> bool {
        self.contains(MaskRegister::LEFT_PX_SPRITES)
    }

    pub fn show_bg(&self) -> bool {
        self.contains(MaskRegister::BACKGROUND_RENDERING)
    }

    pub fn show_sprites(&self) -> bool {
        self.contains(MaskRegister::SPRITE_RENDERING)
    }

    pub fn emphasize(&self) -> Vec<Color> {
        let mut colors = vec![];
        if self.contains(MaskRegister::BLUE) {colors.push(Color::Blue);}
        if self.contains(MaskRegister::RED) {colors.push(Color::Red);}
        if self.contains(MaskRegister::GREEN) {colors.push(Color::Green);}
        colors
    }

    pub fn update(&mut self, data : u8) {
        self.bits = data;
    }

}