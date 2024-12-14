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
    pub instruction : fn(&mut CPU, AddressingMode, u16) -> usize,
    pub address_mode : AddressingMode,
    pub inst_size : usize,
    pub cpu_cycles : usize,
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
    let mut instructions  : [Opcode; 256] = [Opcode{instruction : CPU::no_bind_yet, address_mode : AddressingMode::Implied, inst_size : 0, cpu_cycles : 0}; 256];

    // ADC
    instructions[0x69] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0x65] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x75] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x6d] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0x7d] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0x79] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0x61] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0x71] = Opcode{instruction : CPU::adc, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // AND
    instructions[0x29] = Opcode{instruction : CPU::and, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0x25] = Opcode{instruction : CPU::and, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x35] = Opcode{instruction : CPU::and, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x2d] = Opcode{instruction : CPU::and, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0x3d] = Opcode{instruction : CPU::and, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0x39] = Opcode{instruction : CPU::and, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0x21] = Opcode{instruction : CPU::and, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0x31] = Opcode{instruction : CPU::and, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // ASL 
    instructions[0x0a] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2};
    instructions[0x06] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x16] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x0e] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x1e] = Opcode{instruction : CPU::asl, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};

    // BCC 
    instructions[0x90] = Opcode{instruction : CPU::bcc, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2};

    // BCS
    instructions[0xb0] = Opcode{instruction : CPU::bcs, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2};

    // BEQ
    instructions[0xf0] = Opcode{instruction : CPU::beq, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2};

    // BIT
    instructions[0x24] = Opcode{instruction : CPU::bit, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x2c] = Opcode{instruction : CPU::bit, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};

    // BMI 
    instructions[0x30] = Opcode{instruction : CPU::bmi, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2};

    // BNE
    instructions[0xd0] = Opcode{instruction : CPU::bne, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2}; 

    // BPL
    instructions[0x10] = Opcode{instruction : CPU::bpl, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2};

    // BRK
    instructions[0x00] = Opcode{instruction : CPU::brk, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 7};

    // BVC
    instructions[0x50] = Opcode{instruction : CPU::bvc, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2};

    // BVS
    instructions[0x70] = Opcode{instruction : CPU::bvs, address_mode : AddressingMode::Relative, inst_size : 2, cpu_cycles : 2};

    // CLC
    instructions[0x18] = Opcode{instruction : CPU::clc, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // CLD
    instructions[0xd8] = Opcode{instruction : CPU::cld, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // CLI
    instructions[0x58] = Opcode{instruction : CPU::cli, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // CLV
    instructions[0xb8] = Opcode{instruction : CPU::clv, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // CMP
    instructions[0xc9] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xc5] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xd5] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0xcd] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0xdd] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0xd9] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0xc1] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0xd1] = Opcode{instruction : CPU::cmp, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // CPX
    instructions[0xe0] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xe4] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xec] = Opcode{instruction : CPU::cpx, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    
    // CPY
    instructions[0xc0] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xc4] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xcc] = Opcode{instruction : CPU::cpy, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};

    // DEC
    instructions[0xc6] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0xd6] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0xce] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0xde] = Opcode{instruction : CPU::dec, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};

    // DEX
    instructions[0xca] = Opcode{instruction : CPU::dex, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // DEY
    instructions[0x88] = Opcode{instruction : CPU::dey, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // EOR
    instructions[0x49] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0x45] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x55] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x4d] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0x5d] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0x59] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0x41] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0x51] = Opcode{instruction : CPU::eor, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // INC
    instructions[0xe6] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0xf6] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0xee] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0xfe] = Opcode{instruction : CPU::inc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};

    // INX
    instructions[0xe8] = Opcode{instruction : CPU::inx, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // INY
    instructions[0xc8] = Opcode{instruction : CPU::iny, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // JMP
    instructions[0x4c] = Opcode{instruction : CPU::jmp, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 3};
    instructions[0x6c] = Opcode{instruction : CPU::jmp, address_mode : AddressingMode::Indirect, inst_size : 3, cpu_cycles : 5};

    // JSR
    instructions[0x20] = Opcode{instruction : CPU::jsr, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};

    // LDA
    instructions[0xa9] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xa5] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xb5] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0xad] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0xbd] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0xb9] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0xa1] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0xb1] = Opcode{instruction : CPU::lda, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // LDX
    instructions[0xa2] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xa6] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xb6] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4};
    instructions[0xae] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0xbe] = Opcode{instruction : CPU::ldx, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};

    // LDY
    instructions[0xa0] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xa4] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xb4] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0xac] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0xbc] = Opcode{instruction : CPU::ldy, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};

    // LSR
    instructions[0x4a] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2};
    instructions[0x46] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x56] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x4e] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x5e] = Opcode{instruction : CPU::lsr, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};

    // NOP
    instructions[0xea] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // ORA
    instructions[0x09] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0x05] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x15] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x0d] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0x1d] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0x19] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0x01] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0x11] = Opcode{instruction : CPU::ora, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // PHA
    instructions[0x48] = Opcode{instruction : CPU::pha, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 3};

    // PHP
    instructions[0x08] = Opcode{instruction : CPU::php, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 3};

    // PLA
    instructions[0x68] = Opcode{instruction : CPU::pla, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 4};

    // PLP
    instructions[0x28] = Opcode{instruction : CPU::plp, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 4};

    // ROL
    instructions[0x2a] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2}; 
    instructions[0x26] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x36] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x2e] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x3e] = Opcode{instruction : CPU::rol, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};

    // ROR
    instructions[0x6a] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::Accumulator, inst_size : 1, cpu_cycles : 2};
    instructions[0x66] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x76] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x6e] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x7e] = Opcode{instruction : CPU::ror, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};

    // RTI
    instructions[0x40] = Opcode{instruction : CPU::rti, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 6};

    // RTS
    instructions[0x60] = Opcode{instruction : CPU::rts, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 6};

    // SBC
    instructions[0xe9] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xe5] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xf5] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0xed] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0xfd] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0xf9] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0xe1] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0xf1] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // SEC
    instructions[0x38] = Opcode{instruction : CPU::sec, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // SED
    instructions[0xf8] = Opcode{instruction : CPU::sed, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // SEI
    instructions[0x78] = Opcode{instruction : CPU::sei, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // STA
    instructions[0x85] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x95] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x8d] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0x9d] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 5};
    instructions[0x99] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5};
    instructions[0x81] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0x91] = Opcode{instruction : CPU::sta, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 6};

    // STX
    instructions[0x86] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x96] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4};
    instructions[0x8e] = Opcode{instruction : CPU::stx, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};

    // STY
    instructions[0x84] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x94] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x8c] = Opcode{instruction : CPU::sty, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};

    // TAX
    instructions[0xaa] = Opcode{instruction : CPU::tax, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // TAY
    instructions[0xa8] = Opcode{instruction : CPU::tay, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // TSX
    instructions[0xba] = Opcode{instruction : CPU::tsx, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // TXA
    instructions[0x8a] = Opcode{instruction : CPU::txa, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // TXS
    instructions[0x9a] = Opcode{instruction : CPU::txs, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // TYA
    instructions[0x98] = Opcode{instruction : CPU::tya, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // =================================================================================================================
    // ========================================== Unofficial Opcodes ===================================================
    // =================================================================================================================

    // AAC
    instructions[0x0b] = Opcode{instruction : CPU::aac, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0x2b] = Opcode{instruction : CPU::aac, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};

    // AAX
    instructions[0x87] = Opcode{instruction : CPU::aax, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x97] = Opcode{instruction : CPU::aax, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4};
    instructions[0x83] = Opcode{instruction : CPU::aax, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0x8f] = Opcode{instruction : CPU::aax, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};

    // ARR
    instructions[0x6b] = Opcode{instruction : CPU::arr, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};

    // ASR
    instructions[0x4b] = Opcode{instruction : CPU::asr, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};

    // ATX
    instructions[0xab] = Opcode{instruction : CPU::atx, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};

    // AXA
    instructions[0x9f] = Opcode{instruction : CPU::axa, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5};

    // AXS
    instructions[0xcb] = Opcode{instruction : CPU::axs, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};

    // DCP
    instructions[0xc7] = Opcode{instruction : CPU::dcp, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0xd7] = Opcode{instruction : CPU::dcp, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0xcf] = Opcode{instruction : CPU::dcp, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0xdf] = Opcode{instruction : CPU::dcp, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};
    instructions[0xdb] = Opcode{instruction : CPU::dcp, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7};
    instructions[0xc3] = Opcode{instruction : CPU::dcp, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8};
    instructions[0xd3] = Opcode{instruction : CPU::dcp, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8};

    // DOP
    instructions[0x04] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x14] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x34] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x44] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x54] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x64] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0x74] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0x80] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0x82] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0x89] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xc2] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xd4] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};
    instructions[0xe2] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};
    instructions[0xf4] = Opcode{instruction : CPU::dop, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 4};

    // ISC
    instructions[0xe7] = Opcode{instruction : CPU::isc, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0xf7] = Opcode{instruction : CPU::isc, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0xef] = Opcode{instruction : CPU::isc, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0xff] = Opcode{instruction : CPU::isc, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};
    instructions[0xfb] = Opcode{instruction : CPU::isc, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7};
    instructions[0xe3] = Opcode{instruction : CPU::isc, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8};
    instructions[0xf3] = Opcode{instruction : CPU::isc, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8};

    // KIL
    instructions[0x02] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x12] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x22] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x32] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x42] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x52] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x62] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x72] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x92] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0xb2] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0xd2] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0xf2] = Opcode{instruction : CPU::kil, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // LAR
    instructions[0xbb] = Opcode{instruction : CPU::lar, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};

    // LAX
    instructions[0xa7] = Opcode{instruction : CPU::lax, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 3};
    instructions[0xb7] = Opcode{instruction : CPU::lax, address_mode : AddressingMode::ZeroPageY, inst_size : 2, cpu_cycles : 4};
    instructions[0xaf] = Opcode{instruction : CPU::lax, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0xbf] = Opcode{instruction : CPU::lax, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 4};
    instructions[0xa3] = Opcode{instruction : CPU::lax, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 6};
    instructions[0xb3] = Opcode{instruction : CPU::lax, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 5};

    // NOP
    instructions[0x1a] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x3a] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x5a] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0x7a] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0xda] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};
    instructions[0xfa] = Opcode{instruction : CPU::nop, address_mode : AddressingMode::Implied, inst_size : 1, cpu_cycles : 2};

    // RLA
    instructions[0x27] = Opcode{instruction : CPU::rla, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x37] = Opcode{instruction : CPU::rla, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x2f] = Opcode{instruction : CPU::rla, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x3f] = Opcode{instruction : CPU::rla, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};
    instructions[0x3b] = Opcode{instruction : CPU::rla, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7};
    instructions[0x23] = Opcode{instruction : CPU::rla, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8};
    instructions[0x33] = Opcode{instruction : CPU::rla, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8};

    // RRA
    instructions[0x67] = Opcode{instruction : CPU::rra, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x77] = Opcode{instruction : CPU::rra, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x6f] = Opcode{instruction : CPU::rra, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x7f] = Opcode{instruction : CPU::rra, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};
    instructions[0x7b] = Opcode{instruction : CPU::rra, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7};
    instructions[0x63] = Opcode{instruction : CPU::rra, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8};
    instructions[0x73] = Opcode{instruction : CPU::rra, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8};

    // SBC
    instructions[0xeb] = Opcode{instruction : CPU::sbc, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};

    // SLO
    instructions[0x07] = Opcode{instruction : CPU::slo, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x17] = Opcode{instruction : CPU::slo, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x0f] = Opcode{instruction : CPU::slo, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x1f] = Opcode{instruction : CPU::slo, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};
    instructions[0x1b] = Opcode{instruction : CPU::slo, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7};
    instructions[0x03] = Opcode{instruction : CPU::slo, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8};
    instructions[0x13] = Opcode{instruction : CPU::slo, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8};

    // SRE
    instructions[0x47] = Opcode{instruction : CPU::sre, address_mode : AddressingMode::ZeroPage, inst_size : 2, cpu_cycles : 5};
    instructions[0x57] = Opcode{instruction : CPU::sre, address_mode : AddressingMode::ZeroPageX, inst_size : 2, cpu_cycles : 6};
    instructions[0x4f] = Opcode{instruction : CPU::sre, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 6};
    instructions[0x5f] = Opcode{instruction : CPU::sre, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 7};
    instructions[0x5b] = Opcode{instruction : CPU::sre, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 7};
    instructions[0x43] = Opcode{instruction : CPU::sre, address_mode : AddressingMode::IndirectX, inst_size : 2, cpu_cycles : 8};
    instructions[0x53] = Opcode{instruction : CPU::sre, address_mode : AddressingMode::IndirectY, inst_size : 2, cpu_cycles : 8};

    // SXA
    instructions[0x9e] = Opcode{instruction : CPU::sxa, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5};

    // SYA
    instructions[0x9c] = Opcode{instruction : CPU::sya, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 5};

    // TOP
    instructions[0x0c] = Opcode{instruction : CPU::top, address_mode : AddressingMode::Absolute, inst_size : 3, cpu_cycles : 4};
    instructions[0x1c] = Opcode{instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0x3c] = Opcode{instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0x5c] = Opcode{instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0x7c] = Opcode{instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0xdc] = Opcode{instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};
    instructions[0xfc] = Opcode{instruction : CPU::top, address_mode : AddressingMode::AbsoluteX, inst_size : 3, cpu_cycles : 4};

    // XAA
    instructions[0x8b] = Opcode{instruction : CPU::xaa, address_mode : AddressingMode::Immediate, inst_size : 2, cpu_cycles : 2};

    // XAS
    instructions[0x9b] = Opcode{instruction : CPU::xas, address_mode : AddressingMode::AbsoluteY, inst_size : 3, cpu_cycles : 5};


    instructions
};