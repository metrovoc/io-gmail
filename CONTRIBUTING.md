# Contributing guide

Thank you for investing your time in contributing to I/O Gmail.

Whether you are a human or an AI agent, read these in order before touching the code:

1. the [Pimalaya README](https://github.com/pimalaya) for what the project is and how its repositories stack;
2. the [Pimalaya ARCHITECTURE](https://github.com/pimalaya/.github/blob/master/ARCHITECTURE.md) for the conventions every repository shares (layering, `no_std`, modules, errors, code style, licensing, notes for AI agents);
3. the inline header documentation, starting with [src/lib.rs](src/lib.rs), for how this crate is architectured: its `lib.rs` *is* the architecture document (the `v1` layout, the shared send primitive, the coroutine contract and naming, the poll-based watch);
4. this guide, for how to build, test and submit changes here.

## Development environment

The environment is managed by [Nix](https://nixos.org/download.html). `nix develop` spawns a shell with the right toolchain; every cargo command below assumes it (or prefix them with `nix develop --command`).

Without Nix, install a recent stable toolchain via [rustup](https://rust-lang.github.io/rustup/) (`rustup update`); the crate needs Rust matching the `rust-version` in [Cargo.toml](./Cargo.toml).

## Build

I/O Gmail is a `#![no_std]` library (with `alloc`) built on [io-http](https://github.com/pimalaya/io-http), exposing three feature-gated layers:

- the I/O-free coroutines: no feature required, `no_std`, no sockets nor async runtime;
- the light client (`client` feature): a std-blocking `GmailClientStd` wrapping any `Read + Write` stream you opened yourself;
- the full client (`rustls-ring` (default), `rustls-aws` or `native-tls`): opens the TCP/TLS connection itself via [pimalaya/stream](https://github.com/pimalaya/stream).

Check every layer, since gated code (`client`, `std`, TLS) must not leak into the always-on coroutine core:

```sh
cargo build --no-default-features                    # coroutines only, no std leak
cargo build --no-default-features --features client  # light client, no TLS deps
cargo build --release                                # full client (rustls-ring)
```

When touching feature gates or imports, build with and without each feature so no gated code leaks into the core.

## Lint, test, audit

```sh
cargo test --all-features                    # offline + doc tests
cargo clippy --all-targets --all-features
cargo fmt                                    # CI checks `cargo fmt --check`
cargo deny check                             # advisories, licenses, bans, sources
```

The offline test suite runs every coroutine against scripted in-memory HTTP responses, so no network access or OAuth token is required. The end-to-end test against the live API is opt-in:

```sh
GMAIL_ACCESS_TOKEN="<token>" cargo test --test gmail -- --include-ignored
```

## Override dependencies

All Pimalaya crates use `[patch.crates-io]` to point to sibling directories. To build I/O Gmail against a locally modified dependency (e.g. `io-http`), add to [Cargo.toml](./Cargo.toml):

```toml
[patch.crates-io]
io-http.path = "/path/to/io-http"
```

## Commit style

I/O Gmail follows the [conventional commits specification](https://www.conventionalcommits.org/en/v1.0.0/#summary). Keep the subject imperative and scoped; describe the *why* in the body when it is not obvious.
