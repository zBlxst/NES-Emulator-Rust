use super::CPU;

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
    pub name: &'static str,
    pub instruction : fn(&mut CPU, AddressingMode, u16) -> usize,
    pub address_mode : AddressingMode,
    pub inst_size : usize,
    pub cpu_cycles : usize,
    pub official : bool
}


impl Opcode {

    pub fn exec(self, cpu: &mut CPU) -> usize {
        let new_pc: u16 = cpu.reg_pc.wrapping_add(self.inst_size as u16);
        let additionnal_cycles: usize = (self.instruction)(cpu, self.address_mode, new_pc);
        cpu.reg_pc = cpu.reg_pc.wrapping_add(self.inst_size as u16);
        self.cpu_cycles + additionnal_cycles
    }
}

pub static OPCODES: [Opcode; 256] = {
    let mut instructions  : [Opcode; 256] = [Opcode{name : "WTF", instruction : CPU::no_bind_yet, address_mode : AddressingMode::Implied, inst_size : 0, cpu_cycles : 0, official : false}; 256];

    // ADC
    instructions[0x69] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0x65] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x75] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0x6d] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x7d] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x79] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x61] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x71] = Opcode{name : "ADC", instruction : CPU::adc, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : true};

    // AND
    instructions[0x29] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0x25] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x35] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0x2d] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x3d] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x39] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x21] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x31] = Opcode{name : "AND", instruction : CPU::and, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : true};

    // ASL 
    instructions[0x0a] = Opcode{name : "ASL", instruction : CPU::asl, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2, official : true};
    instructions[0x06] = Opcode{name : "ASL", instruction : CPU::asl, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : true};
    instructions[0x16] = Opcode{name : "ASL", instruction : CPU::asl, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x0e] = Opcode{name : "ASL", instruction : CPU::asl, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : true};
    instructions[0x1e] = Opcode{name : "ASL", instruction : CPU::asl, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : true};

    // BCC 
    instructions[0x90] = Opcode{name : "BCC", instruction : CPU::bcc, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true};

    // BCS
    instructions[0xb0] = Opcode{name : "BCS", instruction : CPU::bcs, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true};

    // BEQ
    instructions[0xf0] = Opcode{name : "BEQ", instruction : CPU::beq, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true};

    // BIT
    instructions[0x24] = Opcode{name : "BIT", instruction : CPU::bit, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x2c] = Opcode{name : "BIT", instruction : CPU::bit, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};

    // BMI 
    instructions[0x30] = Opcode{name : "BMI", instruction : CPU::bmi, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true};

    // BNE
    instructions[0xd0] = Opcode{name : "BNE", instruction : CPU::bne, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true}; 

    // BPL
    instructions[0x10] = Opcode{name : "BPL", instruction : CPU::bpl, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true};

    // BRK
    instructions[0x00] = Opcode{name : "BRK", instruction : CPU::brk, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 7, official : true};

    // BVC
    instructions[0x50] = Opcode{name : "BVC", instruction : CPU::bvc, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true};

    // BVS
    instructions[0x70] = Opcode{name : "BVS", instruction : CPU::bvs, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2, official : true};

    // CLC
    instructions[0x18] = Opcode{name : "CLC", instruction : CPU::clc, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // CLD
    instructions[0xd8] = Opcode{name : "CLD", instruction : CPU::cld, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // CLI
    instructions[0x58] = Opcode{name : "CLI", instruction : CPU::cli, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // CLV
    instructions[0xb8] = Opcode{name : "CLV", instruction : CPU::clv, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // CMP
    instructions[0xc9] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0xc5] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0xd5] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0xcd] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xdd] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xd9] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xc1] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0xd1] = Opcode{name : "CMP", instruction : CPU::cmp, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : true};

    // CPX
    instructions[0xe0] = Opcode{name : "CPX", instruction : CPU::cpx, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0xe4] = Opcode{name : "CPX", instruction : CPU::cpx, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0xec] = Opcode{name : "CPX", instruction : CPU::cpx, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    
    // CPY
    instructions[0xc0] = Opcode{name : "CPY", instruction : CPU::cpy, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0xc4] = Opcode{name : "CPY", instruction : CPU::cpy, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0xcc] = Opcode{name : "CPY", instruction : CPU::cpy, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};

    // DEC
    instructions[0xc6] = Opcode{name : "DEC", instruction : CPU::dec, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : true};
    instructions[0xd6] = Opcode{name : "DEC", instruction : CPU::dec, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0xce] = Opcode{name : "DEC", instruction : CPU::dec, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : true};
    instructions[0xde] = Opcode{name : "DEC", instruction : CPU::dec, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : true};

    // DEX
    instructions[0xca] = Opcode{name : "DEX", instruction : CPU::dex, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // DEY
    instructions[0x88] = Opcode{name : "DEY", instruction : CPU::dey, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // EOR
    instructions[0x49] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0x45] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x55] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0x4d] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x5d] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x59] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x41] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x51] = Opcode{name : "EOR", instruction : CPU::eor, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : true};

    // INC
    instructions[0xe6] = Opcode{name : "INC", instruction : CPU::inc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : true};
    instructions[0xf6] = Opcode{name : "INC", instruction : CPU::inc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0xee] = Opcode{name : "INC", instruction : CPU::inc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : true};
    instructions[0xfe] = Opcode{name : "INC", instruction : CPU::inc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : true};

    // INX
    instructions[0xe8] = Opcode{name : "INX", instruction : CPU::inx, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // INY
    instructions[0xc8] = Opcode{name : "INY", instruction : CPU::iny, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // JMP
    instructions[0x4c] = Opcode{name : "JMP", instruction : CPU::jmp, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 3, official : true};
    instructions[0x6c] = Opcode{name : "JMP", instruction : CPU::jmp, address_mode : AddressingMode::Indirect, inst_size : 3, cpu_cycles : 5, official : true};

    // JSR
    instructions[0x20] = Opcode{name : "JSR", instruction : CPU::jsr, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : true};

    // LDA
    instructions[0xa9] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0xa5] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0xb5] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0xad] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xbd] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xb9] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xa1] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0xb1] = Opcode{name : "LDA", instruction : CPU::lda, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : true};

    // LDX
    instructions[0xa2] = Opcode{name : "LDX", instruction : CPU::ldx, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0xa6] = Opcode{name : "LDX", instruction : CPU::ldx, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0xb6] = Opcode{name : "LDX", instruction : CPU::ldx, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0xae] = Opcode{name : "LDX", instruction : CPU::ldx, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xbe] = Opcode{name : "LDX", instruction : CPU::ldx, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};

    // LDY
    instructions[0xa0] = Opcode{name : "LDY", instruction : CPU::ldy, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0xa4] = Opcode{name : "LDY", instruction : CPU::ldy, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0xb4] = Opcode{name : "LDY", instruction : CPU::ldy, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0xac] = Opcode{name : "LDY", instruction : CPU::ldy, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xbc] = Opcode{name : "LDY", instruction : CPU::ldy, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};

    // LSR
    instructions[0x4a] = Opcode{name : "LSR", instruction : CPU::lsr, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2, official : true};
    instructions[0x46] = Opcode{name : "LSR", instruction : CPU::lsr, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : true};
    instructions[0x56] = Opcode{name : "LSR", instruction : CPU::lsr, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x4e] = Opcode{name : "LSR", instruction : CPU::lsr, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : true};
    instructions[0x5e] = Opcode{name : "LSR", instruction : CPU::lsr, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : true};

    // NOP
    instructions[0xea] = Opcode{name : "NOP", instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // ORA
    instructions[0x09] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0x05] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x15] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0x0d] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x1d] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x19] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x01] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x11] = Opcode{name : "ORA", instruction : CPU::ora, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : true};

    // PHA
    instructions[0x48] = Opcode{name : "PHA", instruction : CPU::pha, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 3, official : true};

    // PHP
    instructions[0x08] = Opcode{name : "PHP", instruction : CPU::php, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 3, official : true};

    // PLA
    instructions[0x68] = Opcode{name : "PLA", instruction : CPU::pla, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 4, official : true};

    // PLP
    instructions[0x28] = Opcode{name : "PLP", instruction : CPU::plp, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 4, official : true};

    // ROL
    instructions[0x2a] = Opcode{name : "ROL", instruction : CPU::rol, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2, official : true}; 
    instructions[0x26] = Opcode{name : "ROL", instruction : CPU::rol, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : true};
    instructions[0x36] = Opcode{name : "ROL", instruction : CPU::rol, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x2e] = Opcode{name : "ROL", instruction : CPU::rol, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : true};
    instructions[0x3e] = Opcode{name : "ROL", instruction : CPU::rol, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : true};

    // ROR
    instructions[0x6a] = Opcode{name : "ROR", instruction : CPU::ror, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2, official : true};
    instructions[0x66] = Opcode{name : "ROR", instruction : CPU::ror, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : true};
    instructions[0x76] = Opcode{name : "ROR", instruction : CPU::ror, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x6e] = Opcode{name : "ROR", instruction : CPU::ror, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : true};
    instructions[0x7e] = Opcode{name : "ROR", instruction : CPU::ror, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : true};

    // RTI
    instructions[0x40] = Opcode{name : "RTI", instruction : CPU::rti, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 6, official : true};

    // RTS
    instructions[0x60] = Opcode{name : "RTS", instruction : CPU::rts, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 6, official : true};

    // SBC
    instructions[0xe9] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : true};
    instructions[0xe5] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0xf5] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0xed] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xfd] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xf9] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0xe1] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0xf1] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : true};

    // SEC
    instructions[0x38] = Opcode{name : "SEC", instruction : CPU::sec, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // SED
    instructions[0xf8] = Opcode{name : "SED", instruction : CPU::sed, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // SEI
    instructions[0x78] = Opcode{name : "SEI", instruction : CPU::sei, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // STA
    instructions[0x85] = Opcode{name : "STA", instruction : CPU::sta, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x95] = Opcode{name : "STA", instruction : CPU::sta, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0x8d] = Opcode{name : "STA", instruction : CPU::sta, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};
    instructions[0x9d] = Opcode{name : "STA", instruction : CPU::sta, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 5, official : true};
    instructions[0x99] = Opcode{name : "STA", instruction : CPU::sta, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5, official : true};
    instructions[0x81] = Opcode{name : "STA", instruction : CPU::sta, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : true};
    instructions[0x91] = Opcode{name : "STA", instruction : CPU::sta, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 6, official : true};

    // STX
    instructions[0x86] = Opcode{name : "STX", instruction : CPU::stx, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x96] = Opcode{name : "STX", instruction : CPU::stx, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0x8e] = Opcode{name : "STX", instruction : CPU::stx, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};

    // STY
    instructions[0x84] = Opcode{name : "STY", instruction : CPU::sty, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : true};
    instructions[0x94] = Opcode{name : "STY", instruction : CPU::sty, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : true};
    instructions[0x8c] = Opcode{name : "STY", instruction : CPU::sty, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : true};

    // TAX
    instructions[0xaa] = Opcode{name : "TAX", instruction : CPU::tax, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // TAY
    instructions[0xa8] = Opcode{name : "TAY", instruction : CPU::tay, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // TSX
    instructions[0xba] = Opcode{name : "TSX", instruction : CPU::tsx, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // TXA
    instructions[0x8a] = Opcode{name : "TXA", instruction : CPU::txa, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // TXS
    instructions[0x9a] = Opcode{name : "TXS", instruction : CPU::txs, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // TYA
    instructions[0x98] = Opcode{name : "TYA", instruction : CPU::tya, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : true};

    // =================================================================================================================
    // ========================================== Unofficial Opcodes ===================================================
    // =================================================================================================================

    // AAC
    instructions[0x0b] = Opcode{name : "AAC", instruction : CPU::aac, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};
    instructions[0x2b] = Opcode{name : "AAC", instruction : CPU::aac, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};

    // AAX
    instructions[0x87] = Opcode{name : "SAX", instruction : CPU::aax, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : false};
    instructions[0x97] = Opcode{name : "SAX", instruction : CPU::aax, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4, official : false};
    instructions[0x83] = Opcode{name : "SAX", instruction : CPU::aax, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0x8f] = Opcode{name : "SAX", instruction : CPU::aax, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : false};

    // ARR
    instructions[0x6b] = Opcode{name : "ARR", instruction : CPU::arr, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};

    // ASR
    instructions[0x4b] = Opcode{name : "ASR", instruction : CPU::asr, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};

    // ATX
    instructions[0xab] = Opcode{name : "ATX", instruction : CPU::atx, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};

    // AXA
    instructions[0x9f] = Opcode{name : "AXA", instruction : CPU::axa, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5, official : false};

    // AXS
    instructions[0xcb] = Opcode{name : "AXS", instruction : CPU::axs, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};

    // DCP
    instructions[0xc7] = Opcode{name : "DCP", instruction : CPU::dcp, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : false};
    instructions[0xd7] = Opcode{name : "DCP", instruction : CPU::dcp, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0xcf] = Opcode{name : "DCP", instruction : CPU::dcp, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : false};
    instructions[0xdf] = Opcode{name : "DCP", instruction : CPU::dcp, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0xdb] = Opcode{name : "DCP", instruction : CPU::dcp, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0xc3] = Opcode{name : "DCP", instruction : CPU::dcp, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8, official : false};
    instructions[0xd3] = Opcode{name : "DCP", instruction : CPU::dcp, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8, official : false};

    // DOP
    instructions[0x04] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : false};
    instructions[0x14] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : false};
    instructions[0x34] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : false};
    instructions[0x44] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : false};
    instructions[0x54] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : false};
    instructions[0x64] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : false};
    instructions[0x74] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : false};
    instructions[0x80] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};
    instructions[0x82] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};
    instructions[0x89] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};
    instructions[0xc2] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};
    instructions[0xd4] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : false};
    instructions[0xe2] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};
    instructions[0xf4] = Opcode{name : "NOP", instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4, official : false};

    // ISC
    instructions[0xe7] = Opcode{name : "ISB", instruction : CPU::isc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : false};
    instructions[0xf7] = Opcode{name : "ISB", instruction : CPU::isc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0xef] = Opcode{name : "ISB", instruction : CPU::isc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : false};
    instructions[0xff] = Opcode{name : "ISB", instruction : CPU::isc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0xfb] = Opcode{name : "ISB", instruction : CPU::isc, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0xe3] = Opcode{name : "ISB", instruction : CPU::isc, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8, official : false};
    instructions[0xf3] = Opcode{name : "ISB", instruction : CPU::isc, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8, official : false};

    // KIL
    instructions[0x02] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x12] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x22] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x32] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x42] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x52] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x62] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x72] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x92] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0xb2] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0xd2] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0xf2] = Opcode{name : "KIL", instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};

    // LAR
    instructions[0xbb] = Opcode{name : "LAR", instruction : CPU::lar, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : false};

    // LAX
    instructions[0xa7] = Opcode{name : "LAX", instruction : CPU::lax, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3, official : false};
    instructions[0xb7] = Opcode{name : "LAX", instruction : CPU::lax, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4, official : false};
    instructions[0xaf] = Opcode{name : "LAX", instruction : CPU::lax, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0xbf] = Opcode{name : "LAX", instruction : CPU::lax, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0xa3] = Opcode{name : "LAX", instruction : CPU::lax, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0xb3] = Opcode{name : "LAX", instruction : CPU::lax, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5, official : false};

    // NOP
    instructions[0x1a] = Opcode{name : "NOP", instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x3a] = Opcode{name : "NOP", instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x5a] = Opcode{name : "NOP", instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0x7a] = Opcode{name : "NOP", instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0xda] = Opcode{name : "NOP", instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};
    instructions[0xfa] = Opcode{name : "NOP", instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2, official : false};

    // RLA
    instructions[0x27] = Opcode{name : "RLA", instruction : CPU::rla, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : false};
    instructions[0x37] = Opcode{name : "RLA", instruction : CPU::rla, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0x2f] = Opcode{name : "RLA", instruction : CPU::rla, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : false};
    instructions[0x3f] = Opcode{name : "RLA", instruction : CPU::rla, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x3b] = Opcode{name : "RLA", instruction : CPU::rla, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x23] = Opcode{name : "RLA", instruction : CPU::rla, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8, official : false};
    instructions[0x33] = Opcode{name : "RLA", instruction : CPU::rla, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8, official : false};

    // RRA
    instructions[0x67] = Opcode{name : "RRA", instruction : CPU::rra, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : false};
    instructions[0x77] = Opcode{name : "RRA", instruction : CPU::rra, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0x6f] = Opcode{name : "RRA", instruction : CPU::rra, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : false};
    instructions[0x7f] = Opcode{name : "RRA", instruction : CPU::rra, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x7b] = Opcode{name : "RRA", instruction : CPU::rra, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x63] = Opcode{name : "RRA", instruction : CPU::rra, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8, official : false};
    instructions[0x73] = Opcode{name : "RRA", instruction : CPU::rra, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8, official : false};

    // SBC
    instructions[0xeb] = Opcode{name : "SBC", instruction : CPU::sbc, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};

    // SLO
    instructions[0x07] = Opcode{name : "SLO", instruction : CPU::slo, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : false};
    instructions[0x17] = Opcode{name : "SLO", instruction : CPU::slo, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0x0f] = Opcode{name : "SLO", instruction : CPU::slo, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : false};
    instructions[0x1f] = Opcode{name : "SLO", instruction : CPU::slo, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x1b] = Opcode{name : "SLO", instruction : CPU::slo, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x03] = Opcode{name : "SLO", instruction : CPU::slo, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8, official : false};
    instructions[0x13] = Opcode{name : "SLO", instruction : CPU::slo, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8, official : false};

    // SRE
    instructions[0x47] = Opcode{name : "SRE", instruction : CPU::sre, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5, official : false};
    instructions[0x57] = Opcode{name : "SRE", instruction : CPU::sre, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6, official : false};
    instructions[0x4f] = Opcode{name : "SRE", instruction : CPU::sre, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6, official : false};
    instructions[0x5f] = Opcode{name : "SRE", instruction : CPU::sre, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x5b] = Opcode{name : "SRE", instruction : CPU::sre, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7, official : false};
    instructions[0x43] = Opcode{name : "SRE", instruction : CPU::sre, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8, official : false};
    instructions[0x53] = Opcode{name : "SRE", instruction : CPU::sre, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8, official : false};

    // SXA
    instructions[0x9e] = Opcode{name : "SXA", instruction : CPU::sxa, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5, official : false};

    // SYA
    instructions[0x9c] = Opcode{name : "SYA", instruction : CPU::sya, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 5, official : false};

    // TOP
    instructions[0x0c] = Opcode{name : "NOP", instruction : CPU::top, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0x1c] = Opcode{name : "NOP", instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0x3c] = Opcode{name : "NOP", instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0x5c] = Opcode{name : "NOP", instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0x7c] = Opcode{name : "NOP", instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0xdc] = Opcode{name : "NOP", instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : false};
    instructions[0xfc] = Opcode{name : "NOP", instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4, official : false};

    // XAA
    instructions[0x8b] = Opcode{name : "XAA", instruction : CPU::xaa, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2, official : false};

    // XAS
    instructions[0x9b] = Opcode{name : "XAS", instruction : CPU::xas, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5, official : false};


    instructions
};