use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_traits::ToPrimitive;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoopBegin {
    pub iterations: usize,
}

impl LoopBegin {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}

impl InstructionInfo for LoopBegin {
    fn to_assembly(&self) -> String {
        format!("loop_begin {}", self.iterations)
    }

    fn code() -> InstructionCode {
        InstructionCode::LoopBegin
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_bigint(Self::code(), &self.iterations.into())
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (value, len) = utils::decode_with_bigint(Self::code(), bytes)?;
        let iterations = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Self::new(iterations), len))
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        0
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoopBegin((*self).clone())
    }
}