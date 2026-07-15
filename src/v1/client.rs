//! Std-blocking Gmail client, gated behind the `client` feature.
//!
//! Wraps a `Read + Write` stream plus the bearer credential and runs
//! the coroutines against `gmail.googleapis.com`.

#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use core::time::Duration;
use core::{any::Any, fmt};

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use std::io::{self, Read, Write};

use io_http::rfc6750::bearer::HttpAuthBearer;
#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use pimalaya_stream::{std::stream::StreamStd, tls::Tls};
use thiserror::Error;
#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use url::Url;

#[cfg(any(
    feature = "rustls-aws",
    feature = "rustls-ring",
    feature = "native-tls"
))]
use crate::v1::send::GMAIL_API_BASE;
use crate::{
    coroutine::*,
    v1::rest::labels::{
        GmailLabel, GmailListLabelsResponse, create::GmailCreateLabel, delete::GmailDeleteLabel,
        get::GmailGetLabel, list::GmailListLabels, patch::GmailPatchLabel,
        update::GmailUpdateLabel,
    },
    v1::rest::messages::{
        GmailMessage, GmailMessageFormat, GmailMessageId, delete::GmailDeleteMessage,
        get::GmailGetMessage, list::GmailListMessages, list::GmailListMessagesParams,
        list::GmailListMessagesResponse, modify::GmailModifyMessage, send::GmailSendMessage,
        trash::GmailTrashMessage, untrash::GmailUntrashMessage,
    },
    v1::rest::users::{
        GmailProfile, GmailWatchRequest, GmailWatchResponse, get_profile::GmailGetProfile,
        stop::GmailStop, watch::GmailWatch,
    },
    v1::send::{GmailNoResponse, GmailSendError, GmailSendOutput},
};

/// Errors that can occur on the std client.
#[derive(Debug, Error)]
pub enum GmailClientStdError {
    #[error(transparent)]
    Send(#[from] GmailSendError),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    #[error(transparent)]
    Tls(#[from] anyhow::Error),
    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    #[error("Gmail URL `{0}` has no host")]
    UrlMissingHost(String),
    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    #[error("Gmail URL `{url}` has unsupported scheme `{scheme}` (expected `http` or `https`)")]
    UrlUnsupportedScheme { url: String, scheme: String },
}

/// Optional settings for [`GmailClientStd::connect`]; every field has a
/// default (the TLS backend default, and `me` as the mailbox owner).
pub struct GmailClientStdConnectOptions {
    /// TLS backend configuration.
    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    pub tls: Tls,
    /// Owner of the mailbox the requests target (`me` by default).
    pub user_id: String,
}

impl Default for GmailClientStdConnectOptions {
    fn default() -> Self {
        Self {
            #[cfg(any(
                feature = "rustls-aws",
                feature = "rustls-ring",
                feature = "native-tls"
            ))]
            tls: Tls::default(),
            user_id: String::from("me"),
        }
    }
}

const READ_BUFFER_SIZE: usize = 16 * 1024;

/// Standard, blocking Gmail client.
///
/// Owns the stream, the bearer credential and the mailbox owner; each
/// convenience method builds the matching coroutine and runs it to
/// completion. Coroutines without a convenience method go through
/// [`GmailClientStd::run`].
pub struct GmailClientStd {
    /// The underlying TCP or TLS stream.
    pub stream: Box<dyn GmailStream>,
    /// The OAuth 2.0 bearer credential added to every request.
    pub auth: HttpAuthBearer,
    /// Owner of the mailbox the requests target (usually `me`).
    pub user_id: String,
}

