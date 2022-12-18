//! # Mobile Originated Message
//!
//! ## MO - Information Elements
//!
//! * 0x01 Header IEI
//! * 0x02 Payload IEI
//! * 0x03 Lat/Lon Location Information IEI
//! * 0x05 Confirmation Message IEI
//!
//! ## Example of an MT Message
//!
//! Protocol Revision Number        1   1
//! Overall Message Length          2   31
//! MO Header IEI                   1   0x01
//! MO Header Length                2   28
//! CDR Reference (Auto ID)         4   123456
//! IMEI (User ID)                  15  314159265358979
//! Session Status                  1   0
//! MOMSN                           2   12345
//! MTMSN                           2   54321
//! Time of Session                 4   xxxxxxxx
//! MO Payload IEI                  1   0x02
//! MO Payload Length               2   12
//! MO Payload                      12  "Hello World!"

mod header;
mod payload;

use std::io::Read;

use byteorder::ReadBytesExt;

use crate::error::Error;
use crate::InformationElement;
use header::Header;
use payload::Payload;

#[allow(dead_code)]
#[derive(Debug)]
enum InformationElementType {
    H(Header),
    P(Payload),
}

impl InformationElement for InformationElementType {
    fn identifier(&self) -> u8 {
        match self {
            InformationElementType::H(element) => element.identifier(),
            InformationElementType::P(element) => element.identifier(),
        }
    }

    fn len(&self) -> u16 {
        match self {
            InformationElementType::H(element) => element.len(),
            InformationElementType::P(element) => element.len(),
        }
    }

    /// Total size of Information Element in bytes
    /// This includes the identifier and the field len.
    fn total_size(&self) -> usize {
        3 + usize::from(self.len())
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        match self {
            InformationElementType::H(element) => element.write(wtr),
            InformationElementType::P(element) => element.write(wtr),
        }
    }
}

impl InformationElementType {
    #[allow(dead_code)]
    /// Parse a InformationElementType from a Read trait
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self, Error> {
        let iei = rdr.read_u8()?;
        let buffer = [iei; 1];
        let buffer = buffer.chain(rdr);
        let element = match iei {
            0x01 => {
                todo!();
                // let header = Header::from_reader(buffer).unwrap();
                // InformationElementType::H(header)
            }
            0x02 => {
                let payload = Payload::from_reader(buffer).unwrap();
                InformationElementType::P(payload)
            }
            _ => return Err(Error::Undefined),
        };
        Ok(element)
    }
}

#[derive(Debug)]
pub struct MOMessage {
    elements: Vec<InformationElementType>,
}
