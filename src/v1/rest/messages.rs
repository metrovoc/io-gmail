//! Gmail messages (`users.messages`), including attachments
//! (`users.messages.attachments`).
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.messages>

use alloc::{string::String, vec::Vec};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

pub mod attachments;
pub mod batch_delete;
pub mod batch_modify;
pub mod delete;
pub mod get;
pub mod import;
pub mod insert;
pub mod list;
pub mod modify;
pub mod send;
pub mod trash;
pub mod untrash;

/// A Gmail message resource.
///
/// Populated fields depend on the requested [`GmailMessageFormat`]:
/// `payload` comes with the full format, `raw` with the raw format.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GmailMessage {
    /// The immutable id of the message.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// The id of the thread the message belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// The ids of the labels applied to the message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label_ids: Vec<String>,
    /// The internal message creation timestamp (epoch milliseconds),
    /// which determines ordering in the inbox.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_date: Option<String>,
    /// A short part of the message text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    /// The parsed email structure in the message parts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<GmailMessagePayload>,
    /// The entire message as a base64url-encoded RFC 5322 string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    /// The estimated size of the message in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_estimate: Option<u64>,
    /// The id of the last history record that modified the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_id: Option<String>,
}

/// Decodes a base64url-encoded (URL-safe, no padding) raw string back
/// into RFC 5322 message bytes.
///
/// Whitespace and trailing padding are stripped beforehand, so both
/// padded and unpadded inputs decode.
pub fn decode_raw(raw: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let normalized: String = raw.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let normalized = normalized.trim_end_matches('=');
    URL_SAFE_NO_PAD.decode(normalized)
}

/// Encodes raw RFC 5322 message bytes into the base64url (URL-safe,
/// no padding) string expected by the `raw` field.
pub fn encode_raw(raw: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(raw)
}

/// A lightweight Gmail message resource carrying only its ids.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GmailMessageId {
    /// The immutable id of the message.
    pub id: String,
    /// The id of the thread the message belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

/// A single MIME part of a Gmail message.
///
/// The top-level part is exposed as the payload of a [`GmailMessage`];
/// multipart containers nest their children in `parts`.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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

/// The body of a single MIME part of a Gmail message.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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

/// A single header of a Gmail message part.
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GmailMessageHeader {
    /// The name of the header.
    pub name: String,
    /// The value of the header.
    pub value: String,
}

/// Amount of message detail to return (`format` query parameter).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GmailMessageFormat {
    /// Returns only the message id and labels; no headers, body or
    /// payload.
    Minimal,
    /// Returns the full message data, with the body parsed in the
    /// payload field.
    Full,
    /// Returns the full message data as a base64url-encoded string in
    /// the raw field.
    Raw,
    /// Returns only the message id, labels and headers.
    Metadata,
}

/// Whether messages carrying a label show up in the message list.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum GmailMessageListVisibility {
    /// Messages with the label show in the message list.
    Show,
    /// Messages with the label are hidden from the message list.
    Hide,
}

/// Source of the internal date when importing or inserting a message
/// (`internalDateSource` query parameter).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GmailInternalDateSource {
    /// The internal date is the time the message was received.
    ReceivedTime,
    /// The internal date comes from the Date header of the message.
    DateHeader,
}
