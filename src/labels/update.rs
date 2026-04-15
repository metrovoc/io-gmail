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
struct GmailLabelUpdateRequest<'a> {
    name: &'a str,
    label_list_visibility: &'static str,
    message_list_visibility: &'static str,
}

pub type GmailLabelUpdateResult = GmailSendResult<Label>;

pub struct GmailLabelUpdate {
    send: GmailSend<Label>,
}

impl GmailLabelUpdate {
    pub fn new(
        http_auth: &SecretString,
        user_id: &str,
        id: &str,
        name: &str,
    ) -> Result<Self, GmailSendError> {
        if name.trim().is_empty() {
            return Err(GmailSendError::InvalidRequest(String::from(
                "label name cannot be empty",
            )));
        }

        let url = url::Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/labels/{id}"))?;
        let body = GmailLabelUpdateRequest {
            name,
            label_list_visibility: "labelShow",
            message_list_visibility: "show",
        };

        Ok(Self {
            send: GmailSend::with_method(
                http_auth,
                "PATCH",
                url,
                Some("application/json"),
                serde_json::to_vec(&body).map_err(GmailSendError::SerializeRequest)?,
            ),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailLabelUpdateResult {
        self.send.resume(arg)
    }
}
