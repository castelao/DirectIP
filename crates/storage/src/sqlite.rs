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
    use directip::sample;

    #[tokio::test]
    async fn sqlite_inmemory() {
        let db = SQLiteStorage::connect().await.unwrap();
        db.save(sample());
    }
}
