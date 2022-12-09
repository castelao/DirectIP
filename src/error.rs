use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
/// Possible errors for Iridium Direct-IP protocol
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    UninitializedFieldError(#[from] derive_builder::UninitializedFieldError),

    /// Not the expected Information Element type
    #[error("Wrong IEI for {0}. Expected {1} instead of {2}")]
    WrongIEType(String, u8, u8),

    /// Invalid status for MT::Confirmation::MessageStatus.
    #[error("Invalid MessageStatus: {0}")]
    InvalidMessageStatus(i16),

    /// Invalid status for MO::Header::SeessionStatus.
    #[error("Invalid SessionStatus: {0}")]
    InvalidSessionStatus(u8),

    /// Undefined error
    #[error("Undefined error")]
    Undefined,
}

pub(crate) type Result<T> = core::result::Result<T, Error>;
