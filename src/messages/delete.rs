use alloc::format;

use io_socket::io::SocketOutput;
use secrecy::SecretString;

use crate::send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult, NoResponse};

pub type GmailMessageDeleteResult = GmailSendResult<NoResponse>;

pub struct GmailMessageDelete {
    send: GmailSend<NoResponse>,
}

impl GmailMessageDelete {
    pub fn new(http_auth: &SecretString, user_id: &str, id: &str) -> Result<Self, GmailSendError> {
        let url =
            url::Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/messages/{id}"))?;
        Ok(Self {
            send: GmailSend::delete(http_auth, url),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailMessageDeleteResult {
        self.send.resume(arg)
    }
}
