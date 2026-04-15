use alloc::string::String;

use io_socket::io::SocketOutput;
use secrecy::SecretString;
use serde::Serialize;

use crate::{
    send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult},
    types::message::{MessageId, encode_raw},
};

#[derive(Debug, Serialize)]
struct GmailMessageSendRequest {
    raw: String,
}

pub type GmailMessageSendResult = GmailSendResult<MessageId>;

pub struct GmailMessageSend {
    send: GmailSend<MessageId>,
}

impl GmailMessageSend {
    pub fn new(
        http_auth: &SecretString,
        user_id: &str,
        rfc5322: &[u8],
    ) -> Result<Self, GmailSendError> {
        let url = url::Url::parse(GMAIL_API_BASE)?
            .join(&alloc::format!("users/{user_id}/messages/send"))?;
        let body = GmailMessageSendRequest {
            raw: encode_raw(rfc5322),
        };

        Ok(Self {
            send: GmailSend::post_json(http_auth, url, &body)?,
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailMessageSendResult {
        self.send.resume(arg)
    }
}
