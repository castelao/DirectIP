//! Mobile Terminated - Payload
//!

use super::InformationElementTemplate;
use crate::error::{Error, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use log::debug;

/// Maximum accepted payload length defined by the Direct-IP protocol
const MAX_PAYLOAD_LEN: usize = 1890;

#[derive(Debug)]
/// Mobile Terminated Payload
///
/// Note that length is a 2-bytes and valid range is 1-1890
pub(super) struct Payload {
    payload: Vec<u8>,
}

impl InformationElementTemplate for Payload {
    /// Information Element Identifier
    fn identifier(&self) -> u8 {
        0x42
    }

    fn len(&self) -> u16 {
        self.payload.len().try_into().expect("Payload too large")
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize> {
        if usize::from(self.len()) > MAX_PAYLOAD_LEN {
            debug!("MT-Payload oversized, {} bytes", self.len());
            return Err(Error::Undefined);
        }

        wtr.write_u8(0x42)?;
        wtr.write_u16::<BigEndian>(self.len())?;
        wtr.write_all(&self.payload)?;
        Ok(3 + self.payload.len())
    }
}

impl Payload {
    #[allow(dead_code)]
    fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Payload> {
        let iei = rdr.read_u8()?;
        if iei != 0x42 {
            debug!(
                "Wrong IEI type for MT-Payload. Expected 0x42 instead of {}",
                &iei
            );
            return Err(Error::WrongIEType("MT-Payload".to_string(), 0x42, iei));
        }
        let n = rdr.read_u16::<BigEndian>().unwrap().into();
        if n > MAX_PAYLOAD_LEN {
            debug!("MT-Payload expected to be over-sized: {} bytes", n);
            return Err(Error::Undefined);
        }

        let mut payload = Vec::with_capacity(n);
        rdr.read_exact(&mut payload)?;
        Ok(Payload { payload })
    }
}

#[cfg(test)]
mod test_mt_payload {
    use super::{InformationElementTemplate, Payload};

    #[test]
    fn write() {
        let ie = Payload {
            payload: "Hello World!".into(),
        };
        let v = ie.to_vec();
        assert_eq!(
            v,
            [
                0x42, 0x00, 0x0c, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
                0x21,
            ]
        )
    }
}
