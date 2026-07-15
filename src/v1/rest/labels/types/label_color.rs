//! Gmail label color (users.labels).
//!
//! The text and background colors of a user label.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// Text and background colors of a user label, given as hex strings.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailLabelColor {
    /// The text color of the label as a hex string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// The background color of the label as a hex string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}
