//! List the Gmail delegates (`users.settings.delegates.list`).
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.settings.delegates/list>

use alloc::{format, vec::Vec};

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    coroutine::*,
    gmail_try,
    v1::{
        rest::settings::delegates::GmailDelegate,
        send::{GMAIL_API_BASE, GmailSend, GmailSendError, GmailSendOutput},
    },
};

/// Response wrapping the delegates of a Gmail account.
#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GmailListDelegatesResponse {
    /// Delegates of the Gmail account.
    #[serde(default)]
    pub delegates: Vec<GmailDelegate>,
}

/// I/O-free coroutine listing the delegates of a Gmail account
/// (`users.settings.delegates.list`).
pub struct GmailListDelegates {
    send: GmailSend<GmailListDelegatesResponse>,
}

impl GmailListDelegates {
    /// Builds the `users.settings.delegates.list` request for the given user.
    pub fn new(auth: &HttpAuthBearer, user_id: &str) -> Result<Self, GmailSendError> {
        debug!("prepare gmail delegates listing");
        trace!("user_id: {user_id:?}");

        let url =
            Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/settings/delegates"))?;
        let send = GmailSend::get(auth, url);

        Ok(Self { send })
    }
}

impl GmailCoroutine for GmailListDelegates {
    type Yield = GmailYield;
    type Return = Result<GmailSendOutput<GmailListDelegatesResponse>, GmailSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return> {
        let out = gmail_try!(&mut self.send, arg);
        debug!("gmail delegates listed");
        trace!("out: {out:?}");
        GmailCoroutineState::Complete(Ok(out))
    }
}
