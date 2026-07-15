//! Gmail user-level resource types (profile, watch).
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent users
//! path.

mod label_filter_behavior;
mod profile;
mod watch_request;
mod watch_response;

#[doc(inline)]
pub use label_filter_behavior::GmailLabelFilterBehavior;
#[doc(inline)]
pub use profile::GmailProfile;
#[doc(inline)]
pub use watch_request::GmailWatchRequest;
#[doc(inline)]
pub use watch_response::GmailWatchResponse;
