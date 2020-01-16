use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Cast {
    pub signed: bool,
    pub length: usize,
}

impl Cast {
    pub fn new(signed: bool, length: usize) -> Self {
        Self { signed, length }
    }
}

impl InstructionInfo for Cast {
    fn to_assembly(&self) -> String {
        format!("cast {} {}", self.signed, self.length)
    }

    fn code() -> InstructionCode {
        InstructionCode::Cast
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::Cast as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::Cast((*self).clone())
    }
}
