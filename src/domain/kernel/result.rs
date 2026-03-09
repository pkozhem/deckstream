use crate::domain::kernel::error::DomainError;

pub type Result<T> = std::result::Result<T, DomainError>;

