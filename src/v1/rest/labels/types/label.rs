//! Gmail label (users.labels).
//!
//! The label resource categorizing messages and threads in a mailbox.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::v1::rest::{
    labels::{GmailLabelColor, GmailLabelListVisibility, GmailLabelType},
    messages::GmailMessageListVisibility,
};

/// A Gmail label resource.
///
/// Labels are used to categorize messages and threads within the
/// user's mailbox.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailLabel {
    /// The immutable id of the label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// The display name of the label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The owner type of the label, serialized as `type`.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub label_type: Option<GmailLabelType>,
    /// The visibility of messages with this label in the message list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_list_visibility: Option<GmailMessageListVisibility>,
    /// The visibility of the label in the label list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_list_visibility: Option<GmailLabelListVisibility>,
    /// The total number of messages with the label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_total: Option<u64>,
    /// The number of unread messages with the label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_unread: Option<u64>,
    /// The total number of threads with the label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads_total: Option<u64>,
    /// The number of unread threads with the label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads_unread: Option<u64>,
    /// The color of the label; only available for user labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<GmailLabelColor>,
}
