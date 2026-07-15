//! Gmail message list visibility (users.labels).
//!
//! Whether messages carrying a label show up in the message list.

use serde::{Deserialize, Serialize};

/// Whether messages carrying a label show up in the message list.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailMessageListVisibility {
    /// Messages with the label show in the message list.
    Show,
    /// Messages with the label are hidden from the message list.
    Hide,
}
