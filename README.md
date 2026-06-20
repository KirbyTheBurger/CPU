# Instructions  
**Operands:**  
| Operand | Meaning |
|---------|---------|
|rx       |register x|
|n        |number   |
|[rx]     |value at memory adress stored in rx|

**Instructions:**  
| Opcode | Mnemonic | Operands |
|--------|----------|----------|
|0x00    |HLT       |          |
|0x01    |LD        |rx, n     |
|0x02    |LD        |rx, ry    |
|0x03    |LD        |rx, [ry]  |

**Mnemonics**  
| Mnemonic | Meaning |
|----------|---------|
|HLT       |Stop the current program|
|LD        |Load value into register|
