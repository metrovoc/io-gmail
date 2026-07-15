//! Gmail labels list response (users.labels.list).
//!
//! The response body carrying every label of the mailbox.

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::v1::rest::labels::GmailLabel;

/// Response body of `users.labels.list`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailLabelsListResponse {
    /// The list of labels.
    #[serde(default)]
    pub labels: Vec<GmailLabel>,
}
