//! Mobile Terminated - Payload
//!

#[derive(Debug)]
/// Mobile Terminated Payload
///
/// Note that length is a 2-bytes and valid range is 1-1890
struct Payload {
    payload: Vec<u8>,
}
