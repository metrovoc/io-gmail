//! Gmail label list visibility (users.labels).
//!
//! Whether and when a label shows up in the label list of the Gmail
//! web client.

use serde::{Deserialize, Serialize};

/// Visibility of the label in the label list of the Gmail web client.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailLabelListVisibility {
    /// The label always shows in the label list.
    LabelShow,
    /// The label shows in the label list only when unread.
    LabelShowIfUnread,
    /// The label never shows in the label list.
    LabelHide,
}
