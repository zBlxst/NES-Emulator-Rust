

// ==================================================================================================
// ============================================ UNIT TESTS ==========================================
// ==================================================================================================


#[cfg(test)]
mod test {
    use std::vec;
    use super::super::*;


    impl CPU {
        pub fn test_prog(program: Vec<u8>) -> Self {
            let mut cpu = CPU::new(Rom::new_from_program_rom(program));
            cpu.reset();
            cpu.run();
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

    #[test]
    fn test_stack() {
        // PHA and PLA
        let cpu = CPU::test_prog(vec![0xa9, 0x18, 0x48, 0xa9, 0x12, 0x48, 0xa9, 0xff, 0x68, 0xaa, 0x68, 0xa8, 0x00]);
        assert_eq!(cpu.reg_x, 0x12);
        assert_eq!(cpu.reg_y, 0x18);

        // JSR and RTS
        let cpu = CPU::test_prog(vec![0xe8, 0x20, 0x05, 0x80, 0x00, 0xe8, 0xe8, 0x60, 0x00]);
        cpu.show_stack();
        assert_eq!(cpu.reg_x, 0x03);
        assert_eq!(cpu.reg_pc, 0x8005);
    }


    #[test]
    fn test_misc() {
        // Call a function that adds X and Y to A
        let cpu = CPU::test_prog(vec![0xa2, 0x12, 0xa0, 0x34, 0x20, 0x0a, 0x80, 0x00, 0xa9, 0x00, 0x86, 0x00, 0x98, 0x65, 0x00, 0x60, 0x00]);
        assert_eq!(cpu.reg_a, 0x46);
        assert_eq!(cpu.reg_pc, 0x8008);
    }
    

}