//   filesystem:///var/iridium-storage/

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use chrono::Utc;

use directip::Message;

#[derive(Debug)]
pub struct FileSystemStorage {
    root: PathBuf,
    // current_id: usize,
}

impl super::Storage for FileSystemStorage {}

impl FileSystemStorage {
    pub(super) fn connect(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        assert!(path.is_dir());
        Ok(FileSystemStorage { root: path })
    }

    pub(super) async fn save(&self, msg: Message) {
        let mut path = self.root.clone();

        // Data directory
        path.push("data");
        if !path.exists() {
            tracing::warn!("Creating missing data directory: {:?}", path);
            std::fs::create_dir(&path).unwrap();
        }

        // A directory for each modem
        let imei = match msg.imei() {
            Some(i) => i
                .into_iter()
                .map(|d| format!("{:02x?}", d))
                .collect::<String>(),
            None => "Unknown".to_string(),
        };
        path.push(imei);
        if !path.exists() {
            tracing::info!("Creating directory for new platform: {:?}", path);
            std::fs::create_dir(&path).unwrap();
        }

        // One modem can accumulate a lot of messages. The issue here is not
        // size, but the number of items for some file systems.
        path.push(&Utc::now().format("%Y").to_string());
        if !path.exists() {
            tracing::info!("New annual directory: {:?}", path);
            std::fs::create_dir(&path).unwrap();
        }

        let mut filename = String::new();
        // Add IMEI?
        filename.push_str(&Utc::now().format("%Y%m%d%H%M%S%s").to_string());
        //filename.push_str(&format!("_{}", &self.current_id));
        filename.push_str(".isbd");
        tracing::debug!("Message filename: {:?}", filename);

        path.push(filename);

        tracing::info!("Saving message as: {:?}", path);
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
    use directip::sample;

    #[tokio::test]
    async fn filesystem() {
        let tmp_dir = tempfile::TempDir::new().unwrap();
        let storage = FileSystemStorage::connect(tmp_dir.into_path()).unwrap();
        storage.save(sample()).await;
    }
}
