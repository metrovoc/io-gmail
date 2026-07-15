//! Gmail message header (users.messages).
//!
//! A single name-value header of a message part.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// A single header of a Gmail message part.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct GmailMessageHeader {
    /// The name of the header.
    pub name: String,
    /// The value of the header.
    pub value: String,
}
