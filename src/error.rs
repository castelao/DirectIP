use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
/// Possible errors for Iridium Direct-IP protocol
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Not the expected Information Element type
    #[error("Wrong IEI for {0}. Expected {1} instead of {2}")]
    WrongIEType(String, u8, u8),

    /// Invalid status for MT::Confirmation::MessageStatus.
    #[error("Invalid MessageStatus: {0}")]
    InvalidMessageStatus(i16),

    /// Undefined error
    #[error("Undefined error")]
    Undefined,
}

pub(crate) type Result<T> = core::result::Result<T, Error>;
