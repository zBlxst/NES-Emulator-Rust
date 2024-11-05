use cast::{u8, u16};

#[derive(Debug)]
pub struct CPU {
    pub reg_pc : u16,
    pub reg_sp : u8,
    pub reg_a  : u8,
    pub reg_x  : u8,
    pub reg_y  : u8,
    pub status : u8,
    pub memory : [u8; 0xffff]
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

#[derive(Copy, Clone, Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,

    Implied,
    Indirect,
    Relative,
    Accumulator,

    NoneAddressing,
}

#[derive(Copy, Clone)]
pub struct Opcode {
    pub instruction : fn(&mut CPU, AddressingMode),
    pub address_mode : AddressingMode,
    pub inst_size : u8
}

impl Opcode{

    pub fn exec(self, cpu: &mut CPU){
        (self.instruction)(cpu, self.address_mode);
        cpu.reg_pc = cpu.reg_pc.wrapping_add(self.inst_size as u16 -1);
    }
}

static OPCODES: [Opcode; 256] ={
    let mut instructions  : [Opcode; 256] = [Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1}; 256];

    //ADC TODO
    instructions[0x69] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x65] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x75] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x6d] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x7d] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x79] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x61] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x71] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::IndirectY, inst_size : 2};

    instructions[0x29] = Opcode{instruction : CPU::and, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x25] = Opcode{instruction : CPU::and, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x35] = Opcode{instruction : CPU::and, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x2d] = Opcode{instruction : CPU::and, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x3d] = Opcode{instruction : CPU::and, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x39] = Opcode{instruction : CPU::and, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x21] = Opcode{instruction : CPU::and, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x31] = Opcode{instruction : CPU::and, address_mode : AddressingMode::IndirectY, inst_size : 2};

    //ASL TODO
    instructions[0x0a] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::Accumulator, inst_size : 1}; //Accumulator
    instructions[0x06] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x16] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x0e] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::Absolute, inst_size : 3};

    //BCC TODO
    instructions[0x90] = Opcode{instruction : CPU::bcc, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //BCS TODO
    instructions[0xb0] = Opcode{instruction : CPU::bcs, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //BEQ TODO
    instructions[0xf0] = Opcode{instruction : CPU::beq, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //BIT TODO
    instructions[0x24] = Opcode{instruction : CPU::bit, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x2c] = Opcode{instruction : CPU::bit, address_mode : AddressingMode::Absolute, inst_size : 3};

    //BMI TODO
    instructions[0x30] = Opcode{instruction : CPU::bmi, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //BNE TODO
    instructions[0xd0] = Opcode{instruction : CPU::bne, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //BPL TODO
    instructions[0x10] = Opcode{instruction : CPU::bpl, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //BRK TODO
    instructions[0x00] = Opcode{instruction : CPU::brk, address_mode : AddressingMode::Implied, inst_size : 1};

    //BVC TODO
    instructions[0x50] = Opcode{instruction : CPU::bvc, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //BVS TODO
    instructions[0x70] = Opcode{instruction : CPU::bvs, address_mode : AddressingMode::Relative, inst_size : 2}; //Relative

    //CLC TODO
    instructions[0x18] = Opcode{instruction : CPU::clc, address_mode : AddressingMode::Implied, inst_size : 1};

    //CLD TODO
    instructions[0xd8] = Opcode{instruction : CPU::cld, address_mode : AddressingMode::Implied, inst_size : 1};

    //CLI TODO
    instructions[0x58] = Opcode{instruction : CPU::cli, address_mode : AddressingMode::Implied, inst_size : 1};

    //CLV TODO
    instructions[0xb8] = Opcode{instruction : CPU::clv, address_mode : AddressingMode::Implied, inst_size : 1};

    //CMP TODO
    instructions[0xc9] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xc5] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xd5] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xcd] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xdd] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0xd9] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0xc1] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0xd1] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::IndirectY, inst_size : 2};

    //CPX TODO
    instructions[0xe0] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xe4] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xec] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::Absolute, inst_size : 3};
    
    //CPY TODO
    instructions[0xc0] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xc4] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xcc] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::Absolute, inst_size : 3};

    //DEC TODO
    instructions[0xc6] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xd6] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xce] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xde] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    //DEX TODO
    instructions[0xca] = Opcode{instruction : CPU::dex, address_mode : AddressingMode::Implied, inst_size : 1};

    //DEY TODO
    instructions[0x88] = Opcode{instruction : CPU::dey, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0x49] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x45] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x55] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x4d] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x5d] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x59] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x41] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x51] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::IndirectY, inst_size : 2};

    //INC TODO
    instructions[0xe6] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xf6] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xee] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xfe] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    instructions[0xe8] = Opcode{instruction : CPU::inx, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0xc8] = Opcode{instruction : CPU::iny, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0x4c] = Opcode{instruction : CPU::jmp, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x6c] = Opcode{instruction : CPU::jmp, address_mode : AddressingMode::Indirect, inst_size : 3};

    //JSR TODO
    instructions[0x20] = Opcode{instruction : CPU::jsr, address_mode : AddressingMode::Absolute, inst_size : 3};

    instructions[0xa9] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xa5] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xb5] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xad] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xbd] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0xb9] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0xa1] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0xb1] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::IndirectY, inst_size : 2};

    instructions[0xa2] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xa6] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xb2] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::ZeroPageY, inst_size : 2};
    instructions[0xae] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xbe] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::AbsoluteY, inst_size : 3};

    instructions[0xa0] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xa4] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xb4] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xac] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xbc] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    //LSR TODO
    instructions[0x4a] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::Accumulator, inst_size : 1};
    instructions[0x46] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x56] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x4e] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x5e] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    //NOP TODO
    instructions[0xea] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0x09] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x05] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x15] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x0d] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x1d] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x19] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x01] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x11] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::IndirectY, inst_size : 2};

    //PHA TODO
    instructions[0x48] = Opcode{instruction : CPU::pha, address_mode : AddressingMode::Implied, inst_size : 1};

    //PHP TODO
    instructions[0x08] = Opcode{instruction : CPU::php, address_mode : AddressingMode::Implied, inst_size : 1};

    //PLA TODO
    instructions[0x68] = Opcode{instruction : CPU::pla, address_mode : AddressingMode::Implied, inst_size : 1};

    //PLP TODO
    instructions[0x28] = Opcode{instruction : CPU::plp, address_mode : AddressingMode::Implied, inst_size : 1};

    //ROL TODO
    instructions[0x2a] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::Accumulator, inst_size : 1}; 
    instructions[0x26] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x36] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x2e] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x3e] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    //ROR TODO
    instructions[0x6a] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::Accumulator, inst_size : 1};
    instructions[0x66] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x76] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x6e] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x7e] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    //RTI TODO
    instructions[0x40] = Opcode{instruction : CPU::rti, address_mode : AddressingMode::Implied, inst_size : 1};

    //RTS TODO
    instructions[0x60] = Opcode{instruction : CPU::rts, address_mode : AddressingMode::Implied, inst_size : 1};

    //SBC TODO
    instructions[0xe9] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xe5] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xf5] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xed] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xfd] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0xf9] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0xe1] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0xf1] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::IndirectY, inst_size : 2};

    instructions[0x38] = Opcode{instruction : CPU::sec, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0xf8] = Opcode{instruction : CPU::sed, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0x78] = Opcode{instruction : CPU::sei, address_mode : AddressingMode::Implied, inst_size : 1};

    //STA TODO
    instructions[0x85] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x95] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x8d] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x9d] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x99] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x81] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x91] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::IndirectY, inst_size : 2};

    //STX TODO
    instructions[0x86] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x96] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::ZeroPageY, inst_size : 2};
    instructions[0x8e] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::Absolute, inst_size : 3};

    //STY TODO
    instructions[0x84] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x94] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x8c] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::Absolute, inst_size : 3};

    instructions[0xaa] = Opcode{instruction : CPU::tax, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0xa8] = Opcode{instruction : CPU::tay, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0xba] = Opcode{instruction : CPU::tsx, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0x8a] = Opcode{instruction : CPU::txa, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0x9a] = Opcode{instruction : CPU::txs, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions[0x98] = Opcode{instruction : CPU::tya, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions
};


impl CPU {

    // ===================================================================
    // ============================= API =================================
    // ===================================================================

    pub fn new() -> Self {
        CPU {
            reg_pc : 0,
            reg_sp : 0,
            reg_a  : 0,
            reg_x  : 0,
            reg_y  : 0,
            status : 0,
            memory : [0; 0xffff]
        }
    }

    fn mem_read_u8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    // Handles little endian
    fn mem_read_u16(&self, addr: u16) -> u16 {
        u16(self.memory[addr.wrapping_add(1) as usize]) << 8 | u16(self.memory[addr as usize])
    }

    fn mem_write_u8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    // Handles little endian
    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = u8(value & 0xff).expect("The logical and of the value and 0xff didn't work for cast (this should never happend)");
        self.memory[addr.wrapping_add(1) as usize] = u8(value >> 8).expect("The logical right shift of the value and 0xff didn't work for cast (this should never happend)");
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.reg_sp = 0;
        self.status = 0;

        self.reg_pc = self.mem_read_u16(0xfffc);
    }

    fn load_program(&mut self, program: &Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xfffc, 0x8000);
    }

    pub fn load_and_run(&mut self, program: &Vec<u8>) {
        self.load_program(program);
        self.reset();
        // println!("{:?}", self);
        self.run();
    }

    pub fn run(&mut self) {
       loop {
            let opcode: u8 = self.mem_read_u8(self.reg_pc);
            self.reg_pc = self.reg_pc.wrapping_add(1);

            println!("opcode :{:?}", opcode);
            OPCODES[opcode as usize].exec(self);
            if self.status & CPU::mask_from_flag(CPUFlag::Break) != 0 {
                break;
            }
        }
    }
    
    // ===================================================================
    // ======================== FLAG MANIPULATION ========================
    // ===================================================================

    pub fn mask_from_flag(flag : CPUFlag) -> u8 {
        match flag {
            CPUFlag::Negative  => 0b1000_0000,
            CPUFlag::Overflow  => 0b0100_0000,
            CPUFlag::Break     => 0b0001_0000,
            CPUFlag::Decimal   => 0b0000_1000,
            CPUFlag::Interrupt => 0b0000_0100,
            CPUFlag::Zero      => 0b0000_0010,
            CPUFlag::Carry     => 0b0000_0001,
        }
    }


    fn update_z_flag (&mut self, reg: u8){
        if reg == 0 {
            self.set_flag(CPUFlag::Zero);
        } else {
            self.unset_flag(CPUFlag::Zero);
        }
    }

    fn update_n_flag (&mut self, reg: u8){
        if reg & CPU::mask_from_flag(CPUFlag::Negative) != 0 {
            self.set_flag(CPUFlag::Negative);
        } else {
            self.unset_flag(CPUFlag::Negative);
        }
    }


    fn put_flag(&mut self, flag: CPUFlag, value: bool) {
        match value {
            true => self.set_flag(flag),
            false => self.unset_flag(flag)
        }
    }

    fn set_flag(&mut self, flag: CPUFlag) {
        self.status |= CPU::mask_from_flag(flag);
    }

    fn unset_flag(&mut self, flag: CPUFlag) {
        self.status &= !CPU::mask_from_flag(flag);
    }

    // ===================================================================
    // ======================== INSTRUCTION SET ==========================
    // ===================================================================

    // Implementation of addressing modes
    fn get_address_from_mode(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.reg_pc,
            AddressingMode::ZeroPage => self.mem_read_u8(self.reg_pc) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.reg_pc),
            AddressingMode::ZeroPageX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                pos.wrapping_add(self.reg_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                pos.wrapping_add(self.reg_y) as u16
            }
            AddressingMode::AbsoluteX => {
                let pos: u16 = self.mem_read_u16(self.reg_pc);
                pos.wrapping_add(self.reg_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let pos: u16 = self.mem_read_u16(self.reg_pc);
                pos.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::IndirectX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                let addr: u16 = pos.wrapping_add(self.reg_x) as u16;
                self.mem_read_u16(addr)
            }
            AddressingMode::IndirectY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc);
                let addr: u16 = self.mem_read_u16(pos as u16);
                addr.wrapping_add(self.reg_y as u16)
            }

            // TODO
            AddressingMode::Implied => {
                panic!("Mode : {:?} is not supported", mode);
            }
            AddressingMode::Indirect => {
                panic!("Mode : {:?} is not supported", mode);
            }
            AddressingMode::Relative => {
                panic!("Mode : {:?} is not supported", mode);
            }
            AddressingMode::Accumulator => {
                panic!("Mode : {:?} is not supported", mode);
            }

            AddressingMode::NoneAddressing => {
                panic!("Mode : {:?} is not supported", mode);
            }

        }
    }
    
    // Add with carry
    fn adc(&mut self, addressmode: AddressingMode) {
       // TODO 
    }

    // Logical and between a value and Accumulator
    fn and(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a &= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Arithmetic shift left
    fn asl(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Branch on carry clear
    fn bcc(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Branch on carry set
    fn bcs(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Branch on equal
    fn beq(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Bit test
    fn bit(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Branch on minus
    fn bmi(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Branch on not equal
    fn bne(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Branch on plus
    fn bpl(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Force break
    fn brk(&mut self, addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Break);
    }

    // Branch on overflow clear
    fn bvc(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Branch on overflow set
    fn bvs(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Clear carry flag
    fn clc(&mut self, addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Carry);
    }

    // Clear decimal mode
    fn cld(&mut self, addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Decimal);
    }

    // Clear interrupt disable
    fn cli(&mut self, addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Interrupt);
    }

    // Clear overflow flag
    fn clv(&mut self, addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Overflow);
    }

    // Compare
    fn cmp(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Compare X register
    fn cpx(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Compare Y register
    fn cpy(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Decrement memory
    fn dec(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Decrement X register
    fn dex(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Decrement Y register
    fn dey(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Exclusive or
    // Logical xor between a value and Accumulator
    fn eor(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a ^= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Increment memory
    fn inc(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Increment X register
    fn inx(&mut self, addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_x, overflowed) = self.reg_x.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Increment Y register
    fn iny(&mut self, addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_y, overflowed) = self.reg_y.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Jump to a spectified address
    fn jmp(&mut self, addressmode: AddressingMode) {// Does not match other functions prototype
        todo!("Implement JMP");
    }

    // Jump to a subroutine
    fn jsr(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Loads operand into Accumulator
    fn lda(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Loads operand into X register
    fn ldx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_x = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Loads operand into Y register
    fn ldy(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_y = self.mem_read_u8(pos);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Logical shift right
    fn lsr(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    fn nop(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Logical or between a value and Accumulator
    fn ora(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a |= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Push Accumulator on stack
    fn pha(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Push Processor status on stack
    fn php(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Pull Accumulator from stack
    fn pla(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Pull Processor status from stack
    fn plp(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Rotate left
    fn rol(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Rotate right
    fn ror(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Return from interrupt
    fn rti(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Return from subroutine
    fn rts(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Subtract with carry
    fn sbc(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Set carry flag
    fn sec(&mut self, addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Carry);
    }

    // Set decimal flag
    fn sed(&mut self, addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Decimal);
    }

    // Set interruption disable flag
    fn sei(&mut self, addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Interrupt);
    }

    // Store Accumulator in memory
    fn sta(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Store X register in memory
    fn stx(&mut self, addressmode: AddressingMode) {
        // TODO
    }

    // Store Y register in memory
    fn sty(&mut self, addressmode: AddressingMode) {
        // TODO
    }    

    // Transfer Accumulator to X register
    fn tax(&mut self, addressmode: AddressingMode) {
        self.reg_x = self.reg_a;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Transfer Accumulator to Y register
    fn tay(&mut self, addressmode: AddressingMode) {
        self.reg_y = self.reg_a;
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Transfer SP register to X register
    fn tsx(&mut self, addressmode: AddressingMode) {
        self.reg_x = self.reg_sp;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }


    // Transfer X register to Accumulator
    fn txa(&mut self, addressmode: AddressingMode) {
        self.reg_a = self.reg_x;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }


    // Transfer X register to SP register
    fn txs(&mut self, addressmode: AddressingMode) {
        self.reg_sp = self.reg_x;
    }



    // Transfer Y register to Accumulator
    fn tya(&mut self, addressmode: AddressingMode) {
        self.reg_a = self.reg_y;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    
    }
     
}







// ==================================================================================================
// ============================================ UNIT TESTS ==========================================
// ==================================================================================================


#[cfg(test)]
mod test {
    use super::*;

    impl CPU {
        pub fn test_prog(program: Vec<u8>) -> Self {
            let mut cpu = CPU::new();
            cpu.load_and_run(&program);
            cpu
        }
    }

    #[test]
    fn test_immediate_lda() {
        let cpu = CPU::test_prog(vec![0xa9, 0xc0, 0x00]);

        assert_eq!(cpu.reg_a, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa9, 0x12, 0x00]);

        assert_eq!(cpu.reg_a, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa9, 0x00, 0x00]);

        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_immediate_ldx() {
        let cpu = CPU::test_prog(vec![0xa2, 0xc0, 0x00]);

        assert_eq!(cpu.reg_x, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa2, 0x12, 0x00]);

        assert_eq!(cpu.reg_x, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0x00]);

        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_immediate_ldy() {
        let cpu = CPU::test_prog(vec![0xa0, 0xc0, 0x00]);

        assert_eq!(cpu.reg_y, 0xc0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa0, 0x12, 0x00]);

        assert_eq!(cpu.reg_y, 0x12);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);

        let cpu = CPU::test_prog(vec![0xa0, 0x00, 0x00]);

        assert_eq!(cpu.reg_y, 0x00);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Negative), 0);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Zero), 0);
    }

    #[test]
    fn test_inx() {
        let cpu = CPU::test_prog(vec![0xa2, 0x13, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0x14);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);

        let cpu = CPU::test_prog(vec![0xa2, 0xff, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0x00);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);
    }

    #[test]
    fn test_iny() {
        let cpu = CPU::test_prog(vec![0xa0, 0x13, 0xc8, 0x00]);

        assert_eq!(cpu.reg_y, 0x14);
        assert_eq!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);

        let cpu = CPU::test_prog(vec![0xa0, 0xff, 0xc8, 0x00]);

        assert_eq!(cpu.reg_y, 0x00);
        assert_ne!(cpu.status & CPU::mask_from_flag(CPUFlag::Overflow), 0);
    }


}