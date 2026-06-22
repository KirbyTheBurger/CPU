use std::io::stdin;

use cpu::{assembler::Assembler, cpu::CPU, encoder::Encoder};

fn main() {
    let input = get_input();
    
    let mut assembler = Assembler::new(input);
    let instructions = match assembler.process() {
        Ok(i) => i,
        Err(e) => {
            println!("An error ocurred while parsing the assembly: {e}");
            return;
        },
    };
    println!("{:?}", instructions);

    let mut encoder = Encoder::new(instructions);
    let program = encoder.encode();
    println!("{:?}", program);

    let mut cpu = CPU::new();
    cpu.load(program);
    cpu.run(false);

}

fn get_input() -> String {
    let mut s = String::new();
    if matches!(stdin().read_line(&mut s), Err(_)) {
        s = get_input();
    }
    s.trim().to_string()
}
