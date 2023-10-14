use std::sync::RwLock;

use crate::Message;

#[derive(Debug)]
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
    use directip::sample;

    #[tokio::test]
    async fn volatile() {
        let storage = VolatileStorage::connect().unwrap();
        storage.save(sample()).await;

        assert_eq!(storage.data.read().unwrap().len(), 1);
    }
}
