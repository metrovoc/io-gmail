//! Gmail watch response (users.watch).
//!
//! Result of establishing a push-notification watch on a mailbox.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// Result of establishing a watch (`users.watch`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailWatchResponse {
    /// The id of the current history record of the mailbox.
    #[serde(default)]
    pub history_id: Option<String>,
    /// The expiration time of the watch as epoch milliseconds.
    #[serde(default)]
    pub expiration: Option<String>,
}
