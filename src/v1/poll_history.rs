//! Infinite polling watch coroutine built on the history API.
//!
//! Baselines the history cursor via `users.getProfile`, then polls
//! `users.history.list` on a timer (yielding `WantsSleep`) and emits
//! one raw `GmailHistoryDiff` per tick.
//!
//! Gmail sync guide: <https://developers.google.com/gmail/api/guides/sync>

use core::{convert::Infallible, fmt, mem, time::Duration};

use alloc::{string::String, vec::Vec};

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use thiserror::Error;

use crate::{
    coroutine::*,
    v1::rest::history::{
        GmailHistoryLabel,
        list::{GmailListHistory, GmailListHistoryParams},
    },
    v1::rest::messages::{GmailMessage, GmailMessageFormat, GmailMessageId, get::GmailGetMessage},
    v1::rest::users::get_profile::GmailGetProfile,
    v1::send::GmailSendError,
};

const POLL_SECONDS: u64 = 30;

/// Errors that can occur during the watch.
#[derive(Debug, Error)]
pub enum GmailPollHistoryError {
    #[error(transparent)]
    Send(#[from] GmailSendError),
}

/// One tick's worth of mailbox changes, Gmail-native.
///
/// Consumers translate it into their own change representation; io-gmail
/// does not interpret it further.
#[derive(Clone, Debug, Default)]
pub struct GmailHistoryDiff {
    /// The history cursor after this diff, to persist for resuming.
    pub history_id: String,
    /// Messages added to the mailbox since the last tick.
    pub added: Vec<GmailMessage>,
    /// Messages removed from the mailbox since the last tick.
    pub removed: Vec<GmailMessageId>,
    /// Label additions on individual messages.
    pub labels_added: Vec<GmailHistoryLabel>,
    /// Label removals on individual messages.
    pub labels_removed: Vec<GmailHistoryLabel>,
}

/// I/O request or event yielded by the watch.
#[derive(Debug)]
pub enum GmailPollHistoryYield {
    WantsRead,
    WantsWrite(Vec<u8>),
    /// Asks the caller to sleep until the next poll.
    WantsSleep(Duration),
    /// One tick's worth of changes; the watch then goes back to sleep.
    Diff(GmailHistoryDiff),
}

/// I/O-free coroutine watching a mailbox by polling `users.history.list`.
///
/// Never completes successfully (its return type is `Infallible`): it
/// yields one [`GmailHistoryDiff`] per tick and re-baselines itself when
/// the server reports an expired history cursor.
pub struct GmailPollHistory {
    state: State,
    auth: HttpAuthBearer,
    user_id: String,
    mailbox: String,
    history_id: Option<String>,
}

impl GmailPollHistory {
    /// Builds the watch over the given mailbox label, baselining the
    /// history cursor first.
    pub fn new(
        auth: &HttpAuthBearer,
        user_id: &str,
        mailbox: &str,
    ) -> Result<Self, GmailPollHistoryError> {
        debug!("prepare gmail poll history");
        trace!("user_id: {user_id:?}");
        trace!("mailbox: {mailbox:?}");

        let profile = GmailGetProfile::new(auth, user_id)?;
        Ok(Self {
            state: State::Baseline(profile),
            auth: auth.clone(),
            user_id: user_id.into(),
            mailbox: mailbox.into(),
            history_id: None,
        })
    }

    fn list_history(&self, page_token: Option<&str>) -> Result<GmailListHistory, GmailSendError> {
        let params = GmailListHistoryParams {
            start_history_id: self.history_id.as_deref().unwrap_or_default(),
            label_id: Some(&self.mailbox),
            history_types: &[],
            max_results: None,
            page_token,
        };
        GmailListHistory::new(&self.auth, &self.user_id, &params)
    }

    fn get_message(&self, id: &str) -> Result<GmailGetMessage, GmailSendError> {
        GmailGetMessage::new(
            &self.auth,
            &self.user_id,
            id,
            GmailMessageFormat::Metadata,
            &[],
        )
    }

