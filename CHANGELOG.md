# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-07-15

### Added

- Added the I/O-free coroutine core for the Gmail REST API v1: the `GmailCoroutine` contract, the shared `GmailSend` HTTP/JSON primitive parsing Gmail's error envelope (redirects are never followed), and a `no_std` query-pair serializer for list parameters.
- Added the full `v1::rest` surface mirroring the Gmail REST reference: users (getProfile, watch, stop), labels, messages (including import, insert, batch operations and attachments), drafts, threads, history, and settings (imap, pop, vacation, language, autoForwarding, delegates, filters, forwardingAddresses, sendAs).
- Added `v1::history_poll::GmailHistoryPoll`, an infinite poll-based mailbox watch composing `users.getProfile` and `users.history.list`, emitting one Gmail-native diff per tick and re-baselining on an expired history cursor.
- Added `GmailClientStd` (`client` feature): a std blocking client with one convenience method per first-class verb, a generic `run` loop for the other coroutines, and a `connect` constructor opening gmail.googleapis.com through pimalaya-stream (`rustls-ring` default, `rustls-aws`, `native-tls`).

[unreleased]: https://github.com/pimalaya/io-gmail/compare/v0.1.0..HEAD
[0.1.0]: https://github.com/pimalaya/io-gmail/compare/root..v0.1.0
