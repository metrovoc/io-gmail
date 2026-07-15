//! Delete a Gmail send-as alias (`users.settings.sendAs.delete`).
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.settings.sendAs/delete>

use alloc::format;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use url::Url;

use crate::{
    coroutine::*,
    gmail_try,
    v1::send::{GMAIL_API_BASE, GmailNoResponse, GmailSend, GmailSendError, GmailSendOutput},
};

/// I/O-free coroutine deleting a send-as alias from a Gmail account
/// (`users.settings.sendAs.delete`).
pub struct GmailDeleteSendAs {
    send: GmailSend<GmailNoResponse>,
}

impl GmailDeleteSendAs {
    /// Builds the `users.settings.sendAs.delete` request for the given alias.
    pub fn new(
        auth: &HttpAuthBearer,
        user_id: &str,
        send_as_email: &str,
    ) -> Result<Self, GmailSendError> {
        debug!("prepare gmail send-as alias deletion");
        trace!("send_as_email: {send_as_email:?}");

        let url = Url::parse(GMAIL_API_BASE)?
            .join(&format!("users/{user_id}/settings/sendAs/{send_as_email}"))?;
        let send = GmailSend::delete(auth, url);

        Ok(Self { send })
    }
}

impl GmailCoroutine for GmailDeleteSendAs {
    type Yield = GmailYield;
    type Return = Result<GmailSendOutput<GmailNoResponse>, GmailSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return> {
        let out = gmail_try!(&mut self.send, arg);
        debug!("gmail send-as alias deleted");
        trace!("out: {out:?}");
        GmailCoroutineState::Complete(Ok(out))
    }
}
