use crate::Message;

pub struct VolatileStorage {
    // current_id: usize,
    data: Vec<Message>,
}

impl super::Storage for VolatileStorage {
    fn save(&mut self, msg: Message) {
        self.data.push(msg);
    }
}

impl VolatileStorage {}
