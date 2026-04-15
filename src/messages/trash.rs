use alloc::format;

use io_socket::io::SocketOutput;
use secrecy::SecretString;

use crate::{
    send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult},
    types::message::Message,
};

pub type GmailMessageTrashResult = GmailSendResult<Message>;
pub type GmailMessageUntrashResult = GmailSendResult<Message>;

pub struct GmailMessageTrash {
    send: GmailSend<Message>,
}

pub struct GmailMessageUntrash {
    send: GmailSend<Message>,
}

impl GmailMessageTrash {
    pub fn new(http_auth: &SecretString, user_id: &str, id: &str) -> Result<Self, GmailSendError> {
        let url = url::Url::parse(GMAIL_API_BASE)?
            .join(&format!("users/{user_id}/messages/{id}/trash"))?;
        Ok(Self {
            send: GmailSend::with_method(http_auth, "POST", url, None, alloc::vec::Vec::new()),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailMessageTrashResult {
        self.send.resume(arg)
    }
}

impl GmailMessageUntrash {
    pub fn new(http_auth: &SecretString, user_id: &str, id: &str) -> Result<Self, GmailSendError> {
        let url = url::Url::parse(GMAIL_API_BASE)?
            .join(&format!("users/{user_id}/messages/{id}/untrash"))?;
        Ok(Self {
            send: GmailSend::with_method(http_auth, "POST", url, None, alloc::vec::Vec::new()),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailMessageUntrashResult {
        self.send.resume(arg)
    }
}
