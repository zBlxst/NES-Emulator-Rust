extern crate bitflags;
use bitflags::bitflags;

// We could use bitflags as well for the status flags of CPU?
bitflags! {

    pub struct MaskRegister: u8 {
        const GREYSCALE              = 0b0000_0001;
        const SHOW_BACKGROUND        = 0b0000_0010;
        const SHOW_SPRITES           = 0b0000_0100;
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

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from_bits_truncate(0b0000_0000)
    }

}