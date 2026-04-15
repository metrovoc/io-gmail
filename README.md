# io-gmail

`io-gmail` is a `no_std` sans-io crate that exposes Gmail REST API endpoints as resumable coroutines built on top of `io-http`, so callers own the socket, TLS stack, and runtime.

```rust
use io_gmail::messages::list::GmailMessagesList;
use secrecy::SecretString;

let token = SecretString::from("access-token".to_string());
let _coroutine = GmailMessagesList::new(&token, "me", Some("日本語"), &[], Some(20), None, false)?;
```

The intended CLI integration lives in `himalaya/`, and the crate is licensed under `MIT OR Apache-2.0`.
