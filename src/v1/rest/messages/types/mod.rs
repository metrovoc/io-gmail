//! Gmail message resource types.
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent messages
//! path.

mod internal_date_source;
mod message;
mod message_format;
mod message_header;
mod message_id;
mod message_list_visibility;
mod message_part_body;
mod message_payload;

#[doc(inline)]
pub use internal_date_source::GmailInternalDateSource;
#[doc(inline)]
pub use message::{GmailMessage, decode_raw, encode_raw};
#[doc(inline)]
pub use message_format::GmailMessageFormat;
#[doc(inline)]
pub use message_header::GmailMessageHeader;
#[doc(inline)]
pub use message_id::GmailMessageId;
#[doc(inline)]
pub use message_list_visibility::GmailMessageListVisibility;
#[doc(inline)]
pub use message_part_body::GmailMessagePartBody;
#[doc(inline)]
pub use message_payload::GmailMessagePayload;
