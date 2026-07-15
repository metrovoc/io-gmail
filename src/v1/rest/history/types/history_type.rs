//! Gmail history type (users.history.list).
//!
//! The kind of mailbox change a history list can be filtered on.

use serde::{Deserialize, Serialize};

/// Kind of change to filter the history list on (`historyTypes`).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailHistoryType {
    /// A message was added to the mailbox.
    MessageAdded,
    /// A message was deleted (not trashed) from the mailbox.
    MessageDeleted,
    /// A label was added to a message.
    LabelAdded,
    /// A label was removed from a message.
    LabelRemoved,
}
