//! Gmail filter (users.settings.filters).
//!
//! A mail filter pairing matching criteria with the action applied to
//! matching messages.

use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::v1::rest::settings::filters::{GmailFilterAction, GmailFilterCriteria};

/// Mail filter of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailFilter {
    /// Server-assigned identifier of the filter.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// Matching criteria of the filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<GmailFilterCriteria>,
    /// Action applied to messages matching the criteria.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<GmailFilterAction>,
}
