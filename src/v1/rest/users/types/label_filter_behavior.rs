//! Gmail label filter behavior (users.watch).
//!
//! Whether a watch includes or excludes the label ids it lists.

use serde::{Deserialize, Serialize};

/// Whether a watch includes or excludes its label IDs.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailLabelFilterBehavior {
    /// Only changes on the listed labels trigger a notification.
    Include,
    /// Changes on the listed labels never trigger a notification.
    Exclude,
}
