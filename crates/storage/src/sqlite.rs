use sqlx;

#[cfg(test)]
use tokio;

use directip::Message;

pub struct SQLiteStorage {
    pool: sqlx::SqlitePool,
}

impl super::Storage for SQLiteStorage {
    fn save(&mut self, msg: Message) {
        unimplemented!()
    }
}

impl SQLiteStorage {}
