# Instructions  
**Operands:**  
| Operand | Meaning |
|---------|---------|
|rx       |Register x|
|n        |Number   |
|[rx]     |Value at memory adress stored in rx|
|[n]      |Value at memory adress n|

**Instructions:**  
| Opcode | Mnemonic | Operands |
|--------|----------|----------|
|0x00    |HLT       |          |
|0x01    |LD        |rx, n     |
|0x02    |LD        |rx, ry    |
|0x03    |LD        |rx, [ry]  |
|0x04    |LD        |rx, [n]   |

**Mnemonics**  
| Mnemonic | Meaning |
|----------|---------|
|HLT       |Stop the current program|
|LD        |Load value into register|
