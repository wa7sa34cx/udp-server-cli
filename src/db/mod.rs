//! Database module
//!
//! This module is used to work with the SQLite database
//! in asynchronous mode relying on crate [`sqlx`](https://docs.rs/sqlx/).

pub mod models;

use anyhow::Result;
use dotenv::dotenv;
use models::*;
use sql_builder::{quote, SqlBuilder};
use sqlx::sqlite::SqlitePool;

type Pool = SqlitePool;

/// Asyncronous database
pub struct Db {
    pool: Pool,
}

impl Db {
    /// Initialize database from str
    #[allow(unused)]
    pub async fn new(database_url: &str) -> Self {
        let pool = Pool::connect(database_url).await.unwrap();

        Self { pool }
    }

    /// Initialize database from env
    pub async fn from_env() -> Self {
        let pool = Pool::connect(&get_env("DATABASE_URL")).await.unwrap();

        Self { pool }
    }

    /// Get item by id
    pub async fn get_by_id(&self, id: i64) -> Result<Vec<Item>> {
        let sql = SqlBuilder::select_from("items")
            .and_where_eq("chat_id", chat_id)
            .order_asc("id")
            .sql()?;

        let items: Vec<Item> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;

        Ok(items)
    }
}

fn get_env(env: &'static str) -> String {
    dotenv().ok();
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}
