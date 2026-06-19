use std::fmt::Display;

use crate::error::Error::{self, *};

use Instruction::*;
use Operand::*;

#[derive(Debug)]
pub enum Instruction {
    LDrn(u8, u16),
    LDrr(u8, u8),
    LDran(u8, u16),
    LDrar(u8, u8),
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(u8),
    Number(u16),
    RegAddress(u8),
    NumAdress(u16),
}

pub struct Assembler {
    input: Vec<char>,
    pos: usize,
}

impl Assembler {
    pub fn new(input: String) -> Assembler {
        Assembler {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn process(&mut self) -> Result<Vec<Instruction>, Error> {
        let mut instr = vec![];

        self.skip_whitespace();

        while let Some(&c) = self.current() {
            if let Some(i) = self.process_instruction(c) {
                instr.push(i?);
            }

            self.skip_whitespace();
        }

        Ok(instr)
    }

    fn process_instruction(&mut self, c: char) -> Option<Result<Instruction, Error>> {
        match c {
            'L' => match self.next()? {
                'D' => {
                    self.advance();
                    self.skip_whitespace();

                    let args = match self.parse_args(2) {
                        Ok(a) => a,
                        Err(e) => return throw(e),
                    };

                    let rx = match args[0] {
                        Register(r) => r,
                        _ => return throw(ExpectedReg),
                    };

                    match args[1] {
                        Register(ry) => instr(LDrr(rx, ry)),
                        Number(n) => instr(LDrn(rx, n)),
                        RegAddress(ry) => instr(LDrar(rx, ry)),
                        NumAdress(n) => instr(LDran(rx, n)),
                    }
                }
                _ => todo!()
            }
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
                        Register(r) => args.push(RegAddress(r)),
                        Number(n) => args.push(NumAdress(n)),
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
                            s.push(c);
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

    fn parse_reg(&mut self) -> Result<u8, Error> {
        let next = match self.next() {
            Some(c) => *c,
            None => return Err(MissingRegIndex),
        };
        if matches!(next, '0'..'7') {
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

fn throw(err: Error) -> Option<Result<Instruction, Error>> {
    Some(Err(err))
}

fn instr(i: Instruction) -> Option<Result<Instruction, Error>> {
    Some(Ok(i))
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register(r) => write!(f, "r{r}"),
            Number(n) => write!(f, "{n}"),
            RegAddress(r) => write!(f, "[r{r}]"),
            NumAdress(n) => write!(f, "[{n}]"),
        }
    }
}