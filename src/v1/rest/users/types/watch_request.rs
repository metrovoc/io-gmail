//! Gmail watch request (users.watch).
//!
//! Request body establishing a push-notification watch on a mailbox.

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::users::GmailLabelFilterBehavior;

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
