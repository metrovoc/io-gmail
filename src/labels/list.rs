use alloc::format;

use io_socket::io::SocketOutput;
use secrecy::SecretString;

use crate::{
    send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult},
    types::label::GmailLabelsListResponse,
};

pub type GmailLabelsListResult = GmailSendResult<GmailLabelsListResponse>;

pub struct GmailLabelsList {
    send: GmailSend<GmailLabelsListResponse>,
}

impl GmailLabelsList {
    pub fn new(http_auth: &SecretString, user_id: &str) -> Result<Self, GmailSendError> {
        let url = url::Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/labels"))?;
        Ok(Self {
            send: GmailSend::get(http_auth, url),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailLabelsListResult {
        self.send.resume(arg)
    }
}
