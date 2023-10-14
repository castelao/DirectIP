//! Direct-IP Protocol
//!

mod error;
pub mod mo;
pub mod mt;

use std::io::{Seek, SeekFrom};

use crate::error::{Error, Result};

trait InformationElement {
    fn identifier(&self) -> u8;

    fn len(&self) -> u16;

    /// Total size of Information Element in bytes
    /// This includes the identifier and the field len.
    fn total_size(&self) -> usize {
        3 + usize::from(self.len())
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize>;

    /// Export Information Element to a vec
    fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        self.write(&mut buffer)
            .expect("Failed to write Information Element to a vec.");
        buffer
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum Message {
    MO(mo::MOMessage),
    MT(mt::MTMessage),
}

// consider enum_dispatch
impl Message {
    /// Identify the type of a Message between MO or MT
    pub fn message_type(&self) -> String {
        match &self {
            Message::MO(_) => "MO".to_string(),
            Message::MT(_) => "MT".to_string(),
        }
    }

    /// Parse a Message from a reader
    pub fn from_reader<R: std::io::Read + Seek>(mut rdr: R) -> Result<Self> {
        match mt::MTMessage::from_reader(&mut rdr) {
            Ok(msg) => Ok(Message::MT(msg)),
            Err(Error::WrongIEType(_, _, _)) => {
                rdr.seek(SeekFrom::Start(0))?;
                let msg = mo::MOMessage::from_reader(rdr)?;
                Ok(Message::MO(msg))
            }
            Err(e) => Err(e),
        }
    }

    /// Extract the IMEI from a Message
    pub fn imei(&self) -> Option<[u8; 15]> {
        match &self {
            Message::MO(m) => m.imei(),
            Message::MT(m) => m.imei(),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match &self {
            Message::MO(m) => m.to_vec(),
            Message::MT(m) => m.to_vec(),
        }
    }
}

pub fn sample() -> Message {
    let msg = mt::MTMessage::from_reader(
        [
            0x01, 0x00, 0x1c, 0x44, 0x00, 0x19, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xf5,
        ]
        .as_slice(),
    );
    Message::MT(msg.unwrap())
}
