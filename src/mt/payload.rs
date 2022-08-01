//! Mobile Terminated - Payload
//!

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
/// Mobile Terminated Payload
///
/// Note that length is a 2-bytes and valid range is 1-1890
struct Payload {
    payload: Vec<u8>,
}

impl Payload {
    /// Information Element Identifier
    fn identifier(&self) -> u8 {
        0x42
    }

    fn len(&self) -> u16 {
        self.payload.len().try_into().expect("Payload too large")
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        wtr.write_u8(0x42)?;
        wtr.write_u16::<BigEndian>(self.len())?;
        wtr.write_all(&self.payload)?;
        Ok(3 + self.payload.len())
    }
}
