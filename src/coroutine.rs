//! Coroutine contract shared by every Gmail exchange.
//!
//! Defines the `GmailCoroutine` trait, its `GmailYield` and
//! `GmailCoroutineState` companions, and the `gmail_try!` macro (the
//! coroutine equivalent of `?`).

use alloc::vec::Vec;

/// Progress of a coroutine after a resume: either an I/O request the
/// caller must fulfill, or the terminal value.
#[derive(Debug)]
pub enum GmailCoroutineState<Y, R> {
    /// The coroutine needs I/O before it can go further.
    Yielded(Y),
    /// The coroutine finished with its terminal value.
    Complete(R),
}

/// A resumable, I/O-free Gmail operation.
pub trait GmailCoroutine {
    /// The I/O request type yielded while the exchange is in progress.
    type Yield;
    /// The terminal value type produced when the exchange completes.
    type Return;

    /// Advances the coroutine with the bytes read since the last yield
    /// (`None` when there is nothing to feed).
    fn resume(&mut self, arg: Option<&[u8]>) -> GmailCoroutineState<Self::Yield, Self::Return>;
}

/// The I/O request a coroutine yields: read bytes from the stream, or
/// write the given bytes to it.
#[derive(Debug)]
pub enum GmailYield {
    /// The coroutine wants bytes read from the stream.
    WantsRead,
    /// The coroutine wants the given bytes written to the stream.
    WantsWrite(Vec<u8>),
}

/// Resumes an inner coroutine, forwarding its yields and
/// short-circuiting its errors: the coroutine equivalent of `?`.
#[macro_export]
macro_rules! gmail_try {
    ($coroutine:expr, $arg:expr $(,)?) => {
        match $crate::coroutine::GmailCoroutine::resume($coroutine, $arg) {
            $crate::coroutine::GmailCoroutineState::Yielded(y) => {
                return $crate::coroutine::GmailCoroutineState::Yielded(y.into());
            }
            $crate::coroutine::GmailCoroutineState::Complete(Err(err)) => {
                log::trace!("error during coroutine execution: {err}");
                return $crate::coroutine::GmailCoroutineState::Complete(Err(err.into()));
            }
            $crate::coroutine::GmailCoroutineState::Complete(Ok(value)) => value,
        }
    };
}
