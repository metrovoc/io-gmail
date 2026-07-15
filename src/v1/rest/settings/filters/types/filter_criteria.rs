//! Gmail filter criteria (users.settings.filters).
//!
//! The conditions a filter matches against incoming messages.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::filters::GmailFilterSizeComparison;

/// Conditions matched by a Gmail filter against incoming messages.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailFilterCriteria {
    /// Sender display name or email address to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Recipient display name or email address to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Case-insensitive phrase matched in the message subject.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Gmail search query the message must match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Gmail search query the message must not match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negated_query: Option<String>,
    /// Whether the message must have an attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_attachment: Option<bool>,
    /// Whether chat messages are excluded from matching.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_chats: Option<bool>,
    /// Message size in bytes compared with `size_comparison`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// How the message size relates to the `size` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_comparison: Option<GmailFilterSizeComparison>,
}
