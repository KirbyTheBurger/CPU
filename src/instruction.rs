trait Encode {
    fn encode(&self) -> Vec<u8>;
}

impl Encode for u8 {
    fn encode(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Encode for u16 {
    fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

macro_rules! instructions {
    (
        $(
            $variant:ident $( ( $t0:ty $(, $t1:ty $(, $t2:ty)? )? ) )? = $opcode:expr
        ),* $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy)]
            pub enum Instruction {
                $( $variant $( ( $t0 $(, $t1 $(, $t2)? )? ) )? ),*
            }

            impl Instruction {
                #[allow(unused_variables)]
                pub fn opcode(&self) -> u8 {
                    match self {
                        $(
                            Instruction::$variant $( ([<f0_ $t0>], ..) )? => $opcode
                        ),*
                    }
                }

                pub fn encode(&self) -> Vec<u8> {
                    let mut bytes = vec![self.opcode()];
                    match self {
                        $(
                            Instruction::$variant
                                $( (
                                    [<f0_ $t0>]
                                    $(, [<f1_ $t1>] $(, [<f2_ $t2>])? )?
                                ) )?
                            => {
                                $(
                                    bytes.extend([<f0_ $t0>].encode());
                                    $(
                                        bytes.extend([<f1_ $t1>].encode());
                                        $( bytes.extend([<f2_ $t2>].encode()); )?
                                    )?
                                )?
                            }
                        ),*
                    }
                    bytes
                }
            }
        }
    };
}

pub const OP_HLT: u8 = 0;
pub const OP_LD:  u8 = 1;
pub const OP_ST:  u8 = 2;
pub const OP_ADD: u8 = 3;
pub const OP_SUB: u8 = 4;
pub const OP_MUL: u8 = 5;
pub const OP_DIV: u8 = 6;
pub const OP_NOT: u8 = 7;
pub const OP_AND: u8 = 8;
pub const OP_OR:  u8 = 9;
pub const OP_XOR: u8 = 10;
pub const OP_LSH: u8 = 11;
pub const OP_RSH: u8 = 12;

pub const MODE_REG:  u8 = 0b00;
pub const MODE_IMM:  u8 = 0b01;
pub const MODE_IND:  u8 = 0b10;
pub const MODE_MEM:  u8 = 0b11;
pub const MODE_NONE: u8 = 0b00;

instructions!(
    HLT = (OP_HLT << 2) | MODE_NONE,

    LDrr(u8, u8)   = (OP_LD << 2) | MODE_REG,
    LDrn(u8, u16)  = (OP_LD << 2) | MODE_IMM,
    LDrar(u8, u8)  = (OP_LD << 2) | MODE_IND,
    LDran(u8, u16) = (OP_LD << 2) | MODE_MEM,

    ST(u8, u8) = (OP_ST << 2) | MODE_NONE,

    ADDrr(u8, u8)  = (OP_ADD << 2) | MODE_REG,
    ADDrn(u8, u16) = (OP_ADD << 2) | MODE_IMM,
    SUBrr(u8, u8)  = (OP_SUB << 2) | MODE_REG,
    SUBrn(u8, u16) = (OP_SUB << 2) | MODE_IMM,
    MUL(u8, u8)    = (OP_MUL << 2) | MODE_REG,
    DIV(u8, u8)    = (OP_DIV << 2) | MODE_REG,

    NOT(u8) = (OP_NOT << 2) | MODE_NONE,

    ANDrr(u8, u8)  = (OP_AND << 2) | MODE_REG,
    ANDrn(u8, u16) = (OP_AND << 2) | MODE_IMM,
    ORrr(u8, u8)   = (OP_OR << 2) | MODE_REG,
    ORrn(u8, u16)  = (OP_OR << 2) | MODE_IMM,
    XORrr(u8, u8)  = (OP_XOR << 2) | MODE_REG,
    XORrn(u8, u16) = (OP_XOR << 2) | MODE_IMM,
    LSHrr(u8, u8)  = (OP_LSH << 2) | MODE_REG,
    LSHrn(u8, u16) = (OP_LSH << 2) | MODE_IMM,
    RSHrr(u8, u8)  = (OP_RSH << 2) | MODE_REG,
    RSHrn(u8, u16) = (OP_RSH << 2) | MODE_IMM,
);
