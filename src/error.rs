use std::fmt::Display;

use Error::*;

#[derive(Debug)]
pub enum Error {
    ExpectedReg,
    InvalidReg(char),
    MissingRegIndex,
    MissingArgSeperator,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedReg => write!(f, "Invalid argument, expected register"),
            InvalidReg(c) => write!(f, "Invalid register: register `{c}` does not exist"),
            MissingRegIndex => write!(f, "Missing register index"),
            MissingArgSeperator => write!(f, "Missing argument seperator `,`")
        }
    }
}

impl std::error::Error for Error {}