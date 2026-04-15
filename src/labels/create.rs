use alloc::{format, string::String};

use io_socket::io::SocketOutput;
use secrecy::SecretString;
use serde::Serialize;

use crate::{
    send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult},
    types::label::Label,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GmailLabelCreateRequest<'a> {
    name: &'a str,
    label_list_visibility: &'static str,
    message_list_visibility: &'static str,
}

pub type GmailLabelCreateResult = GmailSendResult<Label>;

pub struct GmailLabelCreate {
    send: GmailSend<Label>,
}

impl GmailLabelCreate {
    pub fn new(
        http_auth: &SecretString,
        user_id: &str,
        name: &str,
    ) -> Result<Self, GmailSendError> {
        if name.trim().is_empty() {
            return Err(GmailSendError::InvalidRequest(String::from(
                "label name cannot be empty",
            )));
        }

        let url = url::Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/labels"))?;
        let body = GmailLabelCreateRequest {
            name,
            label_list_visibility: "labelShow",
            message_list_visibility: "show",
        };

        Ok(Self {
            send: GmailSend::post_json(http_auth, url, &body)?,
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailLabelCreateResult {
        self.send.resume(arg)
    }
}