impl GmailClientStd {
    /// Builds a client over an already-connected stream.
    pub fn new<S: Read + Write + Send + 'static>(
        stream: S,
        token: impl ToString,
        options: GmailClientStdConnectOptions,
    ) -> Self {
        Self {
            stream: Box::new(stream),
            auth: HttpAuthBearer::new(token.to_string()),
            user_id: options.user_id,
        }
    }

    /// Opens a TCP/TLS connection to `gmail.googleapis.com` and builds
    /// the client around it.
    #[cfg(any(
        feature = "rustls-aws",
        feature = "rustls-ring",
        feature = "native-tls"
    ))]
    pub fn connect(
        token: impl ToString,
        options: GmailClientStdConnectOptions,
    ) -> Result<Self, GmailClientStdError> {
        let GmailClientStdConnectOptions { tls, user_id } = options;

        let url = Url::parse(GMAIL_API_BASE).expect("Gmail API base URL is valid");
        let host = url
            .host_str()
            .ok_or_else(|| GmailClientStdError::UrlMissingHost(url.to_string()))?;

        let stream = match url.scheme() {
            "http" => StreamStd::connect_tcp(host, url.port().unwrap_or(80))?,
            "https" => StreamStd::connect_tls(host, url.port().unwrap_or(443), &tls)?,
            scheme => {
                return Err(GmailClientStdError::UrlUnsupportedScheme {
                    url: url.to_string(),
                    scheme: scheme.to_string(),
                });
            }
        };

        stream.set_read_timeout(Some(Duration::from_secs(30)))?;

        Ok(Self {
            stream: Box::new(stream),
            auth: HttpAuthBearer::new(token.to_string()),
            user_id,
        })
    }

    /// Replaces the underlying stream, e.g. after reconnecting.
    pub fn set_stream<S: Read + Write + Send + 'static>(&mut self, stream: S) {
        self.stream = Box::new(stream);
    }

    /// Runs the given coroutine to completion against the stream,
    /// reading on `WantsRead` and writing on `WantsWrite`.
    pub fn run<C, T>(&mut self, mut coroutine: C) -> Result<GmailSendOutput<T>, GmailClientStdError>
    where
        C: GmailCoroutine<Yield = GmailYield, Return = Result<GmailSendOutput<T>, GmailSendError>>,
    {
        let mut buf = [0u8; READ_BUFFER_SIZE];
        let mut arg: Option<&[u8]> = None;

        loop {
            match coroutine.resume(arg.take()) {
                GmailCoroutineState::Complete(Ok(out)) => return Ok(out),
                GmailCoroutineState::Complete(Err(err)) => return Err(err.into()),
                GmailCoroutineState::Yielded(GmailYield::WantsRead) => {
                    let n = self.stream.read(&mut buf)?;
                    arg = Some(&buf[..n]);
                }
                GmailCoroutineState::Yielded(GmailYield::WantsWrite(bytes)) => {
                    self.stream.write_all(&bytes)?;
                    arg = None;
                }
            }
        }
    }

    /// Gets the profile of the mailbox (`users.getProfile`).
    pub fn get_profile(&mut self) -> Result<GmailSendOutput<GmailProfile>, GmailClientStdError> {
        let coroutine = GmailGetProfile::new(&self.auth, &self.user_id)?;
        self.run(coroutine)
    }

    /// Sets up Pub/Sub push notifications (`users.watch`).
    pub fn watch(
        &mut self,
        request: &GmailWatchRequest,
    ) -> Result<GmailSendOutput<GmailWatchResponse>, GmailClientStdError> {
        let coroutine = GmailWatch::new(&self.auth, &self.user_id, request)?;
        self.run(coroutine)
    }

    /// Stops Pub/Sub push notifications (`users.stop`).
    pub fn stop(&mut self) -> Result<GmailSendOutput<GmailNoResponse>, GmailClientStdError> {
        let coroutine = GmailStop::new(&self.auth, &self.user_id)?;
        self.run(coroutine)
    }

    /// Lists the labels of the mailbox (`users.labels.list`).
    pub fn list_labels(
        &mut self,
    ) -> Result<GmailSendOutput<GmailListLabelsResponse>, GmailClientStdError> {
        let coroutine = GmailListLabels::new(&self.auth, &self.user_id)?;
        self.run(coroutine)
    }

    /// Gets a label by id (`users.labels.get`).
    pub fn get_label(
        &mut self,
        id: &str,
    ) -> Result<GmailSendOutput<GmailLabel>, GmailClientStdError> {
        let coroutine = GmailGetLabel::new(&self.auth, &self.user_id, id)?;
        self.run(coroutine)
    }

    /// Creates the given label (`users.labels.create`).
    pub fn create_label(
        &mut self,
        label: &GmailLabel,
    ) -> Result<GmailSendOutput<GmailLabel>, GmailClientStdError> {
        let coroutine = GmailCreateLabel::new(&self.auth, &self.user_id, label)?;
        self.run(coroutine)
    }

    /// Updates the given label in place (`users.labels.update`).
    pub fn update_label(
        &mut self,
        label: &GmailLabel,
    ) -> Result<GmailSendOutput<GmailLabel>, GmailClientStdError> {
        let coroutine = GmailUpdateLabel::new(&self.auth, &self.user_id, label)?;
        self.run(coroutine)
    }

    /// Patches the given label (`users.labels.patch`).
    pub fn patch_label(
        &mut self,
        label: &GmailLabel,
    ) -> Result<GmailSendOutput<GmailLabel>, GmailClientStdError> {
        let coroutine = GmailPatchLabel::new(&self.auth, &self.user_id, label)?;
        self.run(coroutine)
    }

    /// Deletes a label by id (`users.labels.delete`).
    pub fn delete_label(
        &mut self,
        id: &str,
    ) -> Result<GmailSendOutput<GmailNoResponse>, GmailClientStdError> {
        let coroutine = GmailDeleteLabel::new(&self.auth, &self.user_id, id)?;
        self.run(coroutine)
    }

    /// Lists message ids matching the params (`users.messages.list`).
    pub fn list_messages(
        &mut self,
        params: &GmailListMessagesParams,
    ) -> Result<GmailSendOutput<GmailListMessagesResponse>, GmailClientStdError> {
        let coroutine = GmailListMessages::new(&self.auth, &self.user_id, params)?;
        self.run(coroutine)
    }

    /// Gets a message by id (`users.messages.get`).
    pub fn get_message(
        &mut self,
        id: &str,
        format: GmailMessageFormat,
        metadata_headers: &[&str],
    ) -> Result<GmailSendOutput<GmailMessage>, GmailClientStdError> {
        let coroutine =
            GmailGetMessage::new(&self.auth, &self.user_id, id, format, metadata_headers)?;
        self.run(coroutine)
    }

    /// Sends the given message (`users.messages.send`).
    pub fn send_message(
        &mut self,
        message: &GmailMessage,
    ) -> Result<GmailSendOutput<GmailMessageId>, GmailClientStdError> {
        let coroutine = GmailSendMessage::new(&self.auth, &self.user_id, message)?;
        self.run(coroutine)
    }

    /// Adds and removes labels on a message (`users.messages.modify`).
    pub fn modify_message(
        &mut self,
        id: &str,
        add_label_ids: &[String],
        remove_label_ids: &[String],
    ) -> Result<GmailSendOutput<GmailMessage>, GmailClientStdError> {
        let coroutine = GmailModifyMessage::new(
            &self.auth,
            &self.user_id,
            id,
            add_label_ids,
            remove_label_ids,
        )?;
        self.run(coroutine)
    }

    /// Moves a message to the trash (`users.messages.trash`).
    pub fn trash_message(
        &mut self,
        id: &str,
    ) -> Result<GmailSendOutput<GmailMessage>, GmailClientStdError> {
        let coroutine = GmailTrashMessage::new(&self.auth, &self.user_id, id)?;
        self.run(coroutine)
    }

    /// Restores a message from the trash (`users.messages.untrash`).
    pub fn untrash_message(
        &mut self,
        id: &str,
    ) -> Result<GmailSendOutput<GmailMessage>, GmailClientStdError> {
        let coroutine = GmailUntrashMessage::new(&self.auth, &self.user_id, id)?;
        self.run(coroutine)
    }

    /// Permanently deletes a message (`users.messages.delete`).
    pub fn delete_message(
        &mut self,
        id: &str,
    ) -> Result<GmailSendOutput<GmailNoResponse>, GmailClientStdError> {
        let coroutine = GmailDeleteMessage::new(&self.auth, &self.user_id, id)?;
        self.run(coroutine)
    }
}

impl fmt::Debug for GmailClientStd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GmailClientStd")
            .field("auth", &self.auth)
            .field("user_id", &self.user_id)
            .finish_non_exhaustive()
    }
}

/// Boxable client stream: `Read + Write + Send` plus `Any` so callers
/// can downcast back to the concrete stream type.
pub trait GmailStream: Read + Write + Send + Any {
    /// Returns the stream as a mutable `Any` for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Read + Write + Send + Any> GmailStream for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
