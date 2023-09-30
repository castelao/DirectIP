use sqlx;

#[cfg(test)]
use tokio;

use directip::Message;

#[derive(Debug)]
pub struct SQLiteStorage {
    pool: sqlx::SqlitePool,
}

impl super::Storage for SQLiteStorage {}

impl SQLiteStorage {
    pub async fn connect() -> Result<SQLiteStorage, Box<dyn std::error::Error>> {
        // let pool = SqlitePool::connect("mssql://").await?;
        let pool = sqlx::SqlitePool::connect("sqlite://:memory:").await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS inbox (
                payload BLOB);",
        )
        .execute(&pool)
        .await
        .unwrap();

        Ok(SQLiteStorage { pool })
    }

    pub(super) async fn save(&self, msg: Message) {
        let _ = sqlx::query("INSERT INTO inbox (payload) VALUES ($1)")
            .bind(msg.to_vec())
            .execute(&self.pool)
            .await;
    }
}

#[cfg(test)]
mod test_sqlite {
    use super::*;

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
    async fn sqlite_inmemory() {
        let db = SQLiteStorage::connect().await.unwrap();
        db.save(sample());
    }
}