    fn finalize(&mut self, cycle: Cycle) -> GmailHistoryDiff {
        let history_id = cycle
            .new_history_id
            .or_else(|| self.history_id.clone())
            .unwrap_or_default();
        self.history_id = Some(history_id.clone());
        self.state = State::Sleeping;
        GmailHistoryDiff {
            history_id,
            added: cycle.added,
            removed: cycle.removed,
            labels_added: cycle.labels_added,
            labels_removed: cycle.labels_removed,
        }
    }
}

impl GmailCoroutine for GmailPollHistory {
    type Yield = GmailPollHistoryYield;
    type Return = Result<Infallible, GmailPollHistoryError>;

    fn resume(&mut self, bytes: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return> {
        trace!("poll history: {}", self.state);
        let mut bytes = bytes;
        loop {
            match mem::replace(&mut self.state, State::Done) {
                State::Baseline(mut profile) => match profile.resume(bytes.take()) {
                    GmailCoroutineState::Yielded(GmailYield::WantsRead) => {
                        self.state = State::Baseline(profile);
                        return GmailCoroutineState::Yielded(GmailPollHistoryYield::WantsRead);
                    }
                    GmailCoroutineState::Yielded(GmailYield::WantsWrite(out)) => {
                        self.state = State::Baseline(profile);
                        return GmailCoroutineState::Yielded(GmailPollHistoryYield::WantsWrite(
                            out,
                        ));
                    }
                    GmailCoroutineState::Complete(Err(err)) => {
                        return GmailCoroutineState::Complete(Err(err.into()));
                    }
                    GmailCoroutineState::Complete(Ok(out)) => {
                        self.history_id = out.response.history_id;
                        self.state = State::Sleeping;
                    }
                },
                State::Sleeping => {
                    let list = match self.list_history(None) {
                        Ok(list) => list,
                        Err(err) => return GmailCoroutineState::Complete(Err(err.into())),
                    };
                    self.state = State::Listing {
                        list,
                        cycle: Cycle::default(),
                    };
                    return GmailCoroutineState::Yielded(GmailPollHistoryYield::WantsSleep(
                        Duration::from_secs(POLL_SECONDS),
                    ));
                }
                State::Listing {
                    mut list,
                    mut cycle,
                } => match list.resume(bytes.take()) {
                    GmailCoroutineState::Yielded(GmailYield::WantsRead) => {
                        self.state = State::Listing { list, cycle };
                        return GmailCoroutineState::Yielded(GmailPollHistoryYield::WantsRead);
                    }
                    GmailCoroutineState::Yielded(GmailYield::WantsWrite(out)) => {
                        self.state = State::Listing { list, cycle };
                        return GmailCoroutineState::Yielded(GmailPollHistoryYield::WantsWrite(
                            out,
                        ));
                    }
                    GmailCoroutineState::Complete(Err(err)) => {
                        if err.status() == Some(404) {
                            debug!("gmail history cursor expired; re-baselining");
                            let profile = match GmailGetProfile::new(&self.auth, &self.user_id) {
                                Ok(profile) => profile,
                                Err(err) => {
                                    return GmailCoroutineState::Complete(Err(err.into()));
                                }
                            };
                            self.history_id = None;
                            self.state = State::Baseline(profile);
                            continue;
                        }
                        return GmailCoroutineState::Complete(Err(err.into()));
                    }
                    GmailCoroutineState::Complete(Ok(out)) => {
                        let response = out.response;

                        for record in &response.history {
                            for message in &record.messages_added {
                                cycle.added_ids.push(message.message.id.clone());
                            }
                            for message in &record.messages_deleted {
                                cycle.removed.push(GmailMessageId {
                                    id: message.message.id.clone(),
                                    thread_id: message.message.thread_id.clone(),
                                });
                            }
                            for label in &record.labels_added {
                                cycle.labels_added.push(label.clone());
                            }
                            for label in &record.labels_removed {
                                cycle.labels_removed.push(label.clone());
                            }
                        }

                        if let Some(token) = response.next_page_token {
                            let list = match self.list_history(Some(&token)) {
                                Ok(list) => list,
                                Err(err) => {
                                    return GmailCoroutineState::Complete(Err(err.into()));
                                }
                            };
                            self.state = State::Listing { list, cycle };
                            continue;
                        }

                        cycle.new_history_id = response.history_id;

                        if cycle.added_ids.is_empty() {
                            let diff = self.finalize(cycle);
                            return GmailCoroutineState::Yielded(GmailPollHistoryYield::Diff(diff));
                        }

                        let ids = mem::take(&mut cycle.added_ids);
                        let current = match self.get_message(&ids[0]) {
                            Ok(get) => get,
                            Err(err) => return GmailCoroutineState::Complete(Err(err.into())),
                        };
                        self.state = State::Fetching {
                            ids,
                            index: 0,
                            current,
                            cycle,
                        };
                    }
                },
                State::Fetching {
                    ids,
                    index,
                    mut current,
                    mut cycle,
                } => match current.resume(bytes.take()) {
                    GmailCoroutineState::Yielded(GmailYield::WantsRead) => {
                        self.state = State::Fetching {
                            ids,
                            index,
                            current,
                            cycle,
                        };
                        return GmailCoroutineState::Yielded(GmailPollHistoryYield::WantsRead);
                    }
                    GmailCoroutineState::Yielded(GmailYield::WantsWrite(out)) => {
                        self.state = State::Fetching {
                            ids,
                            index,
                            current,
                            cycle,
                        };
                        return GmailCoroutineState::Yielded(GmailPollHistoryYield::WantsWrite(
                            out,
                        ));
                    }
                    GmailCoroutineState::Complete(result) => {
                        match result {
                            Ok(out) => cycle.added.push(out.response),
                            // NOTE: a just-added message may already be gone
                            // by the time we fetch it; skip it rather than
                            // tearing the watch down.
                            Err(err) => trace!("gmail history poll: skipping message get: {err}"),
                        }

                        let index = index + 1;
                        if index < ids.len() {
                            let current = match self.get_message(&ids[index]) {
                                Ok(get) => get,
                                Err(err) => {
                                    return GmailCoroutineState::Complete(Err(err.into()));
                                }
                            };
                            self.state = State::Fetching {
                                ids,
                                index,
                                current,
                                cycle,
                            };
                        } else {
                            let diff = self.finalize(cycle);
                            return GmailCoroutineState::Yielded(GmailPollHistoryYield::Diff(diff));
                        }
                    }
                },
                // SAFETY: every arm reassigns `state` before yielding or
                // continuing, so the watch never rests in `Done`.
                State::Done => unreachable!("gmail watch resumed in terminal state"),
            }
        }
    }
}

#[derive(Default)]
struct Cycle {
    added_ids: Vec<String>,
    added: Vec<GmailMessage>,
    removed: Vec<GmailMessageId>,
    labels_added: Vec<GmailHistoryLabel>,
    labels_removed: Vec<GmailHistoryLabel>,
    new_history_id: Option<String>,
}

enum State {
    Baseline(GmailGetProfile),
    Sleeping,
    Listing {
        list: GmailListHistory,
        cycle: Cycle,
    },
    Fetching {
        ids: Vec<String>,
        index: usize,
        current: GmailGetMessage,
        cycle: Cycle,
    },
    Done,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Baseline(_) => f.write_str("baseline"),
            Self::Sleeping => f.write_str("sleeping"),
            Self::Listing { .. } => f.write_str("listing"),
            Self::Fetching { .. } => f.write_str("fetching"),
            Self::Done => f.write_str("done"),
        }
    }
}
