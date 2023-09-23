//   filesystem:///var/iridium-storage/

use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

use directip::Message;

pub struct FileSystemStorage {
    root: PathBuf,
}

impl super::Storage for FileSystemStorage {
    fn save(&mut self, msg: Message) {
        unimplemented!()
    }
}
