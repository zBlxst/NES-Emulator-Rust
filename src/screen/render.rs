use crate::ppu::PPU;

use super::frame::Frame;
use super::palette::SYSTEM_PALLETE;

pub fn show_tile(chr_rom: &Vec<u8>, bank: usize, tile_n: usize) -> Frame {
    assert!(bank < 2, "There are only 2 banks !");
    let mut frame: Frame = Frame::new();
    let bank: usize = bank * 0x1000;

    let tile: &[u8] = &chr_rom[(bank+tile_n*0x10)..(bank+(tile_n+1)*0x10)];

    for y in 0..=7 {
        let upper: u8 = tile[y];
        let lower: u8 = tile[y+8];

        for x in 0..=7 {
            let value: u8 = ((upper >> x) & 1) << 1 | ((lower >> x) & 1);
            let rgb: (u8, u8, u8) = match value {
                0 => SYSTEM_PALLETE[0x01],
                1 => SYSTEM_PALLETE[0x23],
                2 => SYSTEM_PALLETE[0x27],
                3 => SYSTEM_PALLETE[0x30],
                _ => panic!("This should never happen !"),
            };
            frame.set_pixel(x, y, rgb);
        }
    }

    frame
}

pub fn show_tiles(chr_rom: &Vec<u8>, bank: usize) -> Frame {
    assert!(bank < 2, "There are only 2 banks !");
    let mut frame: Frame = Frame::new();
    let bank: usize = bank * 0x1000;

    for tile_n in 0..=255 {
        let tile_x: usize = 10 * (tile_n % 0x10); 
        let tile_y: usize = 10 * (tile_n / 0x10); 


        let tile: &[u8] = &chr_rom[(bank+ (tile_n*0x10) as usize)..(bank+ ((tile_n+1)*0x10) as usize)];
    
        for y in 0..=7 {
            let upper: u8 = tile[y];
            let lower: u8 = tile[y+8];
    
            for x in 0..=7 {
                let value: u8 = ((upper >> (7-x)) & 1) << 1 | ((lower >> (7-x)) & 1);
                let rgb: (u8, u8, u8) = match value {
                    0 => SYSTEM_PALLETE[0x01],
                    1 => SYSTEM_PALLETE[0x23],
                    2 => SYSTEM_PALLETE[0x27],
                    3 => SYSTEM_PALLETE[0x30],
                    _ => panic!("This should never happen !"),
                };
                frame.set_pixel(tile_x + x, tile_y + y, rgb);
            }
        }

    }

    frame
}
