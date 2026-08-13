const FLAG_ZERO: u8 =  0b0000_0001;
const FLAG_LT: u8 =    0b0000_0010;
const FLAG_CARRY: u8 = 0b0000_0100;

pub const CODE_START: u16 = 0x0000;
pub const CODE_END: u16   = 0x3FFF;
pub const DATA_START: u16 = 0x4000;
pub const DATA_END: u16   = 0xFBFF;
pub const STACK_TOP: u16  = 0xFFFF;
pub const STACK_LOW: u16  = 0xFC00;

pub struct CPU {
    pub reg: [u16; 8],
    pub mem: [u8; 0x10000],
    pub pc: u16,
    pub sp: u16,
    pub running: bool,
    pub flags: u8,
    terminal: console::Term,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: [0; 8],
            mem: [0; _],
            pc: 0,
            sp: STACK_TOP,
            running: false,
            flags: 0,
            terminal: console::Term::stdout(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for b in program {
            self.mem[self.pc as usize] = b;
            self.advance();
        }
        
        self.pc = 0;
    }

    pub fn run(&mut self, debug: bool) -> Result<(), String> {
        self.running = true;

        while self.running {
            self.run_instruction()?;

            if self.pc > CODE_END {
                self.running = false;
                return Err(format!(
                    "Program counter exceeds valid code region in memory ({:#X}-{:#X}), aborting program",
                    CODE_START, CODE_END,
                ));
            }

            if debug {
                println!(
                    "registers: {:?}\npc: {}\nrunning: {}\nflags: {:08b}",
                    self.reg, self.pc, self.running, self.flags,
                );
            }
        }

        Ok(())
    }

    fn run_instruction(&mut self) -> Result<(), String> {
        use crate::instruction::*;

        let byte = self.current();
        let op = byte >> 2;
        let mode = byte & 0b11;

        match op {
            OP_HLT => {
                self.running = false;
            }
            OP_LD => {
                let rx = self.next_byte();

                match mode {
                    MODE_IMM => {
                        let n = self.next_u16();
                        self.set_reg(rx, n);
                    },
                    MODE_REG => {
                        let n = self.next_reg();
                        self.set_reg(rx, n);
                    },
                    MODE_MEM => {
                        let a = self.next_u16();
                        if a < DATA_START || a > DATA_END {
                            return Err(format!(
                                "Attempted to load value from out of bounds memory adress {:#X}", a
                            ));
                        }
                        let n = self.get_addr(a);
                        self.set_reg(rx, n);
                    },
                    MODE_IND => {
                        let a = self.next_reg();
                        if a < DATA_START || a > DATA_END {
                            return Err(format!(
                                "Attempted to load value from out of bounds memory adress {:#X}", a
                            ));
                        }
                        let n = self.get_addr(a);
                        self.set_reg(rx, n);
                    },
                    _ => unreachable!(),
                }
            },
            OP_ST => {
                let a = self.next_reg();
                if a < DATA_START || a > DATA_END {
                    return Err(format!(
                        "Attempted to store value at out of bounds memory adress {:#X}", a
                    ));
                }
                let n = self.next_reg();
                self.set_addr(a, n);
            },
            OP_NOT => {
                let rx = self.next_byte();
                let n = self.get_reg(rx);
                self.set_reg(rx, !n);
            },
            OP_MUL | OP_DIV | OP_MOD => {
                let rx = self.next_byte();
                let n1 = self.get_reg(rx);
                let n2 = self.next_reg();

                let result = match op {
                    OP_MUL => {
                        let (result, carry) = n1.overflowing_mul(n2);
                        self.flags &= !FLAG_CARRY;
                        if carry { self.flags |= FLAG_CARRY };
                        result
                    },
                    OP_DIV => {
                        match n1.checked_div(n2) {
                            Some(n) => n,
                            None => return Err(format!("Attempted to divide {n1} by zero"))
                        }
                    },
                    OP_MOD => {
                        if n2 == 0 {
                            return Err(format!("Attempted to modulo {n1} by zero"));
                        }
                        n1 % n2
                    },
                    _ => unreachable!(),
                };
                self.set_reg(rx, result);
            },
            OP_SUB | OP_ADD => {
                let rx = self.next_byte();
                let n1 = self.get_reg(rx);
                let n2 = match mode {
                    MODE_IMM => self.next_u16(),
                    MODE_REG => self.next_reg(),
                    _ => unreachable!(),
                };
                let (result, carry) = match op {
                    OP_ADD => n1.overflowing_add(n2),
                    OP_SUB => n1.overflowing_sub(n2),
                    _ => unreachable!(),
                };
                self.flags &= !FLAG_CARRY;
                if carry { self.flags |= FLAG_CARRY };
                self.set_reg(rx, result);
            },
            OP_AND | OP_OR | OP_XOR | OP_LSH | OP_RSH => {
                let rx = self.next_byte();
                let n1 = self.get_reg(rx);
                let n2 = match mode {
                    MODE_IMM => self.next_u16(),
                    MODE_REG => self.next_reg(),
                    _ => unreachable!(),
                };

                self.set_reg(rx, match op {
                    OP_AND => n1 & n2,
                    OP_OR => n1 | n2,
                    OP_XOR => n1 ^ n2,
                    OP_LSH => n1 << n2,
                    OP_RSH => n1 >> n2,
                    _ => unreachable!(),
                });
            },
            OP_CMP => {
                let rx = self.next_byte();
                let n1 = self.get_reg(rx);
                let n2 = match mode {
                    MODE_REG => {
                        self.next_reg()
                    },
                    MODE_IMM => {
                        self.next_u16()
                    },
                    _ => unreachable!(),
                };

                self.flags &= !(FLAG_ZERO & FLAG_LT);
                if n1 == n2 { self.flags |= FLAG_ZERO };
                if n1 < n2 { self.flags |= FLAG_LT };
            },
            OP_JMP | OP_JEQ | OP_JNE | OP_JLT | OP_JLE | OP_JGT | OP_JGE => {
                let a = self.next_u16();
                let zero = self.flags & FLAG_ZERO == FLAG_ZERO;
                let lt = self.flags & FLAG_LT == FLAG_LT;

                let jump = match op {
                    OP_JMP => true,
                    OP_JEQ => zero,
                    OP_JNE => !zero,
                    OP_JLT => lt,
                    OP_JLE => lt || zero,
                    OP_JGT => !lt && !zero,
                    OP_JGE => !lt,
                    _ => unreachable!(),
                };

                if jump {
                    self.pc = a;
                    return Ok(());
                }
            },
            OP_OUT => {
                let n = match mode {
                    MODE_REG => self.next_reg(),
                    MODE_IMM => self.next_u16(),
                    _ => unreachable!(),
                };
                let ch = match char::from_u32(n as u32) {
                    Some(c) => c,
                    None => return Err(format!("{:#X} is not valid ASCII", n))
                };
                print!("{ch}");
            },
            OP_IN => {
                let rx = self.next_byte();
                let ch = self.terminal.read_char().unwrap();
                self.set_reg(rx, ch as u16);
            },
            OP_PUSH => {
                let n = match mode {
                    MODE_REG => self.next_reg(),
                    MODE_IMM => self.next_u16(),
                    _ => unreachable!(),
                };
                self.push(n)?;
            },
            OP_POP => {
                let rx = self.next_byte();
                let n = self.pop()?;
                self.set_reg(rx, n);
            },
            OP_CALL => {
                let a = self.next_u16();
                self.push(self.pc + 1)?;
                self.pc = a;
                return Ok(());
            },
            OP_RET => {
                let a = self.pop()?;
                self.pc = a;
                return Ok(());
            },
            OP_LDS => {
                let rx = self.next_byte();
                let offset = self.next_u16();
                if self.sp > STACK_TOP - offset {
                    return Err("LDS adress out of bounds".to_string());
                }
                let a = self.sp + offset;
                let n = self.get_addr(a);
                self.set_reg(rx, n);
            },
            OP_STS => {
                let offset = self.next_u16();
                let n = self.next_reg();
                if self.sp > STACK_TOP - offset {
                    return Err("STS adress out of bounds".to_string());
                }
                let a = self.sp + offset;
                self.set_addr(a, n);
            },
            OP_INC | OP_DEC => {
                let rx = self.next_byte();
                let (result, carry) = match op {
                    OP_INC => self.reg[rx as usize].overflowing_add(1),
                    OP_DEC => self.reg[rx as usize].overflowing_sub(1),
                    _ => unreachable!(),
                };
                self.flags &= !FLAG_CARRY;
                if carry { self.flags |= FLAG_CARRY };
                self.set_reg(rx, result);
            },
            _ => todo!(),
        }

        self.advance();
        Ok(())
    }

