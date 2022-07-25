use thiserror::Error;

/// Possible errors for Iridium Direct-IP protocol
#[derive(Debug, Error)]
pub enum DirectIPError {
    /// Invalid status for MT::Confirmation::MessageStatus.
    #[error("Invalid MessageStatus: {0}")]
    InvalidMessageStatus(i16),

    /// Undefined error
    #[error("Undefined error")]
    Undefined,
}
