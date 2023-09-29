use std::sync::RwLock;

use crate::Message;

pub struct VolatileStorage {
    // current_id: usize,
    data: RwLock<Vec<Message>>,
}

impl super::Storage for VolatileStorage {
    fn save(&mut self, msg: Message) {
        self.data.push(msg);
    }
}

impl VolatileStorage {
    pub(super) fn connect() -> VolatileStorage {
        VolatileStorage { data: vec![] }
    }
}
