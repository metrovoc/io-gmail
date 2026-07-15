//! Batch-modify Gmail message labels (`users.messages.batchModify`).
//!
//! <https://developers.google.com/gmail/api/reference/rest/v1/users.messages/batchModify>

use alloc::{format, string::String};

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use serde::Serialize;
use url::Url;

use crate::{
    coroutine::*,
    gmail_try,
    v1::send::{GMAIL_API_BASE, GmailNoResponse, GmailSend, GmailSendError, GmailSendOutput},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GmailMessageBatchModifyRequest<'a> {
    ids: &'a [String],
    add_label_ids: &'a [String],
    remove_label_ids: &'a [String],
}

/// Gmail REST batch message label modification, yielding no response body.
pub struct GmailMessagesBatchModify {
    send: GmailSend<GmailNoResponse>,
}

impl GmailMessagesBatchModify {
    /// Builds the `users.messages.batchModify` request adding and
    /// removing the given label ids on the given message ids.
    pub fn new(
        auth: &HttpAuthBearer,
        user_id: &str,
        ids: &[String],
        add_label_ids: &[String],
        remove_label_ids: &[String],
    ) -> Result<Self, GmailSendError> {
        debug!("prepare gmail messages batch modification");
        trace!("ids: {ids:?}");
        trace!("add_label_ids: {add_label_ids:?}");
        trace!("remove_label_ids: {remove_label_ids:?}");

        let url =
            Url::parse(GMAIL_API_BASE)?.join(&format!("users/{user_id}/messages/batchModify"))?;
        let body = GmailMessageBatchModifyRequest {
            ids,
            add_label_ids,
            remove_label_ids,
        };
        let send = GmailSend::post_json(auth, url, &body)?;

        Ok(Self { send })
    }
}

impl GmailCoroutine for GmailMessagesBatchModify {
    type Yield = GmailYield;
    type Return = Result<GmailSendOutput<GmailNoResponse>, GmailSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return> {
        let out = gmail_try!(&mut self.send, arg);
        debug!("messages batch modified");
        trace!("out: {out:?}");
        GmailCoroutineState::Complete(Ok(out))
    }
}
