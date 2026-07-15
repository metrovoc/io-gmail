//! Gmail history record (users.history).
//!
//! One mailbox change: added, deleted and relabelled messages grouped
//! under a single history id.

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::{
    history::{GmailHistoryLabel, GmailHistoryMessage},
    messages::GmailMessage,
};

/// A Gmail history record resource.
///
/// A record captures a change to the mailbox and may affect multiple
/// messages in multiple ways.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailHistory {
    /// The mailbox sequence id of the history record.
    pub id: String,
    /// The messages changed in this history record.
    #[serde(default)]
    pub messages: Vec<GmailMessage>,
    /// The messages added to the mailbox in this history record.
    #[serde(default)]
    pub messages_added: Vec<GmailHistoryMessage>,
    /// The messages deleted (not trashed) from the mailbox in this
    /// history record.
    #[serde(default)]
    pub messages_deleted: Vec<GmailHistoryMessage>,
    /// The labels added to messages in this history record.
    #[serde(default)]
    pub labels_added: Vec<GmailHistoryLabel>,
    /// The labels removed from messages in this history record.
    #[serde(default)]
    pub labels_removed: Vec<GmailHistoryLabel>,
}
