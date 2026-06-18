use std::io::stdin;

use crate::assembler::Assembler;

mod cpu;
mod assembler;
mod error;

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
}

fn get_input() -> String {
    let mut s = String::new();
    if matches!(stdin().read_line(&mut s), Err(_)) {
        s = get_input();
    }
    s.trim().to_string()
}
