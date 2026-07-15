//! Gmail filter size comparison (users.settings.filters).
//!
//! How a filter compares the message size with its size criterion.

use serde::{Deserialize, Serialize};

/// Comparison applied to the message size in a filter criterion.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GmailFilterSizeComparison {
    /// No size comparison specified.
    Unspecified,
    /// Matches messages smaller than the given size.
    Smaller,
    /// Matches messages larger than the given size.
    Larger,
}
