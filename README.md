# Memory layout
**Memory map:**
|    Region    | Purpose |
|--------------|---------|
|0x0000-0x3FFF |Program code|
|0x4000-0x7FFF |Static memory|
|0x8000-0xFFFF |Stack    |

**Registers:**
| Register | Internal id | Purpose |
|----------|-------------|---------|
|r0-r7     |0x00-0x07    |General purpose registers|
|pc        |0x08         |Program counter|
|sp        |0x09         |Stack pointer|

# Instructions  
**Operands:**  
| Operand | Meaning |
|---------|---------|
|rx       |Register x|
|n        |Number   |
|[rx]     |Value at memory adress stored in rx|
|[n]      |Value at memory adress n|

**Instructions:**  
| Opcode | Mnemonic | Operands  |
|--------|----------|-----------|
| 0x00   | HLT      |           |
| 0x04   | LD       | rx, ry    |
| 0x05   | LD       | rx, n     |
| 0x06   | LD       | rx, [ry]  |
| 0x07   | LD       | rx, [n]   |
| 0x08   | ST       | [rx], ry  |
| 0x0C   | ADD      | rx, ry    |
| 0x0D   | ADD      | rx, n     |
| 0x10   | SUB      | rx, ry    |
| 0x11   | SUB      | rx, n     |
| 0x14   | MUL      | rx, ry    |
| 0x18   | DIV      | rx, ry    |
| 0x1C   | NOT      | rx        |
| 0x20   | AND      | rx, ry    |
| 0x21   | AND      | rx, n     |
| 0x24   | OR       | rx, ry    |
| 0x25   | OR       | rx, n     |
| 0x28   | XOR      | rx, ry    |
| 0x29   | XOR      | rx, n     |
| 0x2C   | LSH      | rx, ry    |
| 0x2D   | LSH      | rx, n     |
| 0x30   | RSH      | rx, ry    |
| 0x31   | RSH      | rx, n     |

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
|NOT       |Bitwise NOT|
|AND       |Bitwise AND|
|OR        |Bitwise OR|
|XOR       |Bitwise XOR|
|LSH       |Left shift|
|RSH       |Right shift|

# Internals and project structure  
The overall layout of this project is what you would expect:  
```
source code -> assembler -> encoder -> execution loop
```  
\n
The assembler transforms source code into an `Instruction` enum, then passes this over to the encoder, which transforms instructions into bytecode. The encoder is small but complex, utilizing an efficient macro (see instruction.rs) to decrease repetitive code. The macro accepts any amount of arguments of the form of `<InstructionName> = <Opcode>`. It then constructs the `Instruction` enum used in the assembler, and implements methods for encoding. The encoder only has to loop through a `Vec<Instruction>` and call the `encode` method on each one. After the source is compiled into bytecode, it gets passed over to the main execution loop, which walks through the bytecode and interprets it.
