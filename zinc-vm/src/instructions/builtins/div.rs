extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Div;

impl<E, CS> VMInstruction<E, CS> for Div
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let (div, _rem) = vm.operations().div_rem(left, right)?;

        vm.push(Cell::Value(div))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_div() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped((9).into()))
            .add(PushConst::new_untyped((4).into()))
            .add(Div)
            .add(PushConst::new_untyped((9).into()))
            .add(PushConst::new_untyped((-4).into()))
            .add(Div)
            .add(PushConst::new_untyped((-9).into()))
            .add(PushConst::new_untyped((4).into()))
            .add(Div)
            .add(PushConst::new_untyped((-9).into()))
            .add(PushConst::new_untyped((-4).into()))
            .add(Div)
            .test(&[3, -3, -2, 2])
    }
}