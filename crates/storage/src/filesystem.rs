//   filesystem:///var/iridium-storage/

use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

use chrono::Utc;

use directip::Message;

pub struct FileSystemStorage {
    root: PathBuf,
    // current_id: usize,
}

impl super::Storage for FileSystemStorage {}

impl FileSystemStorage {
    pub(super) fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let tmp_dir = tempfile::TempDir::new().unwrap();
        Ok(FileSystemStorage {
            root: tmp_dir.path().into(),
        })
    }

    pub(super) async fn save(&self, msg: Message) {
        let mut path = self.root.clone();
        path.push("data");
        let mut filename = String::new();
        // Add IMEI?
        filename.push_str(&Utc::now().format("%Y%m%d%H%M%S%s").to_string());
        //filename.push_str(&format!("_{}", &self.current_id));
        filename.push_str(".isbd");
        path.push(filename);

        let mut file = BufWriter::new(File::create(path).unwrap());
        file.write(&msg.to_vec()).unwrap();
    }
    /*
    pub fn current_id(&self) -> usize {
        self.current_id
    }
    */

    // Probably distinguish between creating a new from scratch versus
    // connecting to an existing one. Connecting requires less checks,
    // faster, and a different level of errors.
    /*
    pub fn initiatedb(root: PathBuf) -> FileSystemStorage {
        let mut next_session_id = root.clone();
        next_session_id.push("next_session.txt");

        let current_id: usize = if next_session_id.exists() {
            let mut file = File::open(next_session_id).unwrap();
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            if buffer.ends_with('\n') {
                buffer.pop();
            }
            buffer.parse().unwrap()
        } else {
            let mut file = std::fs::File::create(next_session_id).unwrap();
            file.write(b"1").unwrap();
            1
            // let mut file = BufWriter::new(File::create(next_session_id).unwrap());
            //file.write("0").unwrap();
        };
        FileSystemStorage { root, current_id }
    }
    */
}

#[cfg(test)]
mod test_filesystem {
    use super::*;
    use directip;

    fn sample() -> directip::Message {
        let msg = directip::mt::MTMessage::from_reader(
            [
                0x01, 0x00, 0x1c, 0x44, 0x00, 0x19, 0x00, 0x00, 0x27, 0x0f, 0x00, 0x01, 0x02, 0x03,
                0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xf5,
            ]
            .as_slice(),
        );
        directip::Message::MT(msg.unwrap())
    }

    //#[tokio::test]
    async fn filesystem() {
        let storage = FileSystemStorage::connect().unwrap();
        storage.save(sample()).await;
    }
}
