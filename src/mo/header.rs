//! Mobile Originated Header
//!
//! A mobile originated header is one of the information element types that
//! compose a mobile originated message. It is defined by an information
//! element identifier (IEI) with value 0x01.

#[derive(Debug)]
/// Session Status
///
/// Status:
/// * 0: Success
///
enum SessionStatus {
    Success,
    MTTooLarge,
    BadLocation,
    Timout,
    MOTooLarge,
    RFLoss,
    SSDAnomaly,
    SSDProhibited,
}
