//! Gmail history message (users.history).
//!
//! Wrapper around a message involved in an addition or deletion
//! history change.

use serde::{Deserialize, Serialize};

use crate::v1::rest::messages::GmailMessage;

/// A message involved in a history addition or deletion change.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailHistoryMessage {
    /// The affected message.
    pub message: GmailMessage,
}
