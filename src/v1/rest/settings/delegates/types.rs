//! Gmail delegate resource types.
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.settings.delegates>

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::GmailVerificationStatus;

/// Delegate granted access to a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailDelegate {
    /// Email address of the delegate.
    #[serde(default)]
    pub delegate_email: String,
    /// Verification status of the delegate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<GmailVerificationStatus>,
}
