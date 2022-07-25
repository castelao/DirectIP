use thiserror::Error;

/// Possible errors for Iridium Direct-IP protocol
#[derive(Debug, Error)]
pub enum DirectIPError {
    /// Undefined error
    #[error("Undefined error")]
    Undefined,
}
