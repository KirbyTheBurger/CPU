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
                println!("registers: {:?}\npc: {}\nrunning: {}", self.reg, self.pc, self.running);
            }
        }
    }

    fn run_instruction(&mut self) {
        match self.current() {
            0x00 => {
                self.running = false;
            }
            0x01 => {
                let rx = self.next_byte();
                let n = self.next_u16();
                self.set_reg(rx, n);
            },
            0x02 => {
                let rx = self.next_byte();
                let ry = self.next_byte();
                let n = self.get_reg(ry);
                self.set_reg(rx, n);
            },
            0x03 => {
                let rx = self.next_byte();
                let ry = self.next_byte();
                let n = self.get_reg_addr(ry);
                self.set_reg(rx, n);
            },
            0x04 => {
                let rx = self.next_byte();
                let a = self.next_u16();
                let n = self.get_addr(a);
                self.set_reg(rx, n);
            },
            0x05 => {
                let rx = self.next_byte();
                let ry = self.next_byte();
                self.set_addr(rx, ry);
            }
            _ => todo!(),
        }

        self.advance();
    }

    fn set_addr(&mut self, arx: u8, ry: u8) {
        let a = self.get_reg(arx);
        let n = self.get_reg(ry);
        let b = n.to_be_bytes();

        self.mem[a as usize] = b[0];
        self.mem[a.wrapping_add(1) as usize] = b[1];
    }

    fn get_addr(&self, n: u16) -> u16 {
        let b0 = self.mem[n as usize];
        let b1 = self.mem[n.wrapping_add(1) as usize];

        u16::from_be_bytes([b0, b1])
    }

    fn get_reg_addr(&self, r: u8) -> u16 {
        let n = self.get_reg(r);
        self.get_addr(n)
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