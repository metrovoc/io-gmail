//! Gmail internal date source (users.messages.import, insert).
//!
//! Where the internal date of an imported or inserted message comes
//! from.

use serde::{Deserialize, Serialize};

/// Source of the internal date when importing or inserting a message
/// (`internalDateSource` query parameter).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GmailInternalDateSource {
    /// The internal date is the time the message was received.
    ReceivedTime,
    /// The internal date comes from the Date header of the message.
    DateHeader,
}
