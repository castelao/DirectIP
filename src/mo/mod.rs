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
mod location;
mod payload;

use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::error::{Error, Result};
use crate::InformationElement;
use header::Header;
use location::Location;
use payload::Payload;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum InformationElementType {
    H(Header),
    L(Location),
    P(Payload),
}

impl InformationElement for InformationElementType {
    fn identifier(&self) -> u8 {
        match self {
            InformationElementType::H(element) => element.identifier(),
            InformationElementType::L(element) => element.identifier(),
            InformationElementType::P(element) => element.identifier(),
        }
    }

    fn len(&self) -> u16 {
        match self {
            InformationElementType::H(element) => element.len(),
            InformationElementType::L(element) => element.len(),
            InformationElementType::P(element) => element.len(),
        }
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize> {
        match self {
            InformationElementType::H(element) => element.write(wtr),
            InformationElementType::L(element) => element.write(wtr),
            InformationElementType::P(element) => element.write(wtr),
        }
    }
}

impl InformationElementType {
    /// Parse a InformationElementType from a Read trait
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self> {
        let iei = rdr.read_u8()?;
        let buffer = [iei; 1];
        let buffer = buffer.chain(rdr);
        let element = match iei {
            0x01 => {
                let header = Header::from_reader(buffer).unwrap();
                InformationElementType::H(header)
            }
            0x02 => {
                let payload = Payload::from_reader(buffer).unwrap();
                InformationElementType::P(payload)
            }
            0x03 => {
                let location = Location::from_reader(buffer).unwrap();
                tracing::debug!("Parsed an MO::Location element");
                InformationElementType::L(location)
            }
            _ => {
                tracing::debug!("Not a valid MO IEI ({})", &iei);
                return Err(Error::Undefined);
            }
        };
        Ok(element)
    }
}

impl From<Header> for InformationElementType {
    fn from(header: Header) -> Self {
        InformationElementType::H(header)
    }
}

impl From<Payload> for InformationElementType {
    fn from(payload: Payload) -> Self {
        InformationElementType::P(payload)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct MOMessage {
    elements: Vec<InformationElementType>,
}

impl MOMessage {
    #[allow(dead_code)]
    fn new() -> MOMessage {
        MOMessage {
            elements: Vec::new(),
        }
    }

    /// Overall Message Length
    fn len(&self) -> u16 {
        self.elements.iter().map(|e| 3 + e.len()).sum::<u16>()
    }

    #[allow(dead_code)]
    fn total_size(&self) -> usize {
        3 + self.len() as usize
    }

    // Write the full message
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize> {
        // Protocol version
        wtr.write_u8(1)?;
        // Message total length
        wtr.write_u16::<BigEndian>(self.len())?;
        // Iterate on all Information Elements
        let mut n = 3;
        for e in &self.elements {
            n += e.write(wtr)?;
        }
        Ok(n)
    }

    /// Export MT-Message into a vector of u8
    pub fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        self.write(&mut buffer)
            .expect("Failed to write Information Element to a vec.");
        buffer
    }
    /// Parse bytes from a buffer to compose an MTMessage
    pub fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self> {
        // Protocol version
        let version = rdr.read_u8()?;
        // Expects version 1
        assert_eq!(version, 1);
        // Message total length
        let length = rdr.read_u16::<BigEndian>().expect("Failed to read length") as usize;

        let mut msg = MOMessage { elements: vec![] };
        let mut n = 0;
        while n < length {
            let element = InformationElementType::from_reader(&mut rdr)?;
            n += element.total_size();
            msg.push(element);
        }

        Ok(msg)
    }

    /// Appends an element to the back of an MT-Message
    ///
    /// This should be a good place to check for duplicates, i.e. try to
    /// insert a second header for instance.
    fn push(&mut self, element: InformationElementType) {
        self.elements.push(element);
    }

    fn header(&self) -> Option<&Header> {
        self.elements
            .iter()
            .find(|elem| matches!(elem, InformationElementType::H(_)))
            .map(|e| {
                if let InformationElementType::H(h) = e {
                    h
                } else {
                    unreachable!()
                }
            })
    }

    pub fn imei(&self) -> Option<[u8; 15]> {
        self.header().map(|h| h.imei())
    }
}

#[cfg(all(test, feature = "serde"))]
mod test_mo_information_element_serde {
    use super::{header::SessionStatus, Header, InformationElementType, Payload};
    use chrono::{DateTime, Utc};

    #[test]
    fn payload_roundtrip() {
        let mut payload = vec![0u8; 10];
        payload[0] = 0x42;
        let p = Payload::builder().payload(payload).build().unwrap();
        let ie: InformationElementType = p.into();
        let json = serde_json::to_string(&ie).unwrap();

        let roundtrip: InformationElementType = serde_json::from_str(&json).unwrap();

        assert_eq!(ie, roundtrip);
    }

    #[test]
    fn header_roundtrip() {
        let header = Header::builder()
            .cdr_uid(9999)
            .session_status(SessionStatus::Success)
            .momsn(16)
            .mtmsn(18)
            .imei([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4])
            .time_of_session("2000-03-14T12:12:12Z".parse::<DateTime<Utc>>().unwrap())
            .build()
            .unwrap();
        let ie: InformationElementType = header.into();
        let json = serde_json::to_string(&ie).unwrap();

        let roundtrip: InformationElementType = serde_json::from_str(&json).unwrap();

        assert_eq!(ie, roundtrip);
    }
}
