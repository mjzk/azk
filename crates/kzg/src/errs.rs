use thiserror::Error;

/// Result type
pub type KZGResult<T> = Result<T, KZGError>;
#[derive(Debug, Error)]
pub enum KZGError {
    #[error(transparent)]
    WrappingIOError(#[from] std::io::Error),
    #[error("Deserialization error")]
    DeserializationError,
    #[error("Degree error")]
    DegreeError,
    #[error("PCS prove eval error")]
    PCSProveEvalError,
}
