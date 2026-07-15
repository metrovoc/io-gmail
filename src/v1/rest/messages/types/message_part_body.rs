//! Gmail message part body (users.messages).
//!
//! The body of a single MIME part, inline or pointing at an external
//! attachment.

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// The body of a single MIME part of a Gmail message.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailMessagePartBody {
    /// The id of an external attachment, retrievable via a separate
    /// `users.messages.attachments.get` request.
    #[serde(default)]
    pub attachment_id: Option<String>,
    /// The number of bytes of the message part data.
    #[serde(default)]
    pub size: u32,
    /// The body data as a base64url-encoded string; absent when the
    /// data lives in an external attachment.
    #[serde(default)]
    pub data: Option<String>,
}
