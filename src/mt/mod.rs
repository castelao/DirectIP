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

use crate::error::Error;

mod confirmation;
mod header;
mod payload;

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

impl InformationElement {
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

#[derive(Debug)]
struct MTMessage {
    elements: Vec<InformationElement>,
}

// Let's allow dead while still WIP
#[allow(dead_code)]
impl MTMessage {
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error> {
        // Protocol version
        wtr.write_u8(1)?;
        let n: usize = self.elements.iter().map(|e| e.len()).sum();
        wtr.write_u16::<BigEndian>(
            n.try_into()
                .expect("Sum of MT information elements lengths is longer than u16"),
        )?;

        let mut n = 3;
        for e in &self.elements {
            n += e.write(wtr)?;
        }
        Ok(n)
    }
}
