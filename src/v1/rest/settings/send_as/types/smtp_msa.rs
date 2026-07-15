//! Gmail SMTP MSA (users.settings.sendAs).
//!
//! The SMTP relay configuration attached to a send-as alias.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::send_as::GmailSecurityMode;

/// SMTP relay configuration used to send mail for a send-as alias.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailSmtpMsa {
    /// Hostname of the SMTP service.
    #[serde(default)]
    pub host: String,
    /// Port of the SMTP service.
    #[serde(default)]
    pub port: u32,
    /// Username used for authentication against the SMTP service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password used for authentication against the SMTP service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Transport security used to connect to the SMTP service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_mode: Option<GmailSecurityMode>,
}
