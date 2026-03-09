### Deckstream

Inspired by FastStream.

#### Usage

1) Start Docker Compose:
    ```bash
    docker compose up -d
    ```
2) Set up the `.env` file (from `.env.example`).
3) Run the API:
    ```bash
    cargo run --bin api
    ```
4) Run the worker:
    ```bash
    cargo run --bin worker
    ```
5) Publish a message to a subject via the `api` process.

---

#### Note

`api.rs` in this project is the simplest way to feed input data — you can replace it with anything you want: a web API, desktop app, CLI, etc.

In the Docker Compose file you can find NUI — a graphical interface for interacting with and monitoring NATS streams, subjects, messages, etc. Access NUI at `127.0.0.1:NUI_PORT` (see `.env`). How to connect - in HOST section input `nats://nats:NATS_CLIENT_PORT`.

This project is in progress.
