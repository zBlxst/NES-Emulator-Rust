use crate::error::Error;

pub mod opcode;
pub mod instruction;

use crate::mem::Mem;
use crate::bus::{Bus, PROGRAM_BASE_POINTER};
use crate::rom::Rom;
use opcode::{OPCODES, AddressingMode};
mod test;


#[derive(Debug)]
pub struct CPU {
    pub reg_pc          : u16,
    pub reg_sp          : u8,
    pub reg_a           : u8,
    pub reg_x           : u8,
    pub reg_y           : u8,
    pub status          : u8,
    pub stack_base      : u16, 
    pub program_base    : u16,
    pub bus             : Bus,
}

#[derive(Debug)]
pub enum CPUFlag {
    Negative,
    Overflow,
    Break,
    Decimal,
    Interrupt,
    Zero,
    Carry,
}

impl Mem for CPU {
    fn mem_read_u8(&self, addr: u16) -> u8 {
        self.bus.mem_read_u8(addr)
    }

    fn mem_write_u8(&mut self, addr: u16, value: u8) {
        self.bus.mem_write_u8(addr, value);
    }
}

impl CPU {

    // ===================================================================
    // ============================= API =================================
    // ===================================================================

    pub fn new(rom: Rom) -> Self {
        CPU {
            reg_pc : 0,
            reg_sp : 0,
            reg_a  : 0,
            reg_x  : 0,
            reg_y  : 0,
            status : 0,
            stack_base : 0x0100,
            program_base : 0x8000,
            bus: Bus::new(rom),
        }
    }

    pub fn set_program_base(&mut self, addr: u16) -> Result<(), Error>{
        (addr < 0x2000 || addr >= 0x8000)
            .then(|| { self.program_base = addr; self.bus.rom_write_program_base(self.program_base); })
            .ok_or_else(|| Error::CpuError(String::from("The start of the program cannot exceed 0x2000")))
         
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.reg_sp = 0xff;
        self.status = 0;

        self.reg_pc = self.mem_read_u16(PROGRAM_BASE_POINTER);
    }

    // We should check the size of the program
    // pub fn load_program(&mut self, program: &Vec<u8>) -> Result<(), Error>{
    //     // if self.program_base as usize + program.len() > 0x2000 {
    //     //     return Err(Error::CpuError(String::from("The program cannot exceed 0x2000 (end of CPU RAM)")));
    //     // }
    //     // self.memory[(self.program_base as usize) .. ((self.program_base as usize) + program.len())].copy_from_slice(&program[..]);
    //     self.mem_write_u16(PROGRAM_BASE_POINTER, self.program_base);
    //     Ok(())
    // }

    // pub fn load_and_run(&mut self, program: &Vec<u8>) -> Result<(), Error> {
    //     self.load_program(program)?;
    //     self.reset();
    //     // println!("{:?}", self);
    //     self.run();
    //     Ok(())
    // }

    pub fn run(&mut self) {
       self.run_with_callback(|_| {});
    }

    pub fn run_debug(&mut self) {
        self.run_with_callback_debug(|_| {});
     }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where F: FnMut(&mut CPU) {
        loop {
            callback(self);

            let opcode: u8 = self.mem_read_u8(self.reg_pc);
            // println!("opcode {:?} at {:x}", opcode, self.reg_pc);
            OPCODES[opcode as usize].exec(self);
            if self.status & CPU::mask_from_flag(CPUFlag::Break) != 0 {
                break;
            }
        }
        println!("Execution is over !\n");
    }

    pub fn run_with_callback_debug<F>(&mut self, mut callback: F)
    where F: FnMut(&mut CPU) {
        loop {
            callback(self);

            let opcode: u8 = self.mem_read_u8(self.reg_pc);
            println!("opcode {:?} at {:x}", opcode, self.reg_pc);
            OPCODES[opcode as usize].exec(self);
            if self.status & CPU::mask_from_flag(CPUFlag::Break) != 0 {
                break;
            }
        }
        println!("Execution is over !\n");
    }
   
     
}