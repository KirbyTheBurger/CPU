use std::io::stdin;

use crate::{assembler::Assembler, encoder::Encoder};

mod cpu;
mod assembler;
mod error;
mod instruction;
mod encoder;

fn main() {
    let input = get_input();
    
    let mut assembler = Assembler::new(input);
    let instructions = match assembler.process() {
        Ok(i) => i,
        Err(e) => {
            println!("{e}");
            return;
        },
    };
    println!("{:?}", instructions);

    let mut encoder = Encoder::new(instructions);
    let program = encoder.encode();
    println!("{:?}", program);
}

fn get_input() -> String {
    let mut s = String::new();
    if matches!(stdin().read_line(&mut s), Err(_)) {
        s = get_input();
    }
    s.trim().to_string()
}
