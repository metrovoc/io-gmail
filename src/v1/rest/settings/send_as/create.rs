//! Create a Gmail send-as alias (`users.settings.sendAs.create`).
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.settings.sendAs/create>

use alloc::format;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use url::Url;

use crate::{
    coroutine::*,
    gmail_try,
    v1::{
        rest::settings::send_as::GmailSendAs,
        send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendOutput},
    },
};

/// I/O-free coroutine creating a send-as alias on a Gmail account
/// (`users.settings.sendAs.create`).
pub struct GmailSendAsCreate {
    send: GmailSend<GmailSendAs>,
}

impl GmailSendAsCreate {
    /// Builds the `users.settings.sendAs.create` request for the given alias.
    pub fn new(
        auth: &HttpAuthBearer,
        user_id: &str,
        send_as: &GmailSendAs,
    ) -> Result<Self, GmailSendError> {
        debug!("prepare gmail send-as alias creation");
        trace!("send_as: {send_as:?}");

        let url = Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/settings/sendAs"))?;
        let send = GmailSend::post_json(auth, url, send_as)?;

        Ok(Self { send })
    }
}

impl GmailCoroutine for GmailSendAsCreate {
    type Yield = GmailYield;
    type Return = Result<GmailSendOutput<GmailSendAs>, GmailSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return> {
        let out = gmail_try!(&mut self.send, arg);
        debug!("send-as alias created");
        trace!("out: {out:?}");
        GmailCoroutineState::Complete(Ok(out))
    }
}
