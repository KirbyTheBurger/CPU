use crate::instruction::Instruction;

pub struct Encoder {
    input: Vec<Instruction>,
    pos: usize,
}

impl Encoder {
    pub fn new(input: Vec<Instruction>) -> Encoder {
        Encoder {
            input,
            pos: 0,
        }
    }

    pub fn encode(&mut self) -> Vec<u8> {
        let mut bytes = vec![];

        while let Some(i) = self.current() {
            bytes.extend(i.encode());
            self.advance();
        }

        bytes
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&Instruction> {
        self.input.get(self.pos)
    }
}