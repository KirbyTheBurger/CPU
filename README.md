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

# Internals and project structure  
The overall layout of this project is what you would expect:  
```
source code -> assembler -> encoder -> execution loop
```  
\n
The assembler transforms source code into an `Instruction` enum, then passes this over to the encoder, which transforms instructions into bytecode. The encoder is small but complex, utilizing an efficient macro (see instruction.rs) to decrease repetitive code. The macro accepts any amount of arguments of the form of `<InstructionName> = <Opcode>`. It then constructs the `Instruction` enum used in the assembler, and implements methods for encoding. The encoder only has to loop through a `Vec<Instruction>` and call the `encode` method on each one. After the source is compiled into bytecode, it gets passed over to the main execution loop, which walks through the bytecode and interprets it.
