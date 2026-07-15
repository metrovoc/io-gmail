//! Update the Gmail auto-forwarding settings
//! (`users.settings.updateAutoForwarding`).
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.settings/updateAutoForwarding>

use alloc::format;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use url::Url;

use crate::{
    coroutine::*,
    gmail_try,
    v1::{
        rest::settings::GmailAutoForwarding,
        send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendOutput},
    },
};

/// I/O-free coroutine updating the Gmail auto-forwarding settings
/// (`users.settings.updateAutoForwarding`).
pub struct GmailUpdateAutoForwarding {
    send: GmailSend<GmailAutoForwarding>,
}

impl GmailUpdateAutoForwarding {
    /// Builds the `users.settings.updateAutoForwarding` request wrapping the
    /// given settings.
    pub fn new(
        auth: &HttpAuthBearer,
        user_id: &str,
        settings: GmailAutoForwarding,
    ) -> Result<Self, GmailSendError> {
        debug!("prepare gmail auto-forwarding settings update");
        trace!("settings: {settings:?}");

        let url = Url::parse(GMAIL_API_BASE)?
            .join(&format!("users/{user_id}/settings/autoForwarding"))?;
        let send = GmailSend::put_json(auth, url, &settings)?;

        Ok(Self { send })
    }
}

impl GmailCoroutine for GmailUpdateAutoForwarding {
    type Yield = GmailYield;
    type Return = Result<GmailSendOutput<GmailAutoForwarding>, GmailSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return> {
        let out = gmail_try!(&mut self.send, arg);
        debug!("gmail auto-forwarding settings updated");
        trace!("out: {out:?}");
        GmailCoroutineState::Complete(Ok(out))
    }
}
