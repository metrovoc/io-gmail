//! Gmail message format (users.messages.get).
//!
//! The amount of message detail returned by the fetch methods.

use serde::{Deserialize, Serialize};

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
