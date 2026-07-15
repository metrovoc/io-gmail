//! Gmail language settings (users.settings).
//!
//! The display language configuration of an account.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// Language display settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailLanguageSettings {
    /// Language to display Gmail in, as an RFC 3066 language tag.
    #[serde(default)]
    pub display_language: String,
}
