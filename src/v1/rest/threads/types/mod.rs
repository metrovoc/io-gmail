//! Gmail thread resource types.
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent threads
//! path.

mod thread;
mod thread_summary;

#[doc(inline)]
pub use thread::GmailThread;
#[doc(inline)]
pub use thread_summary::GmailThreadSummary;
