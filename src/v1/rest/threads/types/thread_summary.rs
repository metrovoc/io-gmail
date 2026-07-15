//! Gmail thread summary (users.threads.list).
//!
//! Lightweight thread metadata returned by the list method, without
//! the thread messages.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// A Gmail thread summary, as returned by `users.threads.list`.
///
/// A summary carries the thread metadata without its messages; fetch
/// the full [`GmailThread`](crate::v1::rest::threads::GmailThread)
/// with `users.threads.get`.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailThreadSummary {
    /// Immutable identifier of the thread.
    pub id: String,
    /// Short part of the message text.
    #[serde(default)]
    pub snippet: Option<String>,
    /// Id of the last history record that modified the thread.
    #[serde(default)]
    pub history_id: Option<String>,
}
