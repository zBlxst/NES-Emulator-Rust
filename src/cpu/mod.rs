pub mod opcode;
pub mod instruction;
mod test;

use std::fmt::format;
use std::path::Path;

use crate::error::Error;
use crate::mem::Mem;
use crate::bus::{Bus, PROGRAM_BASE_POINTER};
use crate::rom::Rom;

use opcode::{AddressingMode, Opcode, OPCODES};

const DEFAULT_STATUS: u8 = 0x24;

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
    pub running         : bool,
}

#[derive(Debug)]
pub enum CPUFlag {
    Negative,
    Overflow,
    Break2,
    Break,
    Decimal,
    Interrupt,
    Zero,
    Carry,
}

impl Mem for CPU {
    fn mem_read_u8(&mut self, addr: u16) -> u8 {
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
            status : DEFAULT_STATUS,
            stack_base : 0x0100,
            program_base : 0x8000,
            bus: Bus::new(rom),
            running: false,
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
        self.reg_sp = 0xfd;
        self.status = DEFAULT_STATUS;
        self.running = true;

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

    pub fn log_args_str(cpu: &mut CPU, opcode: &Opcode, args: u16, addressing_mode: AddressingMode) -> String {
        match addressing_mode {
            AddressingMode::Immediate => format!("#${:02X}", args & 0xff),
            AddressingMode::Absolute => if opcode.name == "JMP" || opcode.name == "JSR" { format!("${:04X}", args) } else { format!("${:04X} = {:02X}", args, cpu.mem_read_u8(args)) },
            AddressingMode::ZeroPage => format!("${:02X} = {:02X}", args & 0xff, cpu.mem_read_u8(args & 0xff)),
            AddressingMode::ZeroPageX => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                format!("${:02X},X @ {:02X} = {:02X}", args & 0xff, addr, cpu.mem_read_u8(addr))
            }
            AddressingMode::ZeroPageY => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                format!("${:02X},Y @ {:02X} = {:02X}", args & 0xff, addr, cpu.mem_read_u8(addr))
            }
            AddressingMode::AbsoluteX => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                format!("${:04X},X @ {:04X} = {:02X}", args, addr, cpu.mem_read_u8(addr))
            } 
            AddressingMode::AbsoluteY => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                format!("${:04X},Y @ {:04X} = {:02X}", args, addr, cpu.mem_read_u8(addr))
            }
            AddressingMode::Indirect => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                if opcode.name == "JMP" || opcode.name == "JSR" {
                    format!("(${:04X}) = {:04X}", args, addr)
                } else {
                    format!("(${:04X}) @ {:04X} = {:02X}", args, addr, cpu.mem_read_u8(addr))
                }
            }
            AddressingMode::IndirectX => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                format!("(${:02X},X) @ {:02X} = {:04X} = {:02X}", args & 0xff, args.wrapping_add(cpu.reg_x as u16) & 0xff, addr, cpu.mem_read_u8(addr))
            }
            AddressingMode::IndirectY => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                format!("(${:02X}),Y = {:04X} @ {:04X} = {:02X}", args & 0xff, addr.wrapping_sub(cpu.reg_y as u16), addr, cpu.mem_read_u8(addr))
            }
            AddressingMode::Relative => {
                let (_, addr): (bool, u16) = cpu.get_address_from_mode(addressing_mode, 0);
                let offset: u8 = cpu.mem_read_u8(addr);
                let value: u16 = if offset < 127 { cpu.reg_pc.wrapping_add(2).wrapping_add(offset as u16) } else { cpu.reg_pc.wrapping_add(2).wrapping_sub(256 - offset as u16) };
                format!("${:04X}", value)
            }
            AddressingMode::Accumulator => format!("A"),
            _ => String::from("")
        }
    }


    pub fn run_with_logs(&mut self, game_path : &str) -> Result<(), Error>{
        let mut logs : String = String::from("");
        let path = Path::new(game_path);
        let folder = path.parent().map(|p| p.to_str()).unwrap().unwrap();
        let file_stem = path.file_stem().map(|s| s.to_str()).unwrap().unwrap();
        let log_path = String::from("./") + folder + "/" + file_stem + ".log";
        let mut all_cycles: usize = 7;

        loop {
            let opcode_num : u8 = self.mem_read_u8(self.reg_pc);
            let opcode : Opcode = OPCODES[opcode_num as usize];
            // ================ Creating logs ==================
            let mut cpu_state : String = format!("{:04X}  {:02X}", self.reg_pc, opcode_num);
            let args: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
            match opcode.inst_size {
                1 => cpu_state.push_str("      "),
                2 => cpu_state.push_str(&format!(" {:02X}   ", (args & 0xff) as u8)),
                3 => cpu_state.push_str(&format!(" {:02X} {:02X}", (args & 0xff) as u8, (args >> 8) as u8 )),
                _ => println!("Should not happen")
            }
            //Instruction in ASM
            cpu_state.push_str(&format!(" {}{} {:27} ", if opcode.official { " " } else { "*" }, opcode.name, CPU::log_args_str(self, &opcode, args, opcode.address_mode)));   
            // Registers state
            cpu_state.push_str(&format!("A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}", self.reg_a, self.reg_x, self.reg_y, self.status,self.reg_sp));
            cpu_state.push_str(&format!(" PPU:{:3},{:3} ", self.bus.ppu.scanline, self.bus.ppu.cycles));
            cpu_state.push_str(&format!("CYC:{}\n", all_cycles));
            // cpu_state.push_str(&format!("*sp:{:02x} [{:02x} {:02x} {:02x} {:02x}]\n", self.reg_sp + 0, self.mem_read_u8(self.stack_base + self.reg_sp as u16), self.mem_read_u8(self.stack_base + self.reg_sp as u16 + 1), self.mem_read_u8(self.stack_base + self.reg_sp as u16 + 2), self.mem_read_u8(self.stack_base + self.reg_sp as u16 + 3)));
            logs.push_str(cpu_state.as_str());
            // print!("{}", cpu_state);

            // =============== Execution ========================
            let cpu_cycles: usize = opcode.exec(self);
            all_cycles += cpu_cycles;
            self.bus.tick(cpu_cycles);
            if !(self.running) {
                break;
            }
        }
        
        std::fs::write(log_path, logs)?;
        println!("Execution is over !\n");
        Ok(())
    }

    pub fn run(&mut self) {
       self.run_with_callback(|_| {}, false);
    }

    pub fn run_debug(&mut self) {
        self.run_with_callback(|_| {}, true);
     }

    pub fn run_with_callback<F>(&mut self, mut callback: F, debug : bool)
    where F: FnMut(&mut CPU) {
        loop {
            callback(self);

            let opcode: Opcode = OPCODES[self.mem_read_u8(self.reg_pc) as usize];
            if debug {
                println!("opcode {:02x} at {:04x}", self.mem_read_u8(self.reg_pc), self.reg_pc);
            }
            let cpu_cycles: usize = opcode.exec(self);
            self.bus.tick(cpu_cycles);
            if !(self.running) {
                break;
            }
        }
        println!("Execution is over !\n");
    }
     
}