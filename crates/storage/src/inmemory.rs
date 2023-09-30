use std::sync::RwLock;

use crate::Message;

pub struct VolatileStorage {
    // current_id: usize,
    data: RwLock<Vec<Message>>,
}

impl super::Storage for VolatileStorage {}

impl VolatileStorage {
    pub(super) fn connect() -> Result<VolatileStorage, Box<dyn std::error::Error>> {
        Ok(VolatileStorage {
            data: RwLock::new(vec![]),
        })
    }

    pub(super) async fn save(&self, msg: Message) {
        self.data
            .write()
            .expect("Failed to acquire write lock.")
            .push(msg);
    }
}

#[cfg(test)]
mod test_volatile {
    use super::VolatileStorage;
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

    #[tokio::test]
    async fn volatile() {
        let storage = VolatileStorage::connect().unwrap();
        storage.save(sample()).await;

        assert_eq!(storage.data.read().unwrap().len(), 1);
    }
}
