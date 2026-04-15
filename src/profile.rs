use alloc::format;

use io_socket::io::SocketOutput;
use secrecy::SecretString;

use crate::{
    send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult},
    types::profile::Profile,
};

pub type GmailProfileGetResult = GmailSendResult<Profile>;

pub struct GmailProfileGet {
    send: GmailSend<Profile>,
}

impl GmailProfileGet {
    pub fn new(http_auth: &SecretString, user_id: &str) -> Result<Self, GmailSendError> {
        let url = url::Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/profile"))?;
        Ok(Self {
            send: GmailSend::get(http_auth, url),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailProfileGetResult {
        self.send.resume(arg)
    }
}
