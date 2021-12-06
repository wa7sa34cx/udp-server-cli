//! Database module
//!
//! This module is used to work with the SQLite database
//! in asynchronous mode relying on crate [`sqlx`].
//! 
//! [`sqlx`]: https://docs.rs/sqlx/

pub mod models;

use anyhow::Result;
use dotenv::dotenv;
use models::*;
use sql_builder::SqlBuilder;
use sqlx::sqlite::SqlitePool;

type Pool = SqlitePool;

/// Asyncronous database
pub struct Db {
    pool: Pool,
}

impl Db {
    /// Initializes database from str
    #[allow(unused)]
    pub async fn new(database_url: &str) -> Self {
        let pool = Pool::connect(database_url).await.unwrap();

        Self { pool }
    }

    /// Initializes database from env
    pub async fn from_env() -> Self {
        let pool = Pool::connect(&get_env("DATABASE_URL")).await.unwrap();

        Self { pool }
    }

    /// Gets item by id
    pub async fn get_by_id(&self, id: i64) -> Result<Item> {
        let sql = SqlBuilder::select_from("items")
            .and_where_eq("id", id)
            .order_asc("id")
            .sql()?;

        let item: Item = sqlx::query_as(&sql).fetch_one(&self.pool).await?;

        Ok(item)
    }
}

fn get_env(env: &'static str) -> String {
    dotenv().ok();
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db() {
        let db = Db::from_env().await;
        
        test_get_by_id(&db).await;
    }

    async fn test_get_by_id(db: &Db) {
        assert_eq!(db.get_by_id(1).await.unwrap().text, "one".to_string());
        assert_eq!(db.get_by_id(2).await.unwrap().text, "two".to_string());
        assert_eq!(db.get_by_id(3).await.unwrap().text, "three".to_string());
    }
}
