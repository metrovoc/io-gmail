//! Gmail POP settings (users.settings).
//!
//! The POP access configuration of an account.

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::{GmailDisposition, GmailPopAccessWindow};

/// POP access settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailPopSettings {
    /// Range of messages accessible via POP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_window: Option<GmailPopAccessWindow>,
    /// Action applied to messages after they have been fetched via POP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<GmailDisposition>,
}
