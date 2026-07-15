//! Gmail send-as resource types.
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent send_as
//! path.

mod security_mode;
mod send_as;
mod smtp_msa;

#[doc(inline)]
pub use security_mode::GmailSecurityMode;
#[doc(inline)]
pub use send_as::GmailSendAs;
#[doc(inline)]
pub use smtp_msa::GmailSmtpMsa;
