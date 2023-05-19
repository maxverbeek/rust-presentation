use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct MacStorage {
    pool: Pool<Postgres>,
}

#[derive(Serialize)]
pub struct MacUser {
    pub user: String,
    pub mac: String,
}

impl MacStorage {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(db_url)
            .await?;

        Ok(MacStorage { pool })
    }

    pub async fn add_macuser(self, user: &MacUser) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO macusers (name, macaddr) VALUES ($1, $2)",
            user.user,
            user.mac
        )
        .execute(&self.pool)
        .await
        .map(|_| ()) // ignore the result
    }

    pub async fn list_users(self) -> Result<Vec<MacUser>, sqlx::Error> {
        let users = sqlx::query_as!(
            MacUser,
            "SELECT name as user, macaddr as mac FROM macusers;"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}
