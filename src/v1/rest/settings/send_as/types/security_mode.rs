//! Gmail security mode (users.settings.sendAs).
//!
//! The transport security used against an SMTP relay service.

use serde::{Deserialize, Serialize};

/// Transport security mode of an SMTP relay service.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailSecurityMode {
    /// Unspecified security mode.
    SecurityModeUnspecified,
    /// Unsecured communication with the SMTP service.
    None,
    /// Communication secured using SSL from connection start.
    Ssl,
    /// Communication upgraded to a secure channel using STARTTLS.
    Starttls,
}
