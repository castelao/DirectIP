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
            .expect("Failed to write MT-Confirmation to a vec.");
        buffer
    }
}
