pub mod cpu;
pub mod assembler;
pub mod error;
pub mod instruction;
pub mod encoder;

#[cfg(test)]
mod tests {
    use crate::{assembler::Assembler, cpu::CPU, encoder::Encoder};

    fn run(code: &str) -> CPU {
        let mut asm = Assembler::new(code.to_string());
        let instr = match asm.process() {
            Ok(i) => i,
            Err(e) => panic!("{e}"),
        };
        let mut encoder = Encoder::new(instr);
        let program = encoder.encode();
        let mut cpu = CPU::new();
        cpu.load(program);
        cpu.run(false);
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
}