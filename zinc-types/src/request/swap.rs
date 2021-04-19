//!
//! The contract resource `swap` GET request.
//!

use std::iter::IntoIterator;

use serde::Deserialize;

///
/// The contract resource `source` GET request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The pair
    pub name: String,

}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut result = Vec::with_capacity(2);
        result.push(("name", self.name));
        result.into_iter()
    }
}