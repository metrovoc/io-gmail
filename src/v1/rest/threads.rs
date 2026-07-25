//! Gmail threads (`users.threads`): list, get, modify, trash, untrash,
//! delete.
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.threads>

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::messages::GmailMessage;

pub mod delete;
pub mod get;
pub mod list;
pub mod modify;
pub mod trash;
pub mod untrash;

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

/// A Gmail thread summary, as returned by `users.threads.list`.
///
/// A summary carries the thread metadata without its messages; fetch
/// the full [`GmailThread`] with `users.threads.get`.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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
