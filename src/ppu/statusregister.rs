extern crate bitflags;
use bitflags::bitflags;

// We could use bitflags as well for the status flags of CPU?
bitflags! {

    pub struct StatusRegister: u8 {
        const SPRITE_OVERFLOW      = 0b0010_0000;
        const SPRITE_ZERO_HIT      = 0b0100_0000;
        const VBLANK               = 0b1000_0000;
    }
    /*
        7  bit  0
        ---- ----
        VSOx xxxx
        |||| ||||
        |||+-++++- (PPU open bus or 2C05 PPU identifier), useless
        ||+------- Sprite overflow flag
        |+-------- Sprite 0 hit flag
        +--------- Vblank flag
    */

}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister::from_bits_truncate(0b0000_0000)
    }

    pub fn set_vblank_status(&mut self, status : bool){
        if status { self.bits |= StatusRegister::VBLANK.bits()}
        else { self.bits &= !StatusRegister::VBLANK.bits()}
    }

    pub fn reset_vblank_status(&mut self){
        self.set_vblank_status(false);
    }

}