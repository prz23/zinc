//!
//! The generator function statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::generator::bytecode::Bytecode;
use crate::generator::expression::operand::block::Expression;
use crate::generator::r#type::Type;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The Zinc VM function statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    pub location: Location,
    pub identifier: String,
    pub input_arguments: Vec<(String, Type)>,
    pub body: Expression,
    pub output_type: Option<Type>,
    pub type_id: usize,
    pub is_contract_entry: bool,
    pub is_main: bool,
}

impl Statement {
    pub fn new(
        location: Location,
        identifier: String,
        input_arguments: Vec<(String, SemanticType)>,
        body: Expression,
        output_type: SemanticType,
        type_id: usize,
        is_contract_entry: bool,
        is_main: bool,
    ) -> Self {
        let input_arguments = input_arguments
            .into_iter()
            .filter_map(|(name, r#type)| match Type::try_from_semantic(&r#type) {
                Some(r#type) => Some((name, r#type)),
                None => None,
            })
            .collect();

        let output_type = Type::try_from_semantic(&output_type);

        Self {
            location,
            identifier,
            input_arguments,
            body,
            output_type,
            type_id,
            is_contract_entry,
            is_main,
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let output_size = self
            .output_type
            .as_ref()
            .map(|r#type| r#type.size())
            .unwrap_or_default();

        if self.is_main || self.is_contract_entry {
            bytecode.borrow_mut().start_entry_function(
                self.location,
                self.identifier,
                self.type_id,
                self.input_arguments.clone(),
                self.output_type,
            );
        } else {
            bytecode
                .borrow_mut()
                .start_function(self.location, self.type_id, self.identifier);
        }

        for (argument_name, argument_type) in self.input_arguments.into_iter() {
            bytecode
                .borrow_mut()
                .define_variable(Some(argument_name), argument_type.size());
        }

        self.body.write_all_to_bytecode(bytecode.clone());

        bytecode.borrow_mut().push_instruction(
            Instruction::Return(zinc_bytecode::Return::new(output_size)),
            Some(self.location),
        );
    }
}