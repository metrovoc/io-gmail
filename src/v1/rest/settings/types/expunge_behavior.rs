//! Gmail expunge behavior (users.settings).
//!
//! The action applied to messages expunged from an IMAP folder.

use serde::{Deserialize, Serialize};

/// Behavior applied to messages expunged from an IMAP folder.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailExpungeBehavior {
    /// No expunge behavior specified.
    ExpungeBehaviorUnspecified,
    /// Archive messages marked as deleted.
    Archive,
    /// Move messages marked as deleted to the trash.
    Trash,
    /// Immediately and permanently delete messages marked as deleted.
    DeleteForever,
}
