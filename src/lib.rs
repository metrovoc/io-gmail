#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod error;
pub mod labels;
pub mod messages;
pub mod profile;
pub mod send;
pub mod types;

pub use error::*;
pub use send::{
    GMAIL_API_BASE, GMAIL_UPLOAD_BASE, GmailSend, GmailSendError, GmailSendResult, NoResponse,
};
