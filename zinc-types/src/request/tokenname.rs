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
    pub tokenname: String,
    pub address: String,

}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(tokenname: String,address: String) -> Self {
        Self { tokenname:tokenname, address:address }
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut result = Vec::with_capacity(2);
        result.push(("tokenname", self.tokenname));
        result.push(("address", self.address));
        result.into_iter()
    }
}