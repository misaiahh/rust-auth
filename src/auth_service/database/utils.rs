use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn get_pool() -> Pool<Postgres> {
    let host = env::var("DB_HOST").unwrap_or(String::from("localhost"));
    let port = env::var("DB_PORT").unwrap_or(String::from("5432"));
    let username = env::var("DB_USER").unwrap_or(String::from("postgres"));
    let password = env::var("DB_PASS").unwrap_or(String::from("postgres"));
    let name = env::var("DB_NAME").unwrap_or(String::from("postgres"));
    let url = format!("postgresql://{username}:{password}@{host}:{port}/{name}");

    match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(pool) => pool,
        Err(e) => {
            println!("[get_pool] {}", e);
            panic!();
        }
    }
}
