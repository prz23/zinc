//!
//! The project resource GET `swap` response.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The project resource GET `swap` response body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The project compiler version.
    pub amount: u64,
    /// The project data.
    pub count: u64,
    pub fee: u64,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(amount: u64, count:u64, fee:u64) -> Self {
        Self {
            amount,
            count,
            fee
        }
    }
}