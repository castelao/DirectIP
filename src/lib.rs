//! Direct-IP Protocol
//!

mod error;
pub mod mo;
pub mod mt;

use crate::error::Error;

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
