//! Gmail settings resource types.
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.settings>

use alloc::string::String;

use serde::{Deserialize, Serialize};

/// Vacation auto-reply settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailVacationSettings {
    /// Whether Gmail automatically replies to incoming messages.
    #[serde(default)]
    pub enable_auto_reply: bool,
    /// Optional subject line of the auto-reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_subject: Option<String>,
    /// Response body in plain text format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body_plain_text: Option<String>,
    /// Response body in HTML format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body_html: Option<String>,
    /// Whether responses are only sent to the user's contacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict_to_contacts: Option<bool>,
    /// Whether responses are only sent to users in the same domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrict_to_domain: Option<bool>,
    /// Optional start time for sending auto-replies, in epoch milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Optional end time for sending auto-replies, in epoch milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// IMAP access settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailImapSettings {
    /// Whether IMAP access is enabled for the account.
    #[serde(default)]
    pub enabled: bool,
    /// Whether Gmail immediately expunges messages marked as deleted in
    /// IMAP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_expunge: Option<bool>,
    /// Action applied to messages expunged from the last visible IMAP
    /// folder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expunge_behavior: Option<GmailExpungeBehavior>,
    /// Optional limit on the number of messages an IMAP folder may contain;
    /// zero means no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_folder_size: Option<u32>,
}

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

/// Language display settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailLanguageSettings {
    /// Language to display Gmail in, as an RFC 3066 language tag.
    #[serde(default)]
    pub display_language: String,
}

/// Auto-forwarding settings of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailAutoForwarding {
    /// Whether all incoming mail is automatically forwarded to another
    /// address.
    #[serde(default)]
    pub enabled: bool,
    /// Email address to which all incoming messages are forwarded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Action applied to messages after they have been forwarded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<GmailDisposition>,
}

/// Action applied to a message after it has been forwarded or fetched.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailDisposition {
    DispositionUnspecified,
    LeaveInInbox,
    Archive,
    Trash,
    MarkRead,
}

/// Behavior applied to messages expunged from an IMAP folder.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailExpungeBehavior {
    ExpungeBehaviorUnspecified,
    /// Archive messages marked as deleted.
    Archive,
    /// Move messages marked as deleted to the trash.
    Trash,
    /// Immediately and permanently delete messages marked as deleted.
    DeleteForever,
}

/// Range of messages accessible through POP.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailPopAccessWindow {
    AccessWindowUnspecified,
    /// No messages are accessible via POP.
    Disabled,
    /// Unfetched messages received from now on are accessible via POP.
    FromNowOn,
    /// All unfetched messages are accessible via POP.
    AllMail,
}

/// Verification state of an email address owned by a Gmail account.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailVerificationStatus {
    VerificationStatusUnspecified,
    Accepted,
    Pending,
    Rejected,
    Expired,
}
