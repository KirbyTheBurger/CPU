use std::fmt::Display;

use crate::{error::Error::{self, *}, instruction::Instruction::{self, *}};

use Operand::*;

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(u8),
    Number(u16),
    RegAdress(u8),
    Adress(u16),
}

pub enum Item {
    Instruction(Instruction),
    Label(String),
}

pub struct Assembler {
    input: Vec<char>,
    pos: usize,
}

macro_rules! args {
    ($self:expr, $n:expr) => {
        match $self.parse_args($n) {
            Ok(a) => a,
            Err(e) => return throw(e),
        }
    };
}

/**
    Macro for ensuring an argument is valid, will throw the appropiate error if not.
    Possible inputs:
    - reg: throws `ExpectedReg`
    - num: throws `ExpectedNum`
    - regaddr: throws `ExpectedRegAddr`
*/
macro_rules! ensure {
    ($arg:expr, reg) => {
        match $arg {
            Register(x) => x,
            _ => return throw(ExpectedReg),
        }
    };

    ($arg:expr, num) => {
        match $arg {
            Number(x) => x,
            _ => return throw(ExpectedNum),
        }
    };
    
    ($arg:expr, regaddr) => {
        match $arg {
            RegAdress(x) => x,
            _ => return throw(ExpectedRegAddr),
        }
    };
}

impl Assembler {
    pub fn new(input: String) -> Assembler {
        Assembler {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn process(&mut self) -> Result<Vec<Item>, Error> {
        let mut instr = vec![];

        self.skip_whitespace();

        loop {
            if let Some(i) = self.process_instruction() {
                instr.push(i?);
            } else {
                break;
            }

            self.skip_whitespace();
        }

        Ok(instr)
    }

    fn process_instruction(&mut self) -> Option<Result<Item, Error>> {
        let word = self.read_word()?;
        let word = word.as_str();

        if word.ends_with(':') && word.len() > 1 {
            return Some(Ok(Item::Label(
                word[..word.len() - 1].to_string()
            )));
        }

        match word {
            "LD" => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], reg);

                match args[1] {
                    Register(ry) => instr(LDrr(rx, ry)),
                    Number(n) => instr(LDrn(rx, n)),
                    RegAdress(ry) => instr(LDrar(rx, ry)),
                    Adress(n) => instr(LDran(rx, n)),
                }
            },
            "HLT" => {
                instr(HLT)
            },
            "ST" => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], regaddr);
                let ry = ensure!(args[1], reg);

                instr(ST(rx, ry))
            },
            "MUL" | "DIV" => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], reg);
                let ry = ensure!(args[1], reg);

                match word {
                    "MUL" => instr(MUL(rx, ry)),
                    "DIV" => instr(DIV(rx, ry)),
                    _ => unreachable!()
                }
            },
            "NOT" => {
                let rx = ensure!(args!(self, 1)[0], reg);
                instr(NOT(rx))
            },
            "AND" | "OR" | "XOR" | "LSH" | "RSH" | "SUB" | "ADD" => {
                let args = args!(self, 2);
                let rx = ensure!(args[0], reg);

                match args[1] {
                    Register(ry) => match word {
                        "AND" => instr(ANDrr(rx, ry)),
                        "OR" => instr(ORrr(rx, ry)),
                        "XOR" => instr(XORrr(rx, ry)),
                        "LSH" => instr(LSHrr(rx, ry)),
                        "RSH" => instr(RSHrr(rx, ry)),
                        "ADD" => instr(ADDrr(rx, ry)),
                        "SUB" => instr(SUBrr(rx, ry)),
                        _ => unreachable!(),
                    },
                    Number(n) => match word {
                        "AND" => instr(ANDrn(rx, n)),
                        "OR" => instr(ORrn(rx, n)),
                        "XOR" => instr(XORrn(rx, n)),
                        "LSH" => instr(LSHrn(rx, n)),
                        "RSH" => instr(RSHrn(rx, n)),
                        "ADD" => instr(ADDrn(rx, n)),
                        "SUB" => instr(SUBrn(rx, n)),
                        _ => unreachable!(),
                    },
                    _ => throw(InvalidArg),
                }
            },
            _ => todo!()
        }
    }

    fn parse_args(&mut self, amount: u8) -> Result<Vec<Operand>, Error> {
        let mut args = vec![];

        self.skip_whitespace();

        for i in 0..amount {
            let current = *match self.current() {
                Some(c) => c,
                None => return Err(NotEnoughArgs)
            };

            match current {
                'r' => {
                    let reg = match self.parse_reg() {
                        Ok(r) => r,
                        Err(e) => return Err(e),
                    };
                    args.push(Register(reg));
                },
                '[' => {
                    self.advance();
                    let addr = self.parse_args(1)?[0];

                    match self.current() {
                        Some(']') => self.advance(),
                        Some(c) => return Err(BracketCloseExpected(*c)),
                        None => return Err(BracketCloseEOF),
                    }

                    match addr {
                        Register(r) => args.push(RegAdress(r)),
                        Number(n) => args.push(Adress(n)),
                        _ => return Err(InvalidAddr(addr))
                    }
                },
                c if c.is_numeric() => {
                    let mut s = String::from(c);
                    loop {
                        self.advance();
                        let current = match self.current() {
                            Some(c) => c,
                            None => break,
                        };
                        if current.is_numeric() {
                            s.push(*current);
                        } else {
                            break;
                        }
                    }

                    let num = match s.parse::<u16>() {
                        Ok(n) => n,
                        Err(_) => return Err(NumAboveCap(s)),
                    };
                    args.push(Number(num));
                },
                _ => return Err(InvalidArg),
            }

            if i == amount - 1 {
                break;
            }

            self.skip_whitespace();

            if !matches!(self.current(), Some(',')) {
                return Err(MissingArgSeperator);
            }
            self.advance();
            self.skip_whitespace();
        }

        Ok(args)
    }

    fn read_word(&mut self) -> Option<String> {
        let mut s = String::new();

        loop {
            let c = self.current()?;
            if c.is_whitespace() {
                self.skip_whitespace();
                break;
            }
            s.push(*c);
            self.advance();
        }

        Some(s)
    }

    fn parse_reg(&mut self) -> Result<u8, Error> {
        let next = match self.next() {
            Some(c) => *c,
            None => return Err(MissingRegIndex),
        };
        if matches!(next, '0'..='7') {
            self.advance();
            Ok(next as u8 - b'0')
        } else {
            Err(InvalidReg(next))
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn next(&mut self) -> Option<&char> {
        self.advance();
        self.current()
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.current() && c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}

fn throw(err: Error) -> Option<Result<Item, Error>> {
    Some(Err(err))
}

fn instr(i: Instruction) -> Option<Result<Item, Error>> {
    Some(Ok(Item::Instruction(i)))
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register(r) => write!(f, "r{r}"),
            Number(n) => write!(f, "{n}"),
            RegAdress(r) => write!(f, "[r{r}]"),
            Adress(n) => write!(f, "[{n}]"),
        }
    }
}