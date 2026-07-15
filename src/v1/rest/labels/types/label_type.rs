//! Gmail label type (users.labels).
//!
//! The owner of a label: created by Gmail itself or by the user.

use serde::{Deserialize, Serialize};

/// Owner of the label: created by Gmail or by the user.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailLabelType {
    /// A label created by Gmail, like INBOX or TRASH.
    System,
    /// A label created by the user.
    User,
}
