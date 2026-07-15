//! Gmail disposition (users.settings).
//!
//! The action applied to a message once forwarded or fetched via POP.

use serde::{Deserialize, Serialize};

/// Action applied to a message after it has been forwarded or fetched.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailDisposition {
    /// No disposition specified.
    DispositionUnspecified,
    /// Leave the message untouched in the inbox.
    LeaveInInbox,
    /// Archive the message.
    Archive,
    /// Move the message to the trash.
    Trash,
    /// Leave the message in the inbox but mark it as read.
    MarkRead,
}
