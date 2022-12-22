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

#[derive(Debug)]
pub enum Message {
    MO(mo::MOMessage),
    MT(mt::MTMessage),
}

impl Message {
    pub fn message_type(&self) -> String {
        match &self {
            Message::MO(_) => "MO".to_string(),
            Message::MT(_) => "MT".to_string(),
        }
    }

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
}
