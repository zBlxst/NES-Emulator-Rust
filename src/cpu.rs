use cast::u8;

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

impl Opcode {

    pub fn exec(self, cpu: &mut CPU) {
        (self.instruction)(cpu, self.address_mode);
        cpu.reg_pc = cpu.reg_pc.wrapping_add(self.inst_size as u16);
    }
}

static OPCODES: [Opcode; 256] = {
    let mut instructions  : [Opcode; 256] = [Opcode{instruction : CPU::no_bind_yet, address_mode : AddressingMode::Implied, inst_size : 0}; 256];

    // ADC
    instructions[0x69] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x65] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x75] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x6d] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x7d] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x79] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x61] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x71] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // AND
    instructions[0x29] = Opcode{instruction : CPU::and, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x25] = Opcode{instruction : CPU::and, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x35] = Opcode{instruction : CPU::and, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x2d] = Opcode{instruction : CPU::and, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x3d] = Opcode{instruction : CPU::and, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x39] = Opcode{instruction : CPU::and, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x21] = Opcode{instruction : CPU::and, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x31] = Opcode{instruction : CPU::and, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // ASL 
    instructions[0x0a] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::Accumulator, inst_size : 1};
    instructions[0x06] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x16] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x0e] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x1e] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::Absolute, inst_size : 3};

    // BCC 
    instructions[0x90] = Opcode{instruction : CPU::bcc, address_mode : AddressingMode::Relative, inst_size : 2};

    // BCS
    instructions[0xb0] = Opcode{instruction : CPU::bcs, address_mode : AddressingMode::Relative, inst_size : 2};

    // BEQ
    instructions[0xf0] = Opcode{instruction : CPU::beq, address_mode : AddressingMode::Relative, inst_size : 2};

    // BIT
    instructions[0x24] = Opcode{instruction : CPU::bit, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x2c] = Opcode{instruction : CPU::bit, address_mode : AddressingMode::Absolute, inst_size : 3};

    // BMI 
    instructions[0x30] = Opcode{instruction : CPU::bmi, address_mode : AddressingMode::Relative, inst_size : 2};

    // BNE
    instructions[0xd0] = Opcode{instruction : CPU::bne, address_mode : AddressingMode::Relative, inst_size : 2}; 

    // BPL
    instructions[0x10] = Opcode{instruction : CPU::bpl, address_mode : AddressingMode::Relative, inst_size : 2};

    // BRK
    instructions[0x00] = Opcode{instruction : CPU::brk, address_mode : AddressingMode::Implied, inst_size : 1};

    // BVC
    instructions[0x50] = Opcode{instruction : CPU::bvc, address_mode : AddressingMode::Relative, inst_size : 2};

    // BVS
    instructions[0x70] = Opcode{instruction : CPU::bvs, address_mode : AddressingMode::Relative, inst_size : 2};

    // CLC
    instructions[0x18] = Opcode{instruction : CPU::clc, address_mode : AddressingMode::Implied, inst_size : 1};

    // CLD
    instructions[0xd8] = Opcode{instruction : CPU::cld, address_mode : AddressingMode::Implied, inst_size : 1};

    // CLI
    instructions[0x58] = Opcode{instruction : CPU::cli, address_mode : AddressingMode::Implied, inst_size : 1};

    // CLV
    instructions[0xb8] = Opcode{instruction : CPU::clv, address_mode : AddressingMode::Implied, inst_size : 1};

    // CMP
    instructions[0xc9] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xc5] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xd5] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xcd] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xdd] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0xd9] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0xc1] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0xd1] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // CPX
    instructions[0xe0] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xe4] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xec] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::Absolute, inst_size : 3};
    
    // CPY
    instructions[0xc0] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xc4] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xcc] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::Absolute, inst_size : 3};

    // DEC
    instructions[0xc6] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xd6] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xce] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xde] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    // DEX
    instructions[0xca] = Opcode{instruction : CPU::dex, address_mode : AddressingMode::Implied, inst_size : 1};

    // DEY
    instructions[0x88] = Opcode{instruction : CPU::dey, address_mode : AddressingMode::Implied, inst_size : 1};

    // EOR
    instructions[0x49] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x45] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x55] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x4d] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x5d] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x59] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x41] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x51] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // INC
    instructions[0xe6] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xf6] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xee] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xfe] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    // INX
    instructions[0xe8] = Opcode{instruction : CPU::inx, address_mode : AddressingMode::Implied, inst_size : 1};

    // INY
    instructions[0xc8] = Opcode{instruction : CPU::iny, address_mode : AddressingMode::Implied, inst_size : 1};

    // JMP
    instructions[0x4c] = Opcode{instruction : CPU::jmp, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x6c] = Opcode{instruction : CPU::jmp, address_mode : AddressingMode::Indirect, inst_size : 3};

    // JSR
    instructions[0x20] = Opcode{instruction : CPU::jsr, address_mode : AddressingMode::Absolute, inst_size : 3};

    // LDA
    instructions[0xa9] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xa5] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xb5] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xad] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xbd] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0xb9] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0xa1] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0xb1] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // LDX
    instructions[0xa2] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xa6] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xb2] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::ZeroPageY, inst_size : 2};
    instructions[0xae] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xbe] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::AbsoluteY, inst_size : 3};

    // LDY
    instructions[0xa0] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xa4] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xb4] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xac] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xbc] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    // LSR
    instructions[0x4a] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::Accumulator, inst_size : 1};
    instructions[0x46] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x56] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x4e] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x5e] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    // NOP
    instructions[0xea] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1};

    // ORA
    instructions[0x09] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0x05] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x15] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x0d] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x1d] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x19] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x01] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x11] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // PHA
    instructions[0x48] = Opcode{instruction : CPU::pha, address_mode : AddressingMode::Implied, inst_size : 1};

    // PHP
    instructions[0x08] = Opcode{instruction : CPU::php, address_mode : AddressingMode::Implied, inst_size : 1};

    // PLA
    instructions[0x68] = Opcode{instruction : CPU::pla, address_mode : AddressingMode::Implied, inst_size : 1};

    // PLP
    instructions[0x28] = Opcode{instruction : CPU::plp, address_mode : AddressingMode::Implied, inst_size : 1};

    // ROL
    instructions[0x2a] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::Accumulator, inst_size : 1}; 
    instructions[0x26] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x36] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x2e] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x3e] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    // ROR
    instructions[0x6a] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::Accumulator, inst_size : 1};
    instructions[0x66] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x76] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x6e] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x7e] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::AbsoluteX, inst_size : 3};

    // RTI
    instructions[0x40] = Opcode{instruction : CPU::rti, address_mode : AddressingMode::Implied, inst_size : 1};

    // RTS
    instructions[0x60] = Opcode{instruction : CPU::rts, address_mode : AddressingMode::Implied, inst_size : 1};

    // SBC
    instructions[0xe9] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::Immediate, inst_size : 2};
    instructions[0xe5] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0xf5] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0xed] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0xfd] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0xf9] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0xe1] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0xf1] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // SEC
    instructions[0x38] = Opcode{instruction : CPU::sec, address_mode : AddressingMode::Implied, inst_size : 1};

    // SED
    instructions[0xf8] = Opcode{instruction : CPU::sed, address_mode : AddressingMode::Implied, inst_size : 1};

    // SEI
    instructions[0x78] = Opcode{instruction : CPU::sei, address_mode : AddressingMode::Implied, inst_size : 1};

    // STA
    instructions[0x85] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x95] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x8d] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::Absolute, inst_size : 3};
    instructions[0x9d] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::AbsoluteX, inst_size : 3};
    instructions[0x99] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::AbsoluteY, inst_size : 3};
    instructions[0x81] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::IndirectX, inst_size : 2};
    instructions[0x91] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::IndirectY, inst_size : 2};

    // STX
    instructions[0x86] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x96] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::ZeroPageY, inst_size : 2};
    instructions[0x8e] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::Absolute, inst_size : 3};

    // STY
    instructions[0x84] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::ZeroPage, inst_size : 2};
    instructions[0x94] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::ZeroPageX, inst_size : 2};
    instructions[0x8c] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::Absolute, inst_size : 3};

    // TAX
    instructions[0xaa] = Opcode{instruction : CPU::tax, address_mode : AddressingMode::Implied, inst_size : 1};

    // TAY
    instructions[0xa8] = Opcode{instruction : CPU::tay, address_mode : AddressingMode::Implied, inst_size : 1};

    // TSX
    instructions[0xba] = Opcode{instruction : CPU::tsx, address_mode : AddressingMode::Implied, inst_size : 1};

    // TXA
    instructions[0x8a] = Opcode{instruction : CPU::txa, address_mode : AddressingMode::Implied, inst_size : 1};

    // TXS
    instructions[0x9a] = Opcode{instruction : CPU::txs, address_mode : AddressingMode::Implied, inst_size : 1};

    // TYA
    instructions[0x98] = Opcode{instruction : CPU::tya, address_mode : AddressingMode::Implied, inst_size : 1};

    instructions
};


