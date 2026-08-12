const FLAG_ZERO: u8 = 0b0000_0001;
const FLAG_LT: u8 = 0b0000_0010;

const CODE_START: u16 = 0x0000;
const CODE_END: u16   = 0x3FFF;
const DATA_START: u16 = 0x4000;
const DATA_END: u16   = 0xFBFF;
const STACK_TOP: u16  = 0xFFFF;
const STACK_LOW: u16  = 0xFC00;

pub struct CPU {
    pub reg: [u16; 8],
    pub mem: [u8; 0x10000],
    pub pc: u16,
    pub running: bool,
    pub flags: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: [0; 8],
            mem: [0; _],
            pc: 0,
            running: false,
            flags: 0,
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for b in program {
            self.mem[self.pc as usize] = b;
            self.advance();
        }
        
        self.pc = 0;
    }

    pub fn run(&mut self, debug: bool) {
        self.running = true;

        while self.running {
            self.run_instruction();

            if debug {
                println!(
                    "registers: {:?}\npc: {}\nrunning: {}\nflags: {:08b}",
                    self.reg, self.pc, self.running, self.flags,
                );
            }
        }
    }

    fn run_instruction(&mut self) {
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
                        let n = self.get_addr(a);
                        self.set_reg(rx, n);
                    },
                    MODE_IND => {
                        let a = self.next_reg();
                        let n = self.get_addr(a);
                        self.set_reg(rx, n);
                    },
                    _ => unreachable!(),
                }
            },
            OP_ST => {
                let a = self.next_reg();
                let n = self.next_reg();
                self.set_addr(a, n);
            },
            OP_NOT => {
                let rx = self.next_byte();
                let n = self.get_reg(rx);
                self.set_reg(rx, !n);
            },
            OP_MUL | OP_DIV => {
                let rx = self.next_byte();
                let n1 = self.get_reg(rx);
                let n2 = self.next_reg();

                self.set_reg(rx, match op {
                    OP_MUL => n1 * n2,
                    OP_DIV => n1 / n2,
                    _ => unreachable!(),
                });
            },
            OP_ADD | OP_SUB | OP_AND..=OP_RSH => {
                let rx = self.next_byte();
                let n1 = self.get_reg(rx);
                let n2 = match mode {
                    MODE_IMM => self.next_u16(),
                    MODE_REG => self.next_reg(),
                    _ => unreachable!(),
                };

                self.set_reg(rx, match op {
                    OP_ADD => n1 + n2,
                    OP_SUB => n1 - n2,
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

                self.flags = 0;
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
                    return;
                }
            },
            _ => todo!(),
        }

        self.advance();
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

    fn get_addr(&self, n: u16) -> u16 {
        let b0 = self.mem[n as usize];
        let b1 = self.mem[n.wrapping_add(1) as usize];

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