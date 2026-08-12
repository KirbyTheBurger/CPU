use std::collections::HashMap;

use crate::{assembler::Item, instruction::Instruction};

pub fn encode(items: Vec<Item>) -> Result<Vec<u8>, String> {
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut adress: u16 = 0;

    for item in &items {
        match item {
            Item::Label(s) => {
                labels.insert(s.clone(), adress);
            },
            Item::Instruction(i) => {
                adress += i.encode().len() as u16;
            },
            Item::UnresolvedJump(_) => {
                adress += 3;
            }
        }
    }

    let mut bytes = vec![];

    for item in items {
        match item {
            Item::Label(_) => {},
            Item::Instruction(i) => {
                bytes.extend(i.encode());
            },
            Item::UnresolvedJump(s) => {
                let target = labels.get(&s)
                    .ok_or_else(|| format!("undefined label: {s}"))?;
                bytes.extend(Instruction::JMP(*target).encode());
            },
        }
    }

    Ok(bytes)
}