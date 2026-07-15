//! Gmail POP access window (users.settings).
//!
//! The range of messages accessible through POP.

use serde::{Deserialize, Serialize};

/// Range of messages accessible through POP.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailPopAccessWindow {
    /// No access window specified.
    AccessWindowUnspecified,
    /// No messages are accessible via POP.
    Disabled,
    /// Unfetched messages received from now on are accessible via POP.
    FromNowOn,
    /// All unfetched messages are accessible via POP.
    AllMail,
}
