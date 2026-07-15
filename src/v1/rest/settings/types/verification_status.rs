//! Gmail verification status (users.settings).
//!
//! The verification state of an address owned by an account.

use serde::{Deserialize, Serialize};

/// Verification state of an email address owned by a Gmail account.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailVerificationStatus {
    /// No verification status specified.
    VerificationStatusUnspecified,
    /// The address is verified and usable.
    Accepted,
    /// The verification request is awaiting a response.
    Pending,
    /// The verification request was rejected.
    Rejected,
    /// The verification request expired without a response.
    Expired,
}
