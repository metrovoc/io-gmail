//! Gmail history resource types.
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent history
//! path.

mod history;
mod history_label;
mod history_message;
mod history_type;

#[doc(inline)]
pub use history::GmailHistory;
#[doc(inline)]
pub use history_label::GmailHistoryLabel;
#[doc(inline)]
pub use history_message::GmailHistoryMessage;
#[doc(inline)]
pub use history_type::GmailHistoryType;
