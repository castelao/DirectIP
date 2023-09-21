use crate::MOMessage;

pub struct VolatileStorage {
    current_id: usize,
    data: Vec<MOMessage>,
}

impl super::Storage for VolatileStorage {}

impl VolatileStorage {}
