use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
/// Possible errors for Iridium Direct-IP protocol
pub enum DirectIPError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Invalid status for MT::Confirmation::MessageStatus.
    #[error("Invalid MessageStatus: {0}")]
    InvalidMessageStatus(i16),

    /// Undefined error
    #[error("Undefined error")]
    Undefined,
}

pub(crate) type Result<T> = core::result::Result<T, DirectIPError>;
