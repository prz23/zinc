//!
//! ZRust bytecode instruction opcode.
//!

use std::convert::TryFrom;

pub enum OperationCode {
    NoOperation = 0,
    Pop = 1,
    Push = 2,
    Add = 3,
    Subtract = 4,
    Multiply = 5,
    Divide = 6,
    Remainder = 7,
    Negate = 8,
}

impl TryFrom<u8> for OperationCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::NoOperation,
            1 => Self::Pop,
            2 => Self::Push,
            3 => Self::Add,
            4 => Self::Subtract,
            5 => Self::Multiply,
            6 => Self::Divide,
            7 => Self::Remainder,
            8 => Self::Negate,
            value => return Err(value),
        })
    }
}
