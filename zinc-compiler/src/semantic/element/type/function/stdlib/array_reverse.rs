//!
//! The semantic analyzer standard library `std::array::reverse` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_build::FunctionIdentifier;

use crate::lexical::token::location::Location;
use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

///
/// The semantic analyzer standard library `std::array::reverse` function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The location where the function is called.
    pub location: Option<Location>,
    /// The unique intrinsic function identifier.
    pub stdlib_identifier: FunctionIdentifier,
    /// The function identifier.
    pub identifier: &'static str,
}

impl Function {
    /// The position of the `array` argument in the function argument list.
    pub const ARGUMENT_INDEX_ARRAY: usize = 0;

    /// The expected number of the function arguments.
    pub const ARGUMENT_COUNT: usize = 1;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(stdlib_identifier: FunctionIdentifier) -> Self {
        Self {
            location: None,
            stdlib_identifier,
            identifier: "reverse",
        }
    }

    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(
        self,
        location: Option<Location>,
        argument_list: ArgumentList,
    ) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let location = element.location();

            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::ArgumentNotEvaluable {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        let return_type = match actual_params.get(Self::ARGUMENT_INDEX_ARRAY) {
            Some((Type::Array(array), _location)) if array.r#type.is_scalar() => {
                Type::array(array.location, array.r#type.deref().to_owned(), array.size)
            }
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "array".to_owned(),
                    position: Self::ARGUMENT_INDEX_ARRAY + 1,
                    expected: "[{scalar}; N]".to_owned(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        };

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount {
                location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT,
                found: actual_params.len(),
                reference: None,
            });
        }

        Ok(return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "array::{}(array: [T; N]) -> [T; N]", self.identifier,)
    }
}
