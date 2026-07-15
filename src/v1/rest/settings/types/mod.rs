//! Gmail settings resource types.
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent settings
//! path.

mod auto_forwarding;
mod disposition;
mod expunge_behavior;
mod imap_settings;
mod language_settings;
mod pop_access_window;
mod pop_settings;
mod vacation_settings;
mod verification_status;

#[doc(inline)]
pub use auto_forwarding::GmailAutoForwarding;
#[doc(inline)]
pub use disposition::GmailDisposition;
#[doc(inline)]
pub use expunge_behavior::GmailExpungeBehavior;
#[doc(inline)]
pub use imap_settings::GmailImapSettings;
#[doc(inline)]
pub use language_settings::GmailLanguageSettings;
#[doc(inline)]
pub use pop_access_window::GmailPopAccessWindow;
#[doc(inline)]
pub use pop_settings::GmailPopSettings;
#[doc(inline)]
pub use vacation_settings::GmailVacationSettings;
#[doc(inline)]
pub use verification_status::GmailVerificationStatus;
