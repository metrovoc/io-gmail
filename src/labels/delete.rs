use alloc::format;

use io_socket::io::SocketOutput;
use secrecy::SecretString;

use crate::send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult, NoResponse};

pub type GmailLabelDeleteResult = GmailSendResult<NoResponse>;

pub struct GmailLabelDelete {
    send: GmailSend<NoResponse>,
}

impl GmailLabelDelete {
    pub fn new(http_auth: &SecretString, user_id: &str, id: &str) -> Result<Self, GmailSendError> {
        let url = url::Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/labels/{id}"))?;
        Ok(Self {
            send: GmailSend::delete(http_auth, url),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailLabelDeleteResult {
        self.send.resume(arg)
    }
}
