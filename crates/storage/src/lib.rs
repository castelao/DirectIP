mod filesystem;
mod inmemory;

use directip::Message;
use filesystem::FileSystemStorage;
use inmemory::VolatileStorage;

// Feature sqlite
#[cfg(feature = "sqlite")]
mod sqlite;

trait Storage {
    //fn connect(cfg: String) -> Self;
    // fn save(&self, msg: Message);
}

enum Database {
    M(VolatileStorage),
    F(FileSystemStorage),
    #[cfg(feature = "sqlite")]
    L(sqlite::SQLiteStorage),
}

impl Database {
    pub async fn open(cfg: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if (&cfg.len() >= &(11 as usize)) && (cfg[..11] == "inmemory://".to_string()) {
            Ok(Database::M(VolatileStorage::connect()?))
        } else if (&cfg.len() >= &(13 as usize)) && (cfg[..13] == "filesystem://".to_string()) {
            Ok(Database::F(FileSystemStorage::connect()?))
        } else if cfg[..9] == "sqlite://".to_string() {
            #[cfg(feature = "sqlite")]
            {
                let db = crate::sqlite::SQLiteStorage::connect().await;
                Ok(Database::L(db))
            }
            #[cfg(not(feature = "sqlite"))]
            unimplemented!("Missing sqlite feature")
        } else {
            unimplemented!("Unknown storage")
        }
        /*
        Database::F(FileSystemStorage::initiatedb(std::path::PathBuf::from(
            "./",
        )))
        */
    }

    pub async fn save(&self, msg: Message) {
        match self {
            Database::M(s) => s.save(msg).await,
            Database::F(s) => s.save(msg).await,
            #[cfg(feature = "sqlite")]
            Database::L(s) => s.save(msg).await,
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Database;

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

    #[tokio::test]
    async fn volatile() {
        let db = Database::open("inmemory://").await.unwrap();
        db.save(sample());
    }

    //#[tokio::test]
    async fn filesystem() {
        let db = Database::open("filesystem://").await.unwrap();
        db.save(sample());
    }

    #[cfg(feature = "sqlite")]
    #[tokio::test]
    async fn open_sqlite() {
        let db = Database::open("sqlite://").await.unwrap();
        db.save(sample());
    }
}