    fn push(&mut self, n: u16) -> Result<(), String> {
        if self.sp < STACK_LOW + 2 {
            return Err("Stack out of bounds".to_string());
        }
        self.sp -= 2;
        self.set_addr(self.sp, n);
        Ok(())
    }

    fn pop(&mut self) -> Result<u16, String> {
        if self.sp > STACK_TOP - 2 {
            return Err("Attempted to pop from empty stack".to_string());
        }
        let n = self.get_addr(self.sp);
        self.sp += 2;
        Ok(n)
    }

    fn next_reg(&mut self) -> u16 {
        let r = self.next_byte();
        self.get_reg(r)
    }

    fn set_addr(&mut self, a: u16, n: u16) {
        let b = n.to_be_bytes();

        self.mem[a as usize] = b[0];
        self.mem[a.wrapping_add(1) as usize] = b[1];
    }

    fn get_addr(&self, a: u16) -> u16 {
        let b0 = self.mem[a as usize];
        let b1 = self.mem[a.wrapping_add(1) as usize];

        u16::from_be_bytes([b0, b1])
    }

    fn next_byte(&mut self) -> u8 {
        self.advance();
        self.current()
    }

    fn next_u16(&mut self) -> u16 {
        let b0 = self.next_byte();
        let b1 = self.next_byte();

        u16::from_be_bytes([b0, b1])
    }

    fn set_reg(&mut self, r: u8, v: u16) {
        self.reg[r as usize] = v;
    }

    fn get_reg(&self, r: u8) -> u16 {
        self.reg[r as usize]
    }

    fn advance(&mut self) {
        if self.pc != 0xFFFF {
            self.pc = self.pc.wrapping_add(1);
        }
    }

    fn current(&self) -> u8 {
        self.mem[self.pc as usize]
    }
}