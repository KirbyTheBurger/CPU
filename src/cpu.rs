use crate::error::Error::{self, *};

pub struct CPU {
    pub reg: [u16; 8],
    pub mem: [u8; 0xFFFF],
    pub pc: u16,
    pub running: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: [0; 8],
            mem: [0; 0xFFFF],
            pc: 0,
            running: false,
        }
    }

    pub fn load(&mut self, program: Vec<u8>) -> Result<(), Error> {
        for b in program {
            self.mem[self.pc as usize] = b;
            self.advance()?;
        }
        
        self.pc = 0;
        Ok(())
    }

    pub fn run(&mut self, debug: bool) -> Result<(), Error> {
        self.running = true;

        while self.running {
            self.run_instruction()?;

            if debug {
                println!("registers: {:?}\npc: {}\nrunning: {}", self.reg, self.pc, self.running);
            }
        }

        Ok(())
    }

    fn run_instruction(&mut self) -> Result<(), Error> {
        match self.current() {
            0x00 => {
                self.running = false;
            }
            0x01 => {
                let rx = self.next_byte()?;
                let n = self.next_u16()?;
                self.set_reg(rx, n);
            },
            0x02 => {
                let rx = self.next_byte()?;
                let ry = self.next_byte()?;
                let n = self.get_reg(ry);
                self.set_reg(rx, n);
            },
            0x03 => {
                let rx = self.next_byte()?;
                let ry = self.next_byte()?;
                let n = self.get_addr(ry);
                self.set_reg(rx, n as u16);
            },
            _ => todo!(),
        }

        self.advance()?;
        Ok(())
    }

    fn get_addr(&self, r: u8) -> u8 {
        let n = self.get_reg(r);
        self.mem[n as usize]
    }

    fn next_byte(&mut self) -> Result<u8, Error> {
        self.advance()?;
        Ok(self.current())
    }

    fn next_u16(&mut self) -> Result<u16, Error> {
        let b0 = self.next_byte()?;
        let b1 = self.next_byte()?;

        Ok(u16::from_be_bytes([b0, b1]))
    }

    fn set_reg(&mut self, r: u8, v: u16) {
        self.reg[r as usize] = v;
    }

    fn get_reg(&self, r: u8) -> u16 {
        self.reg[r as usize]
    }

    fn advance(&mut self) -> Result<(), Error> {
        if self.pc != 0xFFFF {
            self.pc += 1;
            Ok(())
        } else {
            Err(PcAboveCap)
        }
    }

    fn current(&self) -> u8 {
        self.mem[self.pc as usize]
    }
}