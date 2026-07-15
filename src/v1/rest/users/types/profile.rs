//! Gmail user profile (users.getProfile).
//!
//! Aggregated mailbox counters and the current history id of a user.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// Aggregated mailbox profile of a Gmail user.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailProfile {
    /// The email address of the user.
    pub email_address: String,
    /// The total number of messages in the mailbox.
    #[serde(default)]
    pub messages_total: Option<u64>,
    /// The total number of threads in the mailbox.
    #[serde(default)]
    pub threads_total: Option<u64>,
    /// The id of the current history record of the mailbox.
    #[serde(default)]
    pub history_id: Option<String>,
}
