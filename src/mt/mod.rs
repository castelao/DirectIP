//! # Mobile Terminated Message
//!
//! ## MT - Information Elements
//!
//! * 0x41 Header IEI
//! * 0x42 Payload IEI
//! * 0x43 Lat/Lon Location Information IEI
//! * 0x44 Confirmation Message IEI
//! * 0x45 LAC/Cell ID Location Informatio IEI
//!
//! ## Example of an MT Message
//!
//! Protocol Revision Number        1   1
//! Overall Message Length          2   39
//! MT Header IEI                   1   0x41
//! MT Header Length                2   21
//! Unique Client Message ID        4   "Msg7"
//! IMEI (User ID)                  15  314159265358979
//! MT Disposition Flags            2   0x0000
//! MT Payload IEI                  1   0x42
//! MT Payload Length               2   12
//! MT Payload                      12  "Hello World!"

mod confirmation;
mod header;
mod payload;

use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::error::Error;
use crate::InformationElement;
use confirmation::Confirmation;
use header::{Header, HeaderBuilder};
use payload::{Payload, PayloadBuilder};

#[derive(Debug)]
enum InformationElementType {
    H(Header),
    P(Payload),
    C(Confirmation),
}

impl InformationElement for InformationElementType {
    fn identifier(&self) -> u8 {
        match self {
            InformationElementType::H(element) => element.identifier(),
            InformationElementType::P(element) => element.identifier(),
            InformationElementType::C(element) => element.identifier(),
        }
    }

    fn len(&self) -> u16 {
        match self {
            InformationElementType::H(element) => element.len(),
            InformationElementType::P(element) => element.len(),
            InformationElementType::C(element) => element.len(),
        }
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        match self {
            InformationElementType::H(element) => element.write(wtr),
            InformationElementType::P(element) => element.write(wtr),
            InformationElementType::C(element) => element.write(wtr),
        }
    }
}

impl InformationElementType {
    /// Parse a InformationElementType from a Read trait
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self, Error> {
        let iei = rdr.read_u8()?;
        let buffer = [iei; 1];
        let buffer = buffer.chain(rdr);
        let element = match iei {
            0x41 => {
                let header = Header::from_reader(buffer).unwrap();
                InformationElementType::H(header)
            }
            0x42 => {
                let payload = Payload::from_reader(buffer).unwrap();
                InformationElementType::P(payload)
            }
            0x44 => {
                let confirmation = Confirmation::from_reader(buffer).unwrap();
                InformationElementType::C(confirmation)
            }
            _ => return Err(Error::Undefined),
        };
        Ok(element)
    }
}

#[cfg(test)]
mod test_mt_information_element {
    use crate::mt::InformationElementType;

    #[test]
    // Need improvements to properly test it
    fn read() {
        let msg = [
            0x41, 0x00, 0x15, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x00, 0x3b,
        ]
        .as_slice();
        let _ie = InformationElementType::from_reader(msg).unwrap();
    }
}

impl From<Confirmation> for InformationElementType {
    fn from(confirmation: Confirmation) -> Self {
        InformationElementType::C(confirmation)
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

#[cfg(test)]
mod test_mt_information_element_from {
    use crate::mt::{Header, InformationElement, InformationElementType, Payload};

    #[test]
    fn header() {
        let header = Header::builder()
            .client_msg_id(9999)
            .imei([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4])
            .build()
            .unwrap();
        let ie = InformationElementType::from(header);
        assert!(ie.identifier() == 0x41);
    }

    #[test]
    fn payload() {
        let payload = Payload::builder().payload("Hey, it's me!").build().unwrap();
        let ie = InformationElementType::from(payload);
        assert!(ie.identifier() == 0x42);
    }
}

#[derive(Debug)]
pub struct MTMessage {
    elements: Vec<InformationElementType>,
}

// Let's allow dead while still WIP
#[allow(dead_code)]
impl MTMessage {
    fn new() -> MTMessage {
        MTMessage {
            elements: Vec::new(),
        }
    }

    /// Overall Message Length
    fn len(&self) -> u16 {
        self.elements
            .iter()
            .map(|e| e.total_size())
            .sum::<usize>()
            .try_into()
            .unwrap()
    }

    fn total_size(&self) -> usize {
        3 + self.elements.iter().map(|e| e.total_size()).sum::<usize>()
    }

    // Write the full message
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
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
    pub fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self, Error> {
        // Protocol version
        let version = rdr.read_u8()?;
        // Expects version 1
        assert_eq!(version, 1);
        // Message total length
        let length = rdr.read_u16::<BigEndian>().expect("Failed to read length") as usize;

        let mut msg = Self::new();
        let mut n = 0;
        while n < length {
            let element = InformationElementType::from_reader(&mut rdr)?;
            n += element.total_size();
            msg.push(element);
        }

        Ok(msg)
    }

    pub fn builder() -> MTMessageBuilder {
        MTMessageBuilder::default()
    }

    /// Appends an element to the back of an MT-Message
    ///
    /// This should be a good place to check for duplicates, i.e. try to insert
    /// a second header for instance.
    fn push(&mut self, element: InformationElementType) {
        self.elements.push(element);
    }

    fn confirmation(&self) -> Option<&Confirmation> {
        self.elements
            .iter()
            .find(|elem| matches!(elem, InformationElementType::C(_)))
            .map(|e| {
                if let InformationElementType::C(c) = e {
                    c
                } else {
                    unreachable!()
                }
            })
    }

    pub fn confirmation_message(&self) -> Option<String> {
        self.confirmation().map(|v| v.message_status().to_string())
    }
}

#[cfg(test)]
mod test_mt_message {
    use super::MTMessage;

    #[test]
    fn to_vec() {}

    #[test]
    // Could improve this test with some checks on the output of from_reader()
    fn confirmation_from_reader() {
        let buffer = [
            0x01, 0x00, 0x1c, 0x44, 0x00, 0x19, 0x00, 0x00, 0x04, 0x57, 0x00, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x2a,
        ];

        MTMessage::from_reader(&buffer[..]).unwrap();
    }
}

pub struct MTMessageBuilder {
    header: HeaderBuilder,
    payload: PayloadBuilder,
}

impl MTMessageBuilder {
    fn default() -> MTMessageBuilder {
        MTMessageBuilder {
            header: HeaderBuilder::default(),
            payload: PayloadBuilder::default(),
        }
    }

    pub fn client_msg_id(mut self, client_msg_id: u32) -> Self {
        self.header = self.header.client_msg_id(client_msg_id);
        self
    }

    pub fn imei(mut self, imei: [u8; 15]) -> Self {
        self.header = self.header.imei(imei);
        self
    }

    pub fn payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = self.payload.payload(payload);
        self
    }

    pub fn build(self) -> MTMessage {
        let mut msg = MTMessage::new();
        msg.push(self.header.build().unwrap().into());
        msg.push(self.payload.build().unwrap().into());
        msg
    }
}

#[cfg(test)]
mod test_mt_message_builder {
    use crate::mt::MTMessageBuilder;

    #[test]
    fn build() {
        let msg = MTMessageBuilder::default()
            .client_msg_id(9999)
            .imei([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5])
            .payload(vec![])
            .build();
        dbg!(msg);

        /*
            //builder.header.set_client_msg_id(9999);
            // let msg = msg.build();
            dbg!(msg.build());
            //assert!(false)
        */
    }
}
