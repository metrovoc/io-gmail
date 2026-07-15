//! Gmail user-level resource types (profile, watch).
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users>

use alloc::{string::String, vec::Vec};

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

/// Push-notification watch request body (`users.watch`).
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailWatchRequest {
    /// The fully qualified Cloud Pub/Sub topic to publish
    /// notifications to.
    pub topic_name: String,
    /// The label ids to restrict notifications about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label_ids: Vec<String>,
    /// The filtering behavior applied to the label ids.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_filter_behavior: Option<GmailLabelFilterBehavior>,
}

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

/// Whether a watch includes or excludes its label IDs.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailLabelFilterBehavior {
    Include,
    Exclude,
}