impl CPU {

    // ===================================================================
    // ============================= API =================================
    // ===================================================================

    const STACK_BASE: u16 = 0x0100;

    pub fn new() -> Self {
        CPU {
            reg_pc : 0,
            reg_sp : 0,
            reg_a  : 0,
            reg_x  : 0,
            reg_y  : 0,
            status : 0,
            memory : [0; 0xffff],
        }
    }

    fn mem_read_u8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    // Handles little endian
    fn mem_read_u16(&self, addr: u16) -> u16 {
        (self.memory[addr.wrapping_add(1) as usize] as u16) << 8 | (self.memory[addr as usize] as u16)
    }

    fn mem_write_u8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    // Handles little endian
    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = u8(value & 0xff).expect("The logical and of the value and 0xff didn't work for cast (this should never happend)");
        self.memory[addr.wrapping_add(1) as usize] = u8(value >> 8).expect("The logical right shift of the value and 8 didn't work for cast (this should never happend)");
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
            println!("opcode {:?} at {:x}", opcode, self.reg_pc);
            OPCODES[opcode as usize].exec(self);
            if self.status & CPU::mask_from_flag(CPUFlag::Break) != 0 {
                break;
            }
        }
        println!("Execution is over !\n");
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


    fn update_z_flag (&mut self, value: u8) {
        self.put_flag(CPUFlag::Zero, value == 0);
    }

    fn update_n_flag (&mut self, value: u8) {
        self.put_flag(CPUFlag::Negative, value & CPU::mask_from_flag(CPUFlag::Negative) != 0);
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

    fn get_flag(&self, flag: CPUFlag) -> bool {
        self.status & CPU::mask_from_flag(flag) != 0
    }

    // ===================================================================
    // ======================== INSTRUCTION SET ==========================
    // ===================================================================

    // Implementation of addressing modes
    fn get_address_from_mode(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate | AddressingMode::Absolute | AddressingMode::Relative => self.reg_pc.wrapping_add(1),
            AddressingMode::ZeroPage => self.mem_read_u8(self.reg_pc.wrapping_add(1)) as u16,
            AddressingMode::ZeroPageX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_y) as u16
            }
            AddressingMode::AbsoluteX => {
                let pos: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let pos: u16 = self.mem_read_u16(self.reg_pc.wrapping_add(1));
                pos.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::IndirectX => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                let addr: u16 = pos.wrapping_add(self.reg_x) as u16;
                self.mem_read_u16(addr)
            }
            AddressingMode::IndirectY => {
                let pos: u8 = self.mem_read_u8(self.reg_pc.wrapping_add(1));
                let addr: u16 = self.mem_read_u16(pos as u16);
                addr.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::Indirect => {
                self.mem_read_u16(self.reg_pc.wrapping_add(1))
            }

            AddressingMode::Accumulator | AddressingMode::Implied | AddressingMode::NoneAddressing => {
                panic!("Mode : {:?} is not supported", mode);
            }

        }
    }

    fn jump_rel(&mut self, offset: u8) {
        if offset & CPU::mask_from_flag(CPUFlag::Negative) != 0 {
            self.reg_pc = self.reg_pc.wrapping_sub(256 - offset as u16);
        } else {
            self.reg_pc = self.reg_pc.wrapping_add(offset as u16);
        }
    }

    fn stack_push_u8(&mut self, value: u8) {
        self.mem_write_u8(CPU::STACK_BASE + self.reg_sp as u16, value);
        self.reg_sp = self.reg_sp.wrapping_sub(1);
    }

    fn stack_pop_u8(&mut self) -> u8 {
        self.reg_sp = self.reg_sp.wrapping_sub(1);
        self.mem_read_u8(CPU::STACK_BASE + self.reg_sp as u16)   
    }

    fn stack_push_u16(&mut self, value: u16) {
        self.stack_push_u8(u8(value & 0xff).expect("The logical and of the value and 0xff didn't work for cast (this should never happend)"));
        self.stack_push_u8(u8(value >> 8).expect("The logical right shift of the value and 8 didn't work for cast (this should never happend)"));
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop_u8();
        let high = self.stack_pop_u8();
        (high as u16) << 8 | low as u16
    }
    
    fn no_bind_yet(&mut self, _addressmode: AddressingMode) {
       panic!("This opcode is not binded yet !")
    }

    // Add with carry
    fn adc(&mut self, addressmode: AddressingMode) {
        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {1} else {0} };
        let pos: u16 = self.get_address_from_mode(addressmode);
        let overflowed: bool;
        let overflowed2: bool;

        let base_a: u8 = self.reg_a;
        let to_add: u8 = self.mem_read_u8(pos);

        (self.reg_a, overflowed) = self.reg_a.overflowing_add(to_add);
        (self.reg_a, overflowed2) = self.reg_a.overflowing_add(carry);
        
        self.put_flag(CPUFlag::Carry, overflowed | overflowed2);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);

        // Set overflow if we add two positive (negative) integers which result to a negative (positive) integer
        // First parenthesis (with negation) has MSB set if base_a and to_add have the same MSB
        // Second parenthesis has MSB set if base_a and the result have different MSB 
        self.put_flag(CPUFlag::Overflow, (!(base_a ^ to_add) & (base_a ^ self.reg_a) & 0b1000_0000) != 0);
        
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
        let overflowing: bool;
        let res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                (res, overflowing) = self.reg_a.overflowing_mul(2);
                self.reg_a = res;
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                (res, overflowing) = self.mem_read_u8(pos).overflowing_mul(2);
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, overflowing);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    // Branch on carry clear
    fn bcc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Carry) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on carry set
    fn bcs(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Carry) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on equal
    fn beq(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Zero) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Bit test
    fn bit(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Zero, value & self.reg_a == 0);
        self.put_flag(CPUFlag::Overflow, value & CPU::mask_from_flag(CPUFlag::Overflow) == 1);
        self.put_flag(CPUFlag::Negative, value & CPU::mask_from_flag(CPUFlag::Negative) == 1);
    }

    // Branch on minus
    fn bmi(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Negative) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on not equal
    fn bne(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Zero) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on plus
    fn bpl(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Negative) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Force break
    fn brk(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Break);
    }

    // Branch on overflow clear
    fn bvc(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if !self.get_flag(CPUFlag::Overflow) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Branch on overflow set
    fn bvs(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        if self.get_flag(CPUFlag::Overflow) {
            self.jump_rel(self.mem_read_u8(pos));
        }
    }

    // Clear carry flag
    fn clc(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Carry);
    }

    // Clear decimal mode
    fn cld(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Decimal);
    }

    // Clear interrupt disable
    fn cli(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Interrupt);
    }

    // Clear overflow flag
    fn clv(&mut self, _addressmode: AddressingMode) {
        self.unset_flag(CPUFlag::Overflow);
    }

    // Compare
    fn cmp(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_a >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_a == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_a <= to_compare);
    }

    // Compare X register
    fn cpx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_x >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_x == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_x <= to_compare);
    }

    // Compare Y register
    fn cpy(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let to_compare: u8 = self.mem_read_u8(pos);
        self.put_flag(CPUFlag::Carry, self.reg_y >= to_compare);
        self.put_flag(CPUFlag::Zero, self.reg_y == to_compare);
        self.put_flag(CPUFlag::Negative, self.reg_y <= to_compare);
    }

    // Decrement memory
    fn dec(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos).wrapping_sub(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
    }

    // Decrement X register
    fn dex(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_x.wrapping_sub(1);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);

    }

    // Decrement Y register
    fn dey(&mut self, _addressmode: AddressingMode) {
        self.reg_y = self.reg_y.wrapping_sub(1);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
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
        let pos: u16 = self.get_address_from_mode(addressmode);
        let value: u8 = self.mem_read_u8(pos).wrapping_add(1);
        self.mem_write_u8(pos, value);
        self.update_n_flag(value);
        self.update_z_flag(value);
    }

    // Increment X register
    fn inx(&mut self, _addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_x, overflowed) = self.reg_x.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Increment Y register
    fn iny(&mut self, _addressmode: AddressingMode) {
        let overflowed : bool;
        (self.reg_y, overflowed) = self.reg_y.overflowing_add(1);

        self.put_flag(CPUFlag::Overflow, overflowed);
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Jump to a spectified address
    fn jmp(&mut self, addressmode: AddressingMode) {// Does not match other functions prototype
        let pos: u16 = self.get_address_from_mode(addressmode);
        println!("Pos : {:x}", pos);
        // Substracts 3 to balance the +3 after the instruction
        // We still have to check if the compiler/assembler doesn't already handles it
        self.reg_pc = self.mem_read_u16(pos).wrapping_sub(3);
    }

    // Jump to a subroutine
    fn jsr(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);

        // We add two to handle the 3-bit sized instruction 
        self.stack_push_u16(self.reg_pc + 3);

        // Substracts 3 to balance the +3 after the instruction
        // We still have to check if the compiler/assembler doesn't already handles it
        self.reg_pc = self.mem_read_u16(pos).wrapping_sub(3);
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
        let old_value: u8;
        let res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                old_value = self.reg_a;
                res = old_value / 2;
                self.reg_a = res;
                
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                old_value = self.mem_read_u8(pos);
                res = old_value / 2;
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, old_value & 0b0000_0001 != 0);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    fn nop(&mut self, _addressmode: AddressingMode) {}

    // Logical or between a value and Accumulator
    fn ora(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.reg_a |= self.mem_read_u8(pos);
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }

    // Push Accumulator on stack
    fn pha(&mut self, _addressmode: AddressingMode) {
        self.stack_push_u8(self.reg_a);
    }

    // Push Processor status on stack
    fn php(&mut self, _addressmode: AddressingMode) {
        self.stack_push_u8(self.status);
    }

    // Pull Accumulator from stack
    fn pla(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.stack_pop_u8();
    }

    // Pull Processor status from stack
    fn plp(&mut self, _addressmode: AddressingMode) {
        self.status = self.stack_pop_u8();
    }

    // Rotate left
    fn rol(&mut self, addressmode: AddressingMode) {
        let overflowing: bool;
        let mut res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                (res, overflowing) = self.reg_a.overflowing_mul(2);
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b0000_0001;
                }
                self.reg_a = res;
                
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                (res, overflowing) = self.mem_read_u8(pos).overflowing_mul(2);
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b0000_0001;
                }
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, overflowing);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    // Rotate right
    fn ror(&mut self, addressmode: AddressingMode) {
        let old_value: u8;
        let mut res: u8;
        match addressmode {
            AddressingMode::Accumulator => {
                old_value = self.reg_a;
                res = old_value / 2;
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b1000_0000;
                }
                self.reg_a = res;
                
            }
            _ => {
                let pos: u16 = self.get_address_from_mode(addressmode);
                old_value = self.mem_read_u8(pos);
                res = old_value / 2;
                if self.get_flag(CPUFlag::Carry) {
                    res |= 0b1000_0000;
                }
                self.mem_write_u8(pos, res);
            }
        }
        self.put_flag(CPUFlag::Carry, old_value & 0b0000_0001 != 0);
        self.update_n_flag(res);

        // On nesdev it says if A = 0 but on doc it says if res = 0 
        self.update_z_flag(res);
    }

    // Return from interrupt
    fn rti(&mut self, _addressmode: AddressingMode) {
        self.status = self.stack_pop_u8();

        // Substracts 1 to balance the +1 after the instruction
        // We still have to check if the compiler/assembler doesn't already handles it
        self.reg_pc = self.stack_pop_u16().wrapping_sub(1);
    }

    // Return from subroutine
    fn rts(&mut self, _addressmode: AddressingMode) {

        // Substracts 1 to balance the +1 after the instruction
        // We still have to check if the compiler/assembler doesn't already handles it
        self.reg_pc = self.stack_pop_u16().wrapping_sub(1);
    }

    // Subtract with carry
    fn sbc(&mut self, addressmode: AddressingMode) {
        let carry: u8 = { if self.get_flag(CPUFlag::Carry) {0} else {1} };
        let pos: u16 = self.get_address_from_mode(addressmode);
        let overflowed: bool;
        let overflowed2: bool;

        let base_a: u8 = self.reg_a;
        let to_sub: u8 = self.mem_read_u8(pos);

        (self.reg_a, overflowed) = self.reg_a.overflowing_sub(to_sub);
        (self.reg_a, overflowed2) = self.reg_a.overflowing_sub(carry);
        
        self.put_flag(CPUFlag::Carry, !(overflowed | overflowed2));
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);

        // Set overflow if we add two positive (negative) integers which result to a negative (positive) integer
        // First parenthesis has MSB set if base_a and to_add have the different MSB (+/- or -/+)
        // Second parenthesis has MSB set if base_a and the result have different MSB (+/- or -/+)
        // They are both set if we substract a negative to a positive and result is negative (or the contrary)
        self.put_flag(CPUFlag::Overflow, ((base_a ^ to_sub) & (base_a ^ self.reg_a) & 0b1000_0000) != 0);
    }

    // Set carry flag
    fn sec(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Carry);
    }

    // Set decimal flag
    fn sed(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Decimal);
    }

    // Set interruption disable flag
    fn sei(&mut self, _addressmode: AddressingMode) {
        self.set_flag(CPUFlag::Interrupt);
    }

    // Store Accumulator in memory
    fn sta(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_a);
    }

    // Store X register in memory
    fn stx(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_x);
    }

    // Store Y register in memory
    fn sty(&mut self, addressmode: AddressingMode) {
        let pos: u16 = self.get_address_from_mode(addressmode);
        self.mem_write_u8(pos, self.reg_y);
    }    

    // Transfer Accumulator to X register
    fn tax(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_a;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }

    // Transfer Accumulator to Y register
    fn tay(&mut self, _addressmode: AddressingMode) {
        self.reg_y = self.reg_a;
        self.update_n_flag(self.reg_y);
        self.update_z_flag(self.reg_y);
    }

    // Transfer SP register to X register
    fn tsx(&mut self, _addressmode: AddressingMode) {
        self.reg_x = self.reg_sp;
        self.update_n_flag(self.reg_x);
        self.update_z_flag(self.reg_x);
    }


    // Transfer X register to Accumulator
    fn txa(&mut self, _addressmode: AddressingMode) {
        self.reg_a = self.reg_x;
        self.update_n_flag(self.reg_a);
        self.update_z_flag(self.reg_a);
    }


    // Transfer X register to SP register
    fn txs(&mut self, _addressmode: AddressingMode) {
        self.reg_sp = self.reg_x;
    }



    // Transfer Y register to Accumulator
    fn tya(&mut self, _addressmode: AddressingMode) {
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
    use std::vec;

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

    #[test]
    fn test_jmp_and_branches() {
        // Jump
        let cpu = CPU::test_prog(vec![0xe8, 0x4c, 0x06, 0x80, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x03);
        assert_eq!(cpu.reg_pc, 0x8009);

        let cpu = CPU::test_prog(vec![0xe8, 0x30, 0x03, 0x6c, 0xfc, 0xff, 0x00]);
        assert_eq!(cpu.reg_x, 0x80);
        assert_eq!(cpu.reg_pc, 0x8007);
        
        // Carry set
        let cpu = CPU::test_prog(vec![0x38, 0xb0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8008);

        let cpu = CPU::test_prog(vec![0xea, 0xb0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8005);

        // Carry clear
        let cpu = CPU::test_prog(vec![0x38, 0x90, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8005);

        let cpu = CPU::test_prog(vec![0xea, 0x90, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8008);

        // Negative set
        let cpu = CPU::test_prog(vec![0xa2, 0xff, 0x30, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8009);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0x30, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8006);

        // Negative clear
        let cpu = CPU::test_prog(vec![0xa2, 0xff, 0x10, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.reg_pc, 0x8006);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0x10, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8009);

        // Zero set
        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0xf0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8009);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xf0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8006);

        // Zero clear
        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0xd0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8006);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xd0, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x8009);

        // Overflow set
        let cpu = CPU::test_prog(vec![0xa9, 0x7f, 0x69, 0x7f, 0x70, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x800b);

        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xea, 0xea, 0x70, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8008);

        // Overflow clear
        let cpu = CPU::test_prog(vec![0xa9, 0x7f, 0x69, 0x7f, 0x50, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x01);
        assert_eq!(cpu.reg_pc, 0x8008);
        
        let cpu = CPU::test_prog(vec![0xea, 0xea, 0xea, 0xea, 0x50, 0x02, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x02);
        assert_eq!(cpu.reg_pc, 0x800b);

        // Negative branching
        let cpu = CPU::test_prog(vec![0xa2, 0x00, 0xf0, 0xfd, 0xe8, 0x00, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.reg_pc, 0x8002);

    }

    #[test]
    fn test_shifts() {
        // ASL
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x0a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0101_0000);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b1110_1000, 0x0a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1101_0000);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        // LSR
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x4a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1001, 0x4a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        // ROL
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0101_0000);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b1110_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1101_0000);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b0010_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0101_0001);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b1110_1000, 0x2a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1101_0001);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        // ROR
        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1000, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0b0010_1001, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b0001_0100);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(!cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b0010_1000, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1001_0100);
        assert!(!cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0b0010_1001, 0x6a, 0x00]);
        assert_eq!(cpu.reg_a, 0b1001_0100);
        assert!(cpu.get_flag(CPUFlag::Carry));
        assert!(!cpu.get_flag(CPUFlag::Zero));
        assert!(cpu.get_flag(CPUFlag::Negative));

    }

    #[test]
    fn test_adc_sbc() {
        // ADC
        let cpu = CPU::test_prog(vec![0xa9, 0x18, 0x69, 0x12, 0x00]);
        assert_eq!(cpu.reg_a, 0x2a);
        assert!(!cpu.get_flag(CPUFlag::Overflow));

        let cpu = CPU::test_prog(vec![0xa9, 0x81, 0x69, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0x49);
        assert!(cpu.get_flag(CPUFlag::Overflow));

        let cpu = CPU::test_prog(vec![0xa9, 0x51, 0x69, 0x7f, 0x00]);
        assert_eq!(cpu.reg_a, 0xd0);
        assert!(cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0x81, 0x69, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0x4a);
        assert!(cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Carry));

        // SBC
        let cpu = CPU::test_prog(vec![0xa9, 0x18, 0xe9, 0x12, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert!(!cpu.get_flag(CPUFlag::Overflow));

        let cpu = CPU::test_prog(vec![0xa9, 0x81, 0xe9, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0xb8);
        assert!(!cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0xa9, 0x51, 0xe9, 0x80, 0x00]);
        assert_eq!(cpu.reg_a, 0xd0);
        assert!(cpu.get_flag(CPUFlag::Overflow));
        assert!(cpu.get_flag(CPUFlag::Negative));

        let cpu = CPU::test_prog(vec![0x38, 0xa9, 0x81, 0xe9, 0xc8, 0x00]);
        assert_eq!(cpu.reg_a, 0xb9);
        assert!(!cpu.get_flag(CPUFlag::Overflow));
        assert!(!cpu.get_flag(CPUFlag::Carry));

    }
    

}