extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::And;

impl<E, O> VMInstruction<E, O> for And
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

        let and = vm.operations().and(left, right)?;

        vm.push(Cell::Value(and))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_and() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(0.into()))
            .add(And)
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(And)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(0.into()))
            .add(And)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(And)
            .test(&[1, 0, 0, 0])
    }
}
