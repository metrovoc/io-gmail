//! Gmail auto-forwarding (users.settings).
//!
//! The automatic forwarding configuration of an account.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::GmailDisposition;

/// Auto-forwarding settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailAutoForwarding {
    /// Whether all incoming mail is automatically forwarded to another
    /// address.
    #[serde(default)]
    pub enabled: bool,
    /// Email address to which all incoming messages are forwarded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Action applied to messages after they have been forwarded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<GmailDisposition>,
}
