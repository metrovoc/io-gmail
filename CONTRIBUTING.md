# Contributing guide

Thank you for investing your time in contributing to I/O Gmail.

Whether you are a human or an AI agent, read these in order before touching the code:

1. the [Pimalaya README](https://github.com/pimalaya) for what the project is and how its repositories stack;
2. the [Pimalaya CONTRIBUTING](https://github.com/pimalaya/.github/blob/master/CONTRIBUTING.md) guide, which chains to the shared architecture and guidelines;
3. the inline header documentation, starting with src/lib.rs: it is the architecture document of this crate;
4. the docs/ folder for the development history and living plans.

Everything below documents only what differs from the Pimalaya standards.

## End-to-end test

Besides the offline suite (every coroutine runs against scripted in-memory HTTP responses, no network nor OAuth token required), one ignored end-to-end test drives the full client against the live Gmail API. It needs an OAuth 2.0 access token with read, write and send scopes in the environment:

```sh
GMAIL_ACCESS_TOKEN="<token>" cargo test --test gmail -- --include-ignored
```
