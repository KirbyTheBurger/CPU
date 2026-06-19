macro_rules! pattern {
    ($variant:ident($($field:ty),*)) => {
        Instruction::$variant(..)
    };

    ($variant:ident) => {
        Instruction::$variant
    };
}

macro_rules! instructions {
    (
        $(
            $variant:ident$(($($field:ty),*))? = $opcode:expr
        ),* $(,)?
    ) => {
        #[derive(Debug)]
        pub enum Instruction {
            $($variant$(($($field),*))?),*
        }

        impl Instruction {
            fn opcode(&self) -> u8 {
                match self {
                    $(
                        pattern!($variant$(($($field),*))?) => $opcode
                    ),*
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