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
        Ok(FileSystemStorage {
            root: "./demo".into(),
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

impl super::Storage for FileSystemStorage {
    fn save(&mut self, msg: Message) {
        unimplemented!()
    }
}
