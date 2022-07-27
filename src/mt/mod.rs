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

use crate::error::DirectIPError as Error;

mod header;

trait InformationElementTemplate {
    fn len(&self) -> u16;
    fn total_size(&self) -> usize {
        3 + usize::from(self.len())
    }
    fn write<W: std::io::Write>(&self, wtr: &mut W) -> Result<usize, Error>;
    fn to_vec(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        self.write(&mut buffer)
            .expect("Failed to write Information Element to a vec.");
        buffer
    }
}
