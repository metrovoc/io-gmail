//! Gmail filter resource types.
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent filters
//! path.

mod filter;
mod filter_action;
mod filter_criteria;
mod filter_size_comparison;

#[doc(inline)]
pub use filter::GmailFilter;
#[doc(inline)]
pub use filter_action::GmailFilterAction;
#[doc(inline)]
pub use filter_criteria::GmailFilterCriteria;
#[doc(inline)]
pub use filter_size_comparison::GmailFilterSizeComparison;
