use alloc::{format, string::String};

use io_socket::io::SocketOutput;
use secrecy::SecretString;
use serde::Serialize;

use crate::{
    send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult},
    types::message::Message,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GmailMessageModifyRequest<'a> {
    add_label_ids: &'a [String],
    remove_label_ids: &'a [String],
}

pub type GmailMessageModifyResult = GmailSendResult<Message>;

pub struct GmailMessageModify {
    send: GmailSend<Message>,
}

impl GmailMessageModify {
    pub fn new(
        http_auth: &SecretString,
        user_id: &str,
        id: &str,
        add_label_ids: &[String],
        remove_label_ids: &[String],
    ) -> Result<Self, GmailSendError> {
        if add_label_ids.is_empty() && remove_label_ids.is_empty() {
            return Err(GmailSendError::InvalidRequest(String::from(
                "modify requires at least one label update",
            )));
        }

        let url = url::Url::parse(GMAIL_API_BASE)?
            .join(&format!("users/{user_id}/messages/{id}/modify"))?;
        let body = GmailMessageModifyRequest {
            add_label_ids,
            remove_label_ids,
        };

        Ok(Self {
            send: GmailSend::post_json(http_auth, url, &body)?,
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailMessageModifyResult {
        self.send.resume(arg)
    }
}
