//! Gmail send-as alias (users.settings.sendAs).
//!
//! An address the account may use in the From header of sent mail.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::{GmailVerificationStatus, send_as::GmailSmtpMsa};

/// Send-as alias of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailSendAs {
    /// Email address that appears in the From header of sent mail.
    #[serde(default)]
    pub send_as_email: String,
    /// Display name used in the From header of sent mail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Email address put in the Reply-To header of sent mail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_address: Option<String>,
    /// HTML signature appended to messages composed with this alias.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Whether this alias is the primary address of the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// Whether this alias is the default From address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Whether Gmail treats this address as an alias of the primary address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub treat_as_alias: Option<bool>,
    /// Optional SMTP service used as outbound relay for this alias.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_msa: Option<GmailSmtpMsa>,
    /// Verification status of the alias.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<GmailVerificationStatus>,
}
