//! Gmail thread (users.threads).
//!
//! The full thread resource grouping related messages into a single
//! conversation.

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::messages::GmailMessage;

/// A Gmail thread resource.
///
/// A thread groups related messages into a single conversation.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailThread {
    /// Immutable identifier of the thread.
    pub id: String,
    /// Short part of the message text.
    #[serde(default)]
    pub snippet: Option<String>,
    /// Id of the last history record that modified the thread.
    #[serde(default)]
    pub history_id: Option<String>,
    /// Messages belonging to the thread.
    #[serde(default)]
    pub messages: Vec<GmailMessage>,
}
