use sqlx::SqlitePool;

pub mod types;
pub mod user;

pub type DB = SqlitePool;
