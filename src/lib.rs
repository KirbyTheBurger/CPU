pub mod cpu;
pub mod assembler;
pub mod error;
pub mod instruction;
pub mod encoder;

#[cfg(test)]
mod tests {
    use crate::{assembler::Assembler, cpu::CPU, encoder::encode};

    fn run(code: &str) -> CPU {
        let mut asm = Assembler::new(code.to_string());
        let items = match asm.process() {
            Ok(i) => i,
            Err(e) => panic!("{e}"),
        };
        let program = match encode(items) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
        let mut cpu = CPU::new();
        cpu.load(program);
        cpu.run(false).unwrap();
        cpu
    }

    #[test]
    fn ld_immediate() {
        let cpu = run("LD r0, 42 HLT");
        assert_eq!(cpu.reg[0], 42);
    }

    #[test]
    fn ld_register() {
        let cpu = run("LD r0, 42 LD r1, r0 HLT");
        assert_eq!(cpu.reg[1], 42);
    }

    #[test]
    fn ld_reg_address() {
        let cpu = run("LD r0, 99 LD r1, 32768 ST [r1], r0 LD r2, [r1] HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn ld_fixed_addr() {
        let cpu = run("LD r0, 77 LD r1, 32768 ST [r1], r0 LD r2, [32768] HLT");
        assert_eq!(cpu.reg[2], 77);
    }

    #[test]
    fn st_roundtrip() {
        let cpu = run("LD r0, 123 LD r1, 32768 ST [r1], r0 LD r2, [r1] HLT");
        assert_eq!(cpu.reg[2], 123);
    }

    #[test]
    fn st_overwrites() {
        let cpu = run("LD r0, 1 LD r1, 2 LD r2, 32768 ST [r2], r0 ST [r2], r1 LD r3, [r2] HLT");
        assert_eq!(cpu.reg[3], 2);
    }

    #[test]
    fn add_register() {
        let cpu = run("LD r0, 10 LD r1, 5 ADD r0, r1 HLT");
        assert_eq!(cpu.reg[0], 15);
    }

    #[test]
    fn add_immediate() {
        let cpu = run("LD r0, 10 ADD r0, 5 HLT");
        assert_eq!(cpu.reg[0], 15);
    }

    #[test]
    fn add_zero() {
        let cpu = run("LD r0, 10 ADD r0, 0 HLT");
        assert_eq!(cpu.reg[0], 10);
    }

    #[test]
    fn sub_register() {
        let cpu = run("LD r0, 10 LD r1, 3 SUB r0, r1 HLT");
        assert_eq!(cpu.reg[0], 7);
    }

    #[test]
    fn sub_immediate() {
        let cpu = run("LD r0, 10 SUB r0, 3 HLT");
        assert_eq!(cpu.reg[0], 7);
    }

    #[test]
    fn sub_to_zero() {
        let cpu = run("LD r0, 5 SUB r0, 5 HLT");
        assert_eq!(cpu.reg[0], 0);
    }

    #[test]
    fn mul_basic() {
        let cpu = run("LD r0, 6 LD r1, 7 MUL r0, r1 HLT");
        assert_eq!(cpu.reg[0], 42);
    }

    #[test]
    fn mul_by_zero() {
        let cpu = run("LD r0, 99 LD r1, 0 MUL r0, r1 HLT");
        assert_eq!(cpu.reg[0], 0);
    }

    #[test]
    fn mul_by_one() {
        let cpu = run("LD r0, 42 LD r1, 1 MUL r0, r1 HLT");
        assert_eq!(cpu.reg[0], 42);
    }

    #[test]
    fn div_basic() {
        let cpu = run("LD r0, 20 LD r1, 4 DIV r0, r1 HLT");
        assert_eq!(cpu.reg[0], 5);
    }

    #[test]
    fn div_integer_truncation() {
        let cpu = run("LD r0, 7 LD r1, 2 DIV r0, r1 HLT");
        assert_eq!(cpu.reg[0], 3);
    }

    #[test]
    fn div_by_self() {
        let cpu = run("LD r0, 42 LD r1, 42 DIV r0, r1 HLT");
        assert_eq!(cpu.reg[0], 1);
    }

    #[test]
    fn hlt_stops_execution() {
        let cpu = run("LD r0, 1 HLT LD r0, 99");
        assert_eq!(cpu.reg[0], 1);
    }

    #[test]
    fn registers_dont_interfere() {
        let cpu = run("LD r0, 1 LD r1, 2 LD r2, 3 LD r3, 4 LD r4, 5 LD r5, 6 LD r6, 7 LD r7, 8 HLT");
        assert_eq!(cpu.reg, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn chained_arithmetic() {
        let cpu = run("LD r0, 10 LD r1, 5 ADD r0, r1 LD r1, 2 MUL r0, r1 LD r1, 3 SUB r0, r1 HLT");
        assert_eq!(cpu.reg[0], 27);
    }

    #[test]
    fn and_register() {
        let cpu = run("LD r0, 12 LD r1, 10 AND r0, r1 HLT");
        assert_eq!(cpu.reg[0], 8);
    }

    #[test]
    fn and_immediate() {
        let cpu = run("LD r0, 12 AND r0, 10 HLT");
        assert_eq!(cpu.reg[0], 8);
    }

    #[test]
    fn and_with_zero() {
        let cpu = run("LD r0, 255 AND r0, 0 HLT");
        assert_eq!(cpu.reg[0], 0);
    }

    #[test]
    fn or_register() {
        let cpu = run("LD r0, 12 LD r1, 10 OR r0, r1 HLT");
        assert_eq!(cpu.reg[0], 14);
    }

    #[test]
    fn or_immediate() {
        let cpu = run("LD r0, 12 OR r0, 10 HLT");
        assert_eq!(cpu.reg[0], 14);
    }

    #[test]
    fn or_with_zero() {
        let cpu = run("LD r0, 42 OR r0, 0 HLT");
        assert_eq!(cpu.reg[0], 42);
    }

    #[test]
    fn xor_register() {
        let cpu = run("LD r0, 12 LD r1, 10 XOR r0, r1 HLT");
        assert_eq!(cpu.reg[0], 6);
    }

    #[test]
    fn xor_immediate() {
        let cpu = run("LD r0, 12 XOR r0, 10 HLT");
        assert_eq!(cpu.reg[0], 6);
    }

    #[test]
    fn xor_with_self() {
        let cpu = run("LD r0, 77 LD r1, 77 XOR r0, r1 HLT");
        assert_eq!(cpu.reg[0], 0);
    }

    #[test]
    fn lsh_register() {
        let cpu = run("LD r0, 1 LD r1, 4 LSH r0, r1 HLT");
        assert_eq!(cpu.reg[0], 16);
    }

    #[test]
    fn lsh_immediate() {
        let cpu = run("LD r0, 1 LSH r0, 4 HLT");
        assert_eq!(cpu.reg[0], 16);
    }

    #[test]
    fn lsh_by_zero() {
        let cpu = run("LD r0, 42 LSH r0, 0 HLT");
        assert_eq!(cpu.reg[0], 42);
    }

    #[test]
    fn rsh_register() {
        let cpu = run("LD r0, 16 LD r1, 4 RSH r0, r1 HLT");
        assert_eq!(cpu.reg[0], 1);
    }

    #[test]
    fn rsh_immediate() {
        let cpu = run("LD r0, 16 RSH r0, 4 HLT");
        assert_eq!(cpu.reg[0], 1);
    }

    #[test]
    fn rsh_by_zero() {
        let cpu = run("LD r0, 42 RSH r0, 0 HLT");
        assert_eq!(cpu.reg[0], 42);
    }

    #[test]
    fn not() {
        let cpu = run("LD r0, 5 NOT r0 HLT");
        assert_eq!(cpu.reg[0], !5u16);
    }

    #[test]
    fn not_zero() {
        let cpu = run("LD r0, 0 NOT r0 HLT");
        assert_eq!(cpu.reg[0], !0u16);
    }

    #[test]
    fn not_twice_is_identity() {
        let cpu = run("LD r0, 12345 NOT r0 NOT r0 HLT");
        assert_eq!(cpu.reg[0], 12345);
    }

    #[test]
    fn combined_bitwise_chain() {
        let cpu = run("LD r0, 15 AND r0, 10 OR r0, 1 XOR r0, 15 LSH r0, 2 RSH r0, 1 HLT");
        assert_eq!(cpu.reg[0], 8);
    }

    #[test]
    fn ld_reg_address_different_registers() {
        let cpu = run("LD r0, 88 LD r5, 32768 ST [r5], r0 LD r3, [r5] HLT");
        assert_eq!(cpu.reg[3], 88);
    }

    #[test]
    fn st_different_address() {
        let cpu = run("LD r0, 12 LD r1, 40000 ST [r1], r0 LD r2, [40000] HLT");
        assert_eq!(cpu.reg[2], 12);
    }

    #[test]
    fn jmp_basic_forward() {
        let cpu = run("JMP skip LD r0, 99 skip: LD r0, 1 HLT");
        assert_eq!(cpu.reg[0], 1);
    }

    #[test]
    fn jmp_skips_intermediate_instructions() {
        let cpu = run("LD r0, 1 JMP target LD r0, 2 LD r0, 3 target: LD r1, 42 HLT");
        assert_eq!(cpu.reg[0], 1);
        assert_eq!(cpu.reg[1], 42);
    }

    #[test]
    fn jmp_to_label_at_start() {
        let cpu = run("start: LD r0, 7 HLT");
        assert_eq!(cpu.reg[0], 7);
    }

    #[test]
    fn cmp_equal_sets_zero_flag() {
        let cpu = run("LD r0, 5 LD r1, 5 CMP r0, r1 HLT");
        assert_eq!(cpu.flags & 0b1, 0b1);
        assert_eq!(cpu.flags & 0b10, 0);
    }

    #[test]
    fn cmp_less_than_sets_lt_flag() {
        let cpu = run("LD r0, 3 LD r1, 10 CMP r0, r1 HLT");
        assert_eq!(cpu.flags & 0b10, 0b10);
        assert_eq!(cpu.flags & 0b1, 0);
    }

    #[test]
    fn cmp_greater_than_clears_both_flags() {
        let cpu = run("LD r0, 10 LD r1, 3 CMP r0, r1 HLT");
        assert_eq!(cpu.flags & 0b1, 0);
        assert_eq!(cpu.flags & 0b10, 0);
    }

    #[test]
    fn cmp_immediate_equal() {
        let cpu = run("LD r0, 7 CMP r0, 7 HLT");
        assert_eq!(cpu.flags & 0b1, 0b1);
        assert_eq!(cpu.flags & 0b10, 0);
    }

    #[test]
    fn cmp_immediate_less_than() {
        let cpu = run("LD r0, 2 CMP r0, 9 HLT");
        assert_eq!(cpu.flags & 0b10, 0b10);
        assert_eq!(cpu.flags & 0b1, 0);
    }

    #[test]
    fn cmp_immediate_greater_than() {
        let cpu = run("LD r0, 9 CMP r0, 2 HLT");
        assert_eq!(cpu.flags & 0b1, 0);
        assert_eq!(cpu.flags & 0b10, 0);
    }

    #[test]
    fn cmp_with_zero() {
        let cpu = run("LD r0, 0 LD r1, 0 CMP r0, r1 HLT");
        assert_eq!(cpu.flags & 0b1, 0b1);
        assert_eq!(cpu.flags & 0b10, 0);
    }

    #[test]
    fn cmp_zero_less_than_nonzero() {
        let cpu = run("LD r0, 0 LD r1, 1 CMP r0, r1 HLT");
        assert_eq!(cpu.flags & 0b10, 0b10);
    }

    #[test]
    fn cmp_overwrites_previous_flags() {
        let cpu = run("LD r0, 1 LD r1, 5 CMP r0, r1 LD r2, 5 CMP r2, r1 HLT");
        assert_eq!(cpu.flags & 0b1, 0b1);
        assert_eq!(cpu.flags & 0b10, 0);
    }

    #[test]
    fn cmp_does_not_modify_registers() {
        let cpu = run("LD r0, 10 LD r1, 3 CMP r0, r1 HLT");
        assert_eq!(cpu.reg[0], 10);
        assert_eq!(cpu.reg[1], 3);
    }

    #[test]
    fn jeq_taken_when_equal() {
        let cpu = run("LD r0, 5 LD r1, 5 CMP r0, r1 JEQ target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jeq_not_taken_when_not_equal() {
        let cpu = run("LD r0, 5 LD r1, 3 CMP r0, r1 JEQ target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn jne_taken_when_not_equal() {
        let cpu = run("LD r0, 5 LD r1, 3 CMP r0, r1 JNE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jne_not_taken_when_equal() {
        let cpu = run("LD r0, 5 LD r1, 5 CMP r0, r1 JNE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn jlt_taken_when_less() {
        let cpu = run("LD r0, 3 LD r1, 10 CMP r0, r1 JLT target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jlt_not_taken_when_equal() {
        let cpu = run("LD r0, 5 LD r1, 5 CMP r0, r1 JLT target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn jlt_not_taken_when_greater() {
        let cpu = run("LD r0, 10 LD r1, 3 CMP r0, r1 JLT target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn jge_taken_when_greater() {
        let cpu = run("LD r0, 10 LD r1, 3 CMP r0, r1 JGE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jge_taken_when_equal() {
        let cpu = run("LD r0, 5 LD r1, 5 CMP r0, r1 JGE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jge_not_taken_when_less() {
        let cpu = run("LD r0, 3 LD r1, 10 CMP r0, r1 JGE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn jgt_taken_when_greater() {
        let cpu = run("LD r0, 10 LD r1, 3 CMP r0, r1 JGT target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jgt_not_taken_when_equal() {
        let cpu = run("LD r0, 5 LD r1, 5 CMP r0, r1 JGT target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn jgt_not_taken_when_less() {
        let cpu = run("LD r0, 3 LD r1, 10 CMP r0, r1 JGT target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn jle_taken_when_less() {
        let cpu = run("LD r0, 3 LD r1, 10 CMP r0, r1 JLE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jle_taken_when_equal() {
        let cpu = run("LD r0, 5 LD r1, 5 CMP r0, r1 JLE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 0);
    }

    #[test]
    fn jle_not_taken_when_greater() {
        let cpu = run("LD r0, 10 LD r1, 3 CMP r0, r1 JLE target LD r2, 99 target: HLT");
        assert_eq!(cpu.reg[2], 99);
    }

    #[test]
    fn countdown_loop_with_jlt() {
        let cpu = run("
            LD r0, 0
            LD r1, 1
            LD r2, 5
            loop: ADD r0, r1
            CMP r0, r2
            JLT loop
            HLT
        ");
        assert_eq!(cpu.reg[0], 5);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_mem_address() {
        run("LD r0, [1]");
    }

    #[test]
    fn ld_hex_literal() {
        let cpu = run("LD r0, 0xFF HLT");
        assert_eq!(cpu.reg[0], 255);
    }

    #[test]
    fn ld_hex_literal_lowercase() {
        let cpu = run("LD r0, 0xff HLT");
        assert_eq!(cpu.reg[0], 255);
    }

    #[test]
    fn ld_hex_literal_mixed_case() {
        let cpu = run("LD r0, 0xAb12 HLT");
        assert_eq!(cpu.reg[0], 0xAB12);
    }

    #[test]
    fn ld_hex_zero() {
        let cpu = run("LD r0, 0x0 HLT");
        assert_eq!(cpu.reg[0], 0);
    }

    #[test]
    fn ld_hex_max_u16() {
        let cpu = run("LD r0, 0xFFFF HLT");
        assert_eq!(cpu.reg[0], 65535);
    }

    #[test]
    fn ld_hex_single_digit() {
        let cpu = run("LD r0, 0x5 HLT");
        assert_eq!(cpu.reg[0], 5);
    }

    #[test]
    fn ld_binary_literal() {
        let cpu = run("LD r0, 0b1010 HLT");
        assert_eq!(cpu.reg[0], 10);
    }

    #[test]
    fn ld_binary_zero() {
        let cpu = run("LD r0, 0b0 HLT");
        assert_eq!(cpu.reg[0], 0);
    }

    #[test]
    fn ld_binary_single_bit() {
        let cpu = run("LD r0, 0b1 HLT");
        assert_eq!(cpu.reg[0], 1);
    }

    #[test]
    fn ld_binary_all_ones_byte() {
        let cpu = run("LD r0, 0b11111111 HLT");
        assert_eq!(cpu.reg[0], 255);
    }

    #[test]
    fn ld_binary_16_bits() {
        let cpu = run("LD r0, 0b1111111111111111 HLT");
        assert_eq!(cpu.reg[0], 65535);
    }

    #[test]
    fn ld_decimal_still_works_after_zero() {
        let cpu = run("LD r0, 0 HLT");
        assert_eq!(cpu.reg[0], 0);
    }

    #[test]
    fn ld_decimal_normal_number_unaffected() {
        let cpu = run("LD r0, 42 HLT");
        assert_eq!(cpu.reg[0], 42);
    }

    #[test]
    fn hex_and_decimal_in_same_program() {
        let cpu = run("LD r0, 0x10 LD r1, 16 CMP r0, r1 HLT");
        assert_eq!(cpu.flags & 0b1, 0b1);
    }

    #[test]
    fn binary_in_arithmetic() {
        let cpu = run("LD r0, 0b1100 AND r0, 0b1010 HLT");
        assert_eq!(cpu.reg[0], 0b1000);
    }

    #[test]
    fn hex_in_shift() {
        let cpu = run("LD r0, 0x1 LSH r0, 0x4 HLT");
        assert_eq!(cpu.reg[0], 16);
    }
}