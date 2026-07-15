//! Gmail vacation settings (users.settings).
//!
//! The vacation auto-reply configuration of an account.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// Vacation auto-reply settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailVacationSettings {
    /// Whether Gmail automatically replies to incoming messages.
    #[serde(default)]
    pub enable_auto_reply: bool,
    /// Optional subject line of the auto-reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_subject: Option<String>,
    /// Response body in plain text format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body_plain_text: Option<String>,
    /// Response body in HTML format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body_html: Option<String>,
    /// Whether responses are only sent to the user's contacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict_to_contacts: Option<bool>,
    /// Whether responses are only sent to users in the same domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict_to_domain: Option<bool>,
    /// Optional start time for sending auto-replies, in epoch milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Optional end time for sending auto-replies, in epoch milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
