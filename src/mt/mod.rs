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
use confirmation::Confirmation;
use header::{Header, HeaderBuilder};
use payload::{Payload, PayloadBuilder};

trait InformationElementTemplate {
    fn identifier(&self) -> u8;
    fn len(&self) -> u16;
    /// Total size of Information Element in bytes
    /// This includes the identifier and the field len.
    fn total_size(&self) -> usize {
        3 + usize::from(self.len())
    }
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error>;
    /// Export Information Element to a vec
    fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        self.write(&mut buffer)
            .expect("Failed to write Information Element to a vec.");
        buffer
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum InformationElement {
    H(Header),
    P(Payload),
    C(Confirmation),
}

impl InformationElementTemplate for InformationElement {
    fn identifier(&self) -> u8 {
        match self {
            InformationElement::H(element) => element.identifier(),
            InformationElement::P(element) => element.identifier(),
            InformationElement::C(element) => element.identifier(),
        }
    }

    fn len(&self) -> u16 {
        match self {
            InformationElement::H(element) => element.len(),
            InformationElement::P(element) => element.len(),
            InformationElement::C(element) => element.len(),
        }
    }

    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        match self {
            InformationElement::H(element) => element.write(wtr),
            InformationElement::P(element) => element.write(wtr),
            InformationElement::C(element) => element.write(wtr),
        }
    }
}

impl InformationElement {
    #[allow(dead_code)]
    /// Parse a InformationElement from a Read trait
    pub(super) fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self, Error> {
        let iei = rdr.read_u8()?;
        let buffer = [iei; 1];
        let buffer = buffer.chain(rdr);
        let element = match iei {
            0x41 => {
                let header = Header::from_reader(buffer).unwrap();
                InformationElement::H(header)
            }
            0x42 => {
                let payload = Payload::from_reader(buffer).unwrap();
                InformationElement::P(payload)
            }
            0x44 => {
                let confirmation = Confirmation::from_reader(buffer).unwrap();
                InformationElement::C(confirmation)
            }
            _ => return Err(Error::Undefined),
        };
        Ok(element)
    }
}

#[cfg(test)]
mod test_mt_information_element {
    use crate::mt::InformationElement;

    #[test]
    // Need improvements to properly test it
    fn read() {
        let msg = [
            0x41, 0x00, 0x15, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x00, 0x3b,
        ]
        .as_slice();
        let _ie = InformationElement::from_reader(msg).unwrap();
    }
}

impl From<Confirmation> for InformationElement {
    fn from(confirmation: Confirmation) -> Self {
        InformationElement::C(confirmation)
    }
}

impl From<Header> for InformationElement {
    fn from(header: Header) -> Self {
        InformationElement::H(header)
    }
}

impl From<Payload> for InformationElement {
    fn from(payload: Payload) -> Self {
        InformationElement::P(payload)
    }
}

#[cfg(test)]
mod test_mt_information_element_from {
    use crate::mt::{Header, InformationElement, InformationElementTemplate, Payload};

    #[test]
    fn header() {
        let header = Header::builder()
            .client_msg_id(9999)
            .imei([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4])
            .build()
            .unwrap();
        let ie = InformationElement::from(header);
        assert!(ie.identifier() == 0x41);
    }

    #[test]
    fn payload() {
        let payload = Payload::builder().payload("Hey, it's me!").build().unwrap();
        let ie = InformationElement::from(payload);
        assert!(ie.identifier() == 0x42);
    }
}

#[derive(Debug)]
pub struct MTMessage {
    elements: Vec<InformationElement>,
}

// Let's allow dead while still WIP
#[allow(dead_code)]
impl MTMessage {
    // Length of the full message
    fn len(&self) -> u16 {
        self.elements.iter().map(|e| e.len()).sum()
    }

    fn total_size(&self) -> usize {
        3 + usize::from(self.len())
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

    fn new() -> MTMessage {
        MTMessage {
            elements: Vec::new(),
        }
    }

    pub fn builder() -> MTMessageBuilder {
        MTMessageBuilder::default()
    }

    /// Appends an element to the back of an MT-Message
    ///
    /// This should be a good place to check for duplicates, i.e. try to insert
    /// a second header for instance.
    fn push(&mut self, element: InformationElement) {
        self.elements.push(element);
    }

    /*
    fn from_reader<R: std::io::Read>(mut rdr: R) -> Result<Self, Error> {
        let version = rdr.read_u8()?;
        assert_eq!(version, 1);
        let len = rdr.read_u16::<BigEndian>()?;
        let mut msg = Self::new();
        while Some(element) = InformationElement::from_reader(rdr).unwrap() {
            msg.push(element);
        }

        Ok(msg)
    }
    */
}

#[cfg(test)]
mod test_mt_message {

    #[test]
    fn to_vec() {}
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
