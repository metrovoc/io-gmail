//! Gmail IMAP settings (users.settings).
//!
//! The IMAP access configuration of an account.

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::GmailExpungeBehavior;

/// IMAP access settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailImapSettings {
    /// Whether IMAP access is enabled for the account.
    #[serde(default)]
    pub enabled: bool,
    /// Whether Gmail immediately expunges messages marked as deleted in
    /// IMAP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_expunge: Option<bool>,
    /// Action applied to messages expunged from the last visible IMAP
    /// folder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expunge_behavior: Option<GmailExpungeBehavior>,
    /// Optional limit on the number of messages an IMAP folder may contain;
    /// zero means no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_folder_size: Option<u32>,
}
