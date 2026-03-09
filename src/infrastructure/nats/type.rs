use crate::infrastructure::nats::error::NatsError;

pub type Result<T> = std::result::Result<T, NatsError>;
