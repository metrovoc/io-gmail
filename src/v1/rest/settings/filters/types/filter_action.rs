//! Gmail filter action (users.settings.filters).
//!
//! The action a filter applies to a matching message.

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

/// Action applied by a Gmail filter to a matching message.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailFilterAction {
    /// Identifiers of labels to add to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_label_ids: Option<Vec<String>>,
    /// Identifiers of labels to remove from the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_label_ids: Option<Vec<String>>,
    /// Email address the message is forwarded to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward: Option<String>,
}
