//! Gmail label resource types.
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.labels>

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::messages::GmailMessageListVisibility;

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

/// Owner of the label: created by Gmail or by the user.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailLabelType {
    System,
    User,
}

/// Visibility of the label in the label list of the Gmail web client.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailLabelListVisibility {
    LabelShow,
    LabelShowIfUnread,
    LabelHide,
}

/// Text and background colors of a user label, given as hex strings.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailLabelColor {
    /// The text color of the label as a hex string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// The background color of the label as a hex string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}

/// Response body of `users.labels.list`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailListLabelsResponse {
    /// The list of labels.
    #[serde(default)]
    pub labels: Vec<GmailLabel>,
}
