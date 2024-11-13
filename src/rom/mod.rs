use crate::error::{Error::RomError, Error};

const NES_TAG: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];
const PROG_ROM_PAGE_SIZE: usize = 0x4000; // 16kB
const CHR_ROM_PAGE_SIZE: usize = 0x2000; // 8kB

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
    FOURSCREEN,
}

#[derive(Debug)]
pub struct Rom {
    pub program_rom: [u8; 0x8000],
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(data: &Vec<u8>) -> Result<Self, Error> {
        // Data[0..3] => NES^Z
        // Data[4]    => Program ROM Size
        // Data[5]    => Chr ROM Size
        // Data[6]    => Control Byte
        // Data[7]    => Control Byte
        // Data[8]    => Program RAM Size
        // Data[9]    => TV System (Ignored here)
        // Data[10]    => TV System, Program RAM Presence (Ignored Here)

        // Data[6]
        // 76543210
        // ||||||||
        // |||||||+- Nametable arrangement: 1: vertical arrangement ("horizontal mirrored") (CIRAM A10 = PPU A11)
        // |||||||                          0: horizontal arrangement ("vertically mirrored") (CIRAM A10 = PPU A10)
        // ||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
        // |||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
        // ||||+---- 1: Alternative nametable layout
        // ++++----- Lower part of mapper number

        // Data[7]
        // 76543210
        // ||||||||
        // |||||||+- VS Unisystem
        // ||||||+-- PlayChoice-10 (8 KB of Hint Screen data stored after CHR data)
        // ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
        // ++++----- Upper part of mapper number



        if &data[0..=3] != NES_TAG {
            return Err(RomError(String::from("This is not a iNES file")))
        }

        let mapper: u8 = (data[7] & 0b1111_0000) | (data[6] >> 4);

        let ines_version: u8 = (data[7] >> 2) & 0b11;
        if ines_version != 0b00 {
            return Err(RomError(String::from("This version of NES is not supported!")))
        }

        let four_screen: bool = data[6] & 0b0000_1000 != 0;
        let vertical_mirroring: bool = data[6] & 0b0000_0001 != 0;
        let screen_mirroring: Mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FOURSCREEN,
            (false, true) => Mirroring::VERTICAL,
            (false, false) => Mirroring::HORIZONTAL,
        };

        let program_rom_size: usize = data[4] as usize * PROG_ROM_PAGE_SIZE;
        let chr_rom_size: usize = data[5] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer: bool = data[6] & 0b0000_0100 != 0;

        let program_rom_start: usize = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start: usize = program_rom_start + program_rom_size;

        let mut program_rom: [u8; 0x8000] = [0; 0x8000];
        program_rom[..program_rom_size].copy_from_slice(&data[program_rom_start..(program_rom_start+program_rom_size)]);

        Ok(Rom{
            program_rom: program_rom,
            chr_rom: data[chr_rom_start..(chr_rom_start+chr_rom_size)].to_vec(),
            mapper: mapper,
            screen_mirroring: screen_mirroring
        })
    }

    pub fn new_from_program_rom(data: Vec<u8>) -> Self {
        if data.len() > 0x8000 {
            panic!("The program is to huge to fit in the ROM section");
        }
        let mut program_rom: [u8; 0x8000] = [0; 0x8000];
        program_rom[..data.len()].copy_from_slice(&data[..data.len()]);


        Rom {
            program_rom: program_rom,
            chr_rom: vec![],
            mapper: 0,
            screen_mirroring: Mirroring::FOURSCREEN
        }
    }
}