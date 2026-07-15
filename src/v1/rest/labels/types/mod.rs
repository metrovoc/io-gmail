//! Gmail label resource types.
//!
//! One module per type: the per-type modules are an internal
//! organization detail, every type flattens into the parent labels
//! path.

mod label;
mod label_color;
mod label_list_visibility;
mod label_type;
mod labels_list_response;

#[doc(inline)]
pub use label::GmailLabel;
#[doc(inline)]
pub use label_color::GmailLabelColor;
#[doc(inline)]
pub use label_list_visibility::GmailLabelListVisibility;
#[doc(inline)]
pub use label_type::GmailLabelType;
#[doc(inline)]
pub use labels_list_response::GmailLabelsListResponse;
