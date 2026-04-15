# io-gmail

`no_std` sans-io crate exposing Gmail REST API endpoints as resumable coroutines. Each endpoint is a small state machine with `new()` + `resume()`; callers own the socket, TLS stack, and runtime. Built on top of [`io-http`](https://github.com/pimalaya/io-http); follows the same pattern as [`io-imap`](https://github.com/pimalaya/io-imap), [`io-smtp`](https://github.com/pimalaya/io-smtp), [`io-jmap`](https://github.com/pimalaya/io-jmap), and [`io-oauth`](https://github.com/pimalaya/io-oauth).

The primary motivating use case is **CJK full-text search on Gmail**: IMAP `SEARCH` on Gmail's servers discards non-ASCII characters, while the Gmail REST `q` parameter handles them natively. This crate is the protocol layer of that fix.

## Endpoint coverage

| Module | Coroutine | HTTP | Gmail API |
| --- | --- | --- | --- |
| `profile` | `GmailProfileGet` | GET | `users.getProfile` |
| `labels` | `GmailLabelsList` | GET | `users.labels.list` |
| `labels` | `GmailLabelGet` | GET | `users.labels.get` |
| `labels` | `GmailLabelCreate` | POST | `users.labels.create` |
| `labels` | `GmailLabelUpdate` | PATCH | `users.labels.patch` |
| `labels` | `GmailLabelDelete` | DELETE | `users.labels.delete` |
| `messages` | `GmailMessagesList` | GET | `users.messages.list` (with `q`, `labelIds`, pagination) |
| `messages` | `GmailMessageGet` | GET | `users.messages.get` (`minimal`, `metadata`, `full`, `raw`) |
| `messages` | `GmailMessageSend` | POST | `users.messages.send` (raw RFC 2822, base64url) |
| `messages` | `GmailMessageModify` | POST | `users.messages.modify` (add/remove label ids) |
| `messages` | `GmailMessageTrash` | POST | `users.messages.trash` |
| `messages` | `GmailMessageUntrash` | POST | `users.messages.untrash` |
| `messages` | `GmailMessageDelete` | DELETE | `users.messages.delete` (permanent) |

Not yet covered: threads, drafts, history, attachments (direct), batch, watch, push notifications. Added on demand.

## Architecture

```
caller (himalaya, custom app, …)
   │
   ├── owns Stream (rustls / native-tls / plain / …)
   ├── owns runtime (std threads, tokio, …)
   │
   └── drives io-gmail coroutines
          │
          └── io-http  (request/response wire protocol)
                 │
                 └── io-socket  (byte-level I/O events)
```

Each coroutine yields `GmailSendResult::{Io, Ok, Err}`. The caller reads/writes the bytes requested by `Io` and feeds the result back via `resume(io_output)`. No async runtime, no sockets, no TLS inside this crate.

Error handling: `GmailSendError` distinguishes transport errors, JSON (de)serialisation errors, and typed Gmail API errors (HTTP status + message). `err.status()` returns the HTTP status for API errors; `err.is_retryable()` returns `true` for 429 / 500 / 502 / 503 / 504 so callers can back off.

## Usage

```rust
use io_gmail::messages::list::{GmailMessagesList, GmailMessagesListResult};
use io_gmail::GmailSendResult;
use secrecy::SecretString;

let token = SecretString::from("ya29.…".to_string());
let mut coroutine = GmailMessagesList::new(
    &token,
    "me",                         // user id
    Some("日本語 has:attachment"), // Gmail query (CJK safe)
    &[],                          // label ids
    Some(20),                     // page size
    None,                         // page token
    false,                        // include_spam_trash
)?;

// Caller drives the state machine:
let mut arg = None;
loop {
    match coroutine.resume(arg.take()) {
        GmailSendResult::Io { input }        => { arg = Some(/* call io-socket runtime */); }
        GmailSendResult::Ok { response, .. } => { /* GmailMessagesListResponse */ break; }
        GmailSendResult::Err { err }         => return Err(err),
    }
}
```

A complete driver using `io-socket`'s std-stream runtime lives in the himalaya CLI integration (see [`src/gmail/account.rs`](https://github.com/metrovoc/himalaya/blob/v2-gmail/src/gmail/account.rs) on the `v2-gmail` branch).

## Testing

```bash
cargo test        # 24 tests, no network — stub-stream drivers assert wire behaviour
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Each endpoint test constructs the coroutine, drives it against a scripted stub stream that returns a canned HTTP response, and asserts the coroutine produced the expected `Ok { response }` or `Err { err }`. No real network or OAuth required for CI.

## Cargo

```toml
[dependencies]
io-gmail = { git = "https://github.com/metrovoc/io-gmail" }

[patch.crates-io]
io-http.git = "https://github.com/pimalaya/io-http"
io-socket.git = "https://github.com/pimalaya/io-socket"
```

No `std` feature; the crate is `#![no_std]` + `extern crate alloc`. Depends only on `io-http`, `io-socket`, `serde`, `serde_json`, `base64`, `secrecy`, `url`, `thiserror`, `log`.

## Status

Early — version `0.0.1`, not yet on crates.io. Surface covers the common read/write/search paths of Gmail; end-to-end validated against a real account through the himalaya CLI integration (envelope list / message read / send / trash / untrash / delete / label CRUD / flag add-remove, including CJK queries and non-UTF-8 CJK message bodies).

## License

`MIT OR Apache-2.0` — same as `io-oauth`, `io-jmap`, `io-imap`, `io-smtp`.
