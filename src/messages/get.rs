use alloc::format;

use io_socket::io::SocketOutput;
use secrecy::SecretString;

use crate::{
    send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendResult},
    types::message::{Message, MessageFormat},
};

pub type GmailMessageGetResult = GmailSendResult<Message>;

pub struct GmailMessageGet {
    send: GmailSend<Message>,
}

impl GmailMessageGet {
    pub fn new(
        http_auth: &SecretString,
        user_id: &str,
        id: &str,
        format: MessageFormat,
        metadata_headers: &[&str],
    ) -> Result<Self, GmailSendError> {
        let mut url =
            url::Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/messages/{id}"))?;

        {
            let mut query = url.query_pairs_mut();
            query.append_pair("format", format.as_str());

            if matches!(format, MessageFormat::Metadata) {
                for header in metadata_headers {
                    query.append_pair("metadataHeaders", header);
                }
            }
        }

        Ok(Self {
            send: GmailSend::get(http_auth, url),
        })
    }

    pub fn resume(&mut self, arg: Option<SocketOutput>) -> GmailMessageGetResult {
        self.send.resume(arg)
    }
}
