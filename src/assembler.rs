use crate::error::Error::{self, *};

use Instruction::*;

#[derive(Debug)]
pub enum Instruction {
    LDrn(u8, u16),
    LDrr(u8, u8),
    LDran(u8, u16),
    LDrar(u8, u8),
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

                    if !matches!(self.current()?, 'r') {
                        return throw(ExpectedReg);
                    }

                    let ra = match self.parse_reg() {
                        Ok(r) => r,
                        Err(e) => return throw(e),
                    };
                    self.skip_whitespace();

                    if !matches!(self.current()?, ',') {
                        return throw(MissingArgSeperator);
                    }
                    self.advance();
                    self.skip_whitespace();

                    match self.current()? {
                        'r' => {
                            let rb = match self.parse_reg() {
                                Ok(r) => r,
                                Err(e) => return throw(e),
                            };

                            return Some(Ok(LDrr(ra, rb)));
                        },
                        _ => todo!()
                    }
                }
                _ => todo!()
            }
            _ => todo!()
        }
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
