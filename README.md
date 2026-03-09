### Deckstream

Inspired by faststream.
#### Usage
1) Setup .env file.
2) Run api.
```bash
cargo run --bin api
```
3) Run worker.
```bash
cargo run --bin worker
```
4) Publish some message to a subject in "api.rs" process

---

#### PS
The "api.rs" in this project is the most simple way to get input data - instead you can use anything you want: WEB API, desktop app, CLI etc.

In docker compose file you can find NUI - graphical interface to interact and watch for nats streams, subjects, messages etc. You can find NUI by follow 127.0.0.1:NUI_PORT (from .env).

This project is in progress.
