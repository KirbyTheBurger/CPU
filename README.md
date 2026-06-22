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
|0x05    |ST        |[rx], ry  |
|0x06    |ADD       |rx, ry    |
|0x07    |ADD       |rx, n     |
|0x08    |SUB       |rx, ry    |
|0x09    |SUB       |rx, n     |
|0x0A    |MUL       |rx, ry    |
|0x0B    |DIV       |rx, ry    |

**Mnemonics:**  
| Mnemonic | Meaning |
|----------|---------|
|HLT       |Stop the current program|
|LD        |Load value into register|
|ST        |Store value in memory|
|ADD       |Add a value to another value|
|SUB       |Subtract a value from another|
|MUL       |Multiply a value by another|
|DIV       |Divide a value by another|
