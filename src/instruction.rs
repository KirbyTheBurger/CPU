use paste::paste;

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

macro_rules! pattern {
    ($variant:ident($($field:ty),*)) => {
        Instruction::$variant(..)
    };

    ($variant:ident) => {
        Instruction::$variant
    };
}

macro_rules! encode_fields {
    ($path:path, $bytes:ident) => {
        $path() => {}
    };

    ($path:path, $bytes:ident, $t0:ty) => {
        $path(a) => { $bytes.extend(a.encode()); }
    };

    ($path:path, $bytes:ident, $t0:ty, $t1:ty) => {
        $path(a, b) => {
            $bytes.extend(a.encode());
            $bytes.extend(b.encode());
        }
    };

    ($path:path, $bytes:ident, $t0:ty, $t1:ty, $t2:ty) => {
        $path(a, b, c) => {
            $bytes.extend(a.encode());
            $bytes.extend(b.encode());
            $bytes.extend(c.encode());
        }
    };
}

macro_rules! instructions {
    (
        $(
            $variant:ident $( ( $t0:ty $(, $t1:ty $(, $t2:ty)? )? ) )? = $opcode:expr
        ),* $(,)?
    ) => {
        paste! {
            #[derive(Debug)]
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

instructions!(
    LDrn(u8, u16) = 0x01,
    LDrr(u8, u8) = 0x02,
    LDran(u8, u16) = 0x03,
    LDrar(u8, u8) = 0x04,
);