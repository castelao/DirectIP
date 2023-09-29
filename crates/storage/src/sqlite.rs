use sqlx;

#[cfg(test)]
use tokio;

use directip::Message;

#[derive(Debug)]
pub struct SQLiteStorage {
    pool: sqlx::SqlitePool,
}

impl super::Storage for SQLiteStorage {
}

impl SQLiteStorage {
    pub async fn connect() -> Result<SQLiteStorage, Box<dyn std::error::Error>> {
        let pool = sqlx::SqlitePool::connect("sqlite://:memory:").await?;
        Ok(SQLiteStorage { pool })
    }

    pub(super) async fn save(&self, msg: Message) {
        let _ = sqlx::query("INSERT INTO inbox (payload) VALUES ($1)")
            .bind(msg.to_vec())
            .execute(&self.pool)
            .await;
    }

    async fn save(&self, msg: Message) {
            sqlx::query("INSERT INTO inbox (payload) VALUES ($1)")
                .bind(msg.to_vec())
                .execute(&self.pool)
                .await;
    }
}
