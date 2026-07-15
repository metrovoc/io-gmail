//! Gmail message id (users.messages).
//!
//! The lightweight message resource carrying only its identifiers.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// A lightweight Gmail message resource carrying only its ids.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailMessageId {
    /// The immutable id of the message.
    pub id: String,
    /// The id of the thread the message belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}
