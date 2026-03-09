use async_nats::ConnectError;
use std::error::Error as StdError;
use thiserror::Error;

pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, Error)]
pub enum NatsError {
    #[error("NATS connect error")]
    Connect(#[from] ConnectError),

    #[error("JetStream get stream error")]
    GetStream(#[from] async_nats::jetstream::context::GetStreamError),

    #[error("JetStream create stream error")]
    CreateStream(#[from] async_nats::jetstream::context::CreateStreamError),

    #[error("JetStream consumer error")]
    Consumer(#[from] async_nats::jetstream::stream::ConsumerError),

    #[error("JetStream consumer messages stream error")]
    ConsumerStream(#[from] async_nats::jetstream::consumer::StreamError),

    #[error("async-nats error")]
    Nats(#[from] async_nats::Error),

    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Tokio join task error")]
    Join(#[from] tokio::task::JoinError),
}
