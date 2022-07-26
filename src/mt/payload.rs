//! Mobile Terminated - Payload
//!

#[derive(Debug)]
/// Mobile Terminated Payload
///
/// Note that length is a 2-bytes and valid range is 1-1890
struct Payload {
    payload: Vec<u8>,
}

impl Payload {
    fn iei(&self) -> u8 {
        0x42
    }

    fn len(&self) -> u16 {
        self.payload.len().try_into().expect("Payload too large")
    }
}
