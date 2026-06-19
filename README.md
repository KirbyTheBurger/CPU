# Instructions  
**Operands:**  
| Operand | Meaning |
|---------|---------|
|rx       |register x|
|n        |number   |
|[n]      |value at memory adress n|
|[rx]     |value at memory adress stored in rx|

**Instructions:**  
| Opcode | Mnemonic | Operands |
|--------|----------|----------|
|0x01    |LD        |rx, n     |
|0x02    |LD        |rx, ry    |
|0x03    |LD        |rx, [n]   |
|0x04    |LD        |rx, [ry]  |

**Mnemonics**  
| Mnemonic | Meaning |
|----------|---------|
|LD        |Load value into register|
