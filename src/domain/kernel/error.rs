use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("domain error")]
    Generic,
}

