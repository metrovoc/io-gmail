//! Gmail history label (users.history).
//!
//! A label change applied to a message in a history record.

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::messages::GmailMessage;

/// A label change on a message in a history record.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailHistoryLabel {
    /// The affected message.
    pub message: GmailMessage,
    /// The ids of the labels added to or removed from the message.
    #[serde(default)]
    pub label_ids: Vec<String>,
}
