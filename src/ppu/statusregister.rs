extern crate bitflags;
use bitflags::bitflags;

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

    pub fn snapshot(&self) -> u8 {
        self.bits
    }

    pub fn is_in_vblank(&self) -> bool {
        self.contains(StatusRegister::VBLANK)
    }

    pub fn reset_vblank_status(&mut self){
        self.set_vblank_status(false);
    }



    
    pub fn set_vblank_status(&mut self, status : bool){
        self.set(StatusRegister::VBLANK, status);
    }

    pub fn set_sprite_zero_hit(&mut self, status : bool){
        self.set(StatusRegister::SPRITE_ZERO_HIT, status);
    }

    pub fn set_sprite_overflow(&mut self, status : bool){
        self.set(StatusRegister::SPRITE_OVERFLOW, status);
    }

}