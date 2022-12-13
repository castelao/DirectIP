//! Mobile Originated - Payload
//!

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use derive_builder::Builder;
use log::debug;

use crate::error::{Error, Result};
use crate::InformationElementTemplate;

/// Maximum accepted payload length defined by the Direct-IP protocol
const MAX_PAYLOAD_LEN: usize = 1960;
// Some models can have smaller limits.

#[derive(Builder, Debug)]
#[builder(
    pattern = "owned",
    build_fn(error = "crate::error::Error", validate = "Self::validate")
)]
/// Mobile Originated Payload
///
/// Although length is a 2-bytes, the valid range is 1-1960.
pub(super) struct Payload {
    #[builder(setter(into))]
    payload: Vec<u8>,
}

impl InformationElementTemplate for Payload {
    /// Information Element Identifier
    fn identifier(&self) -> u8 {
        0x02
    }

    fn len(&self) -> u16 {
        self.payload.len().try_into().expect("Payload too large")
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize> {
        if usize::from(self.len()) > MAX_PAYLOAD_LEN {
            debug!("MO-Payload oversized, {} bytes", self.len());
            return Err(Error::Undefined);
        }

        wtr.write_u8(0x02)?;
        wtr.write_u16::<BigEndian>(self.len())?;
        wtr.write_all(&self.payload)?;
        Ok(3 + self.payload.len())
    }
}

impl Payload {
    #[allow(dead_code)]
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Payload> {
        let iei = rdr.read_u8()?;
        if iei != 0x02 {
            debug!(
                "Wrong IEI type for MO-Payload. Expected 0x02 instead of {}",
                &iei
            );
            return Err(Error::WrongIEType("MO-Payload".to_string(), 0x02, iei));
        }
        let n = rdr.read_u16::<BigEndian>().unwrap().into();
        if n == 0 {
            Ok(Payload { payload: vec![] })
        } else if n > MAX_PAYLOAD_LEN {
            debug!("MO-Payload expected to be over-sized: {} bytes", n);
            Err(Error::Undefined)
        } else {
            let mut payload = vec![0; n];
            rdr.read_exact(&mut payload)?;
            if payload.len() > n {
                return Err(Error::Undefined);
            }
            Ok(Payload { payload })
        }
    }
    #[allow(dead_code)]
    pub(crate) fn builder() -> PayloadBuilder {
        PayloadBuilder::default()
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
                0x02, 0x00, 0x0c, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
                0x21,
            ]
        )
    }

    #[test]
    // Even if the buffer is longer than required, reads the payload with
    // the correct size.
    // note: buffer size limited by a u8
    fn read_exact() {
        let mut msg = [0u8; 255];
        msg[0] = 0x02;
        for i in 0..252 {
            msg[2] = i;
            let payload = Payload::from_reader(&msg[..]).unwrap();
            assert!(payload.len() == i.into());
            assert!(payload.to_vec().len() - 3 == i.into());
        }
    }
}

impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Payload Element")?;
        write!(f, "  len {}", self.len())?;
        write!(f, "  {:02X?}", self.payload)
    }
}

impl PayloadBuilder {
    fn validate(&self) -> Result<()> {
        if let Some(ref payload) = self.payload {
            if payload.len() > MAX_PAYLOAD_LEN {
                dbg!(&payload);
                return Err(Error::Undefined);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod payload_builder {
    use super::{Error, InformationElementTemplate, PayloadBuilder, MAX_PAYLOAD_LEN};

    #[test]
    /// Build Payload without defining fields
    fn build_default() {
        let payload = PayloadBuilder::default().build();
        match payload {
            Err(Error::UninitializedFieldError(_)) => (),
            _ => panic!(),
        }
    }

    #[test]
    /// Build Payload defining a payload
    /// Note that it implicitly uses into() (payload is a Vec)
    fn build() {
        let payload = PayloadBuilder::default().payload([4, 2]).build().unwrap();
        assert_eq!(payload.to_vec(), [0x02, 0x00, 0x02, 0x04, 0x02]);
    }

    #[test]
    /// The builder should fail with an oversized
    fn build_oversized() {
        let p = [0; (MAX_PAYLOAD_LEN + 1)];
        let e = PayloadBuilder::default().payload(p).build().unwrap_err();
        match e {
            crate::error::Error::Undefined => (),
            _ => panic!(),
        }
    }
}
