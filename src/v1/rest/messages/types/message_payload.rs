//! Gmail message payload (users.messages).
//!
//! A single MIME part of a message, nesting its children for
//! multipart containers.

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::v1::rest::messages::{GmailMessageHeader, GmailMessagePartBody};

/// A single MIME part of a Gmail message.
///
/// The top-level part is exposed as the payload of a
/// [`GmailMessage`](crate::v1::rest::messages::GmailMessage);
/// multipart containers nest their children in `parts`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailMessagePayload {
    /// The immutable id of the message part.
    #[serde(default)]
    pub part_id: Option<String>,
    /// The MIME type of the part.
    #[serde(default)]
    pub mime_type: Option<String>,
    /// The body of the part, which may be empty for container MIME
    /// parts.
    #[serde(default)]
    pub body: Option<GmailMessagePartBody>,
    /// The filename of the attachment; empty when the part is not an
    /// attachment.
    #[serde(default)]
    pub filename: String,
    /// The headers of the part, such as To, From or Subject.
    #[serde(default)]
    pub headers: Vec<GmailMessageHeader>,
    /// The child parts of a container MIME part.
    #[serde(default)]
    pub parts: Vec<GmailMessagePayload>,
}

impl GmailMessagePayload {
    /// Returns the value of the first header matching the given name,
    /// case-insensitively.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case(name))
            .map(|header| header.value.as_str())
    }
}
