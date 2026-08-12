use std::collections::HashMap;

use crate::{assembler::{Item, JumpKind}, cpu::{CODE_END, CODE_START}, instruction::Instruction};

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
            Item::UnresolvedJump{..} | Item::UnresolvedCall(_) => {
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
            Item::UnresolvedJump { kind, label } => {
                let target = labels.get(&label)
                    .ok_or_else(|| format!("undefined label: {label}"))?;
                bytes.extend(match kind {
                    JumpKind::JMP => Instruction::JMP(*target),
                    JumpKind::JEQ => Instruction::JEQ(*target),
                    JumpKind::JNE => Instruction::JNE(*target),
                    JumpKind::JLT => Instruction::JLT(*target),
                    JumpKind::JLE => Instruction::JLE(*target),
                    JumpKind::JGT => Instruction::JGT(*target),
                    JumpKind::JGE => Instruction::JGE(*target),
                }.encode());
            },
            Item::UnresolvedCall(label) => {
                let target = labels.get(&label)
                    .ok_or_else(|| format!("undefined label: {label}"))?;
                bytes.extend(Instruction::CALL(*target).encode());
            }
        }
    }

    if bytes.len() > CODE_END as usize {
        return Err(format!(
            "Program size out of bounds; Reserved size for code: {:#X}-{:#X}, program size: {} bytes",
            CODE_START, CODE_END, bytes.len()
        ));
    }

    Ok(bytes)
}