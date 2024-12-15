use crate::ppu::PPU;

use super::frame::Frame;
use super::palette::{self, SYSTEM_PALLETE};

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

pub fn render(ppu: &PPU, frame: &mut Frame) {
    let bank: usize = ppu.reg_control.bg_pattern_addr() as usize;

    // Draw background
    for i in 0..0x03c0 {
        let tile_index: u16 = ppu.vram[i] as u16;
        let tile_x: usize = i % 32;
        let tile_y: usize = i / 32;
    

        let tile: &[u8] = &ppu.chr_rom[(bank + (tile_index*0x10) as usize)..(bank+ ((tile_index+1)*0x10) as usize)];
        let palette: [u8; 4] = bg_palette(ppu, tile_x, tile_y);
        // println!("Tile : {} {:?}", i, tile);    
        for y in 0..=7 {
            let upper: u8 = tile[y];
            let lower: u8 = tile[y+8];

            for x in 0..=7 {
                let value: u8 = ((lower >> (7-x)) & 1) << 1 | ((upper >> (7-x)) & 1);
                let rgb: (u8, u8, u8) = match value {
                    0 => SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                    1 => SYSTEM_PALLETE[palette[1] as usize],
                    2 => SYSTEM_PALLETE[palette[2] as usize],
                    3 => SYSTEM_PALLETE[palette[3] as usize],
                    _ => panic!("This should never happen !"),
                };
                frame.set_pixel(8*tile_x + x, 8*tile_y + y, rgb);
            }
        }
    }

    // Draw sprites
    for i in (0..ppu.oam_data.len()).step_by(4).rev() {
        let tile_index: u16 = ppu.oam_data[i+1] as u16;
        let tile_x: usize = ppu.oam_data[i+3] as usize;
        let tile_y: usize = ppu.oam_data[i] as usize;
        let last_byte: u8 = ppu.oam_data[i+2];


        let flip_vertical: bool = (last_byte >> 7) & 1 == 1;
        let flip_horizontal: bool = (last_byte >> 6) & 1 == 1;

        let palette_index: u8 = last_byte & 0b11;
        let sprite_palette: [u8; 4] = sprite_palette(ppu, palette_index);
        let bank: usize = ppu.reg_control.sprite_pattern_addr() as usize;

        let tile: &[u8] = &ppu.chr_rom[(bank + (tile_index*0x10) as usize)..(bank + ((tile_index+1)*0x10) as usize)];

        for y in 0..=7 {
            let upper: u8 = tile[y];
            let lower: u8 = tile[y+8];

            for x in 0..=7 {
                let value: u8 = ((lower >> (7-x)) & 1) << 1 | ((upper >> (7-x)) & 1);
                let rgb: (u8, u8, u8) = match value {
                    0 => continue,
                    1 => SYSTEM_PALLETE[sprite_palette[1] as usize],
                    2 => SYSTEM_PALLETE[sprite_palette[2] as usize],
                    3 => SYSTEM_PALLETE[sprite_palette[3] as usize],
                    _ => panic!("This should never happen !"),
                };
                match (flip_horizontal, flip_vertical) {
                    (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                    (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                    (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                    (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                }
            }
        }

    }
}

fn bg_palette(ppu: &PPU, tile_col: usize, tile_row: usize) -> [u8; 4] {
    let attr_table_index: usize = (tile_row / 4) * 8 + (tile_col / 4);
    let attr_byte: u8 = ppu.vram[0x03c0 + attr_table_index];
    
    let palette_index: u8 = match ((tile_col % 4) / 2, (tile_row % 4) / 2) {
        (0, 0) => attr_byte & 0b11,
        (1, 0) => (attr_byte >> 2) & 0b11,
        (0, 1) => (attr_byte >> 4) & 0b11,
        (1, 1) => (attr_byte >> 6) & 0b11,
        _ => panic!("This should never happen !")
    };
    let palette_start: usize = 1 + (palette_index as usize)*4;
    [ppu.palette_table[0], ppu.palette_table[palette_start], ppu.palette_table[palette_start+1], ppu.palette_table[palette_start+2]]
}

fn sprite_palette(ppu: &PPU, palette_index: u8) -> [u8; 4] {
    let start: usize = 0x11 + (palette_index * 4) as usize;
    [0, ppu.palette_table[start], ppu.palette_table[start + 1], ppu.palette_table[start + 2],]
}