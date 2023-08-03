// X provides connection(s) and access to a postgress database
// X pull connection string from env
// provide a method to verify if a registration code is valid
// provide a method to claim the registion code
// provide a method to save a users: email and password
// hash and salt the password
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

// struct RegistrationKey {
//     created_at: String,
//     modified_on: String,
//     modified_by: String,
//     claimed: bool,
//     owned_by: Option<String>,
// }

async fn get_pool() -> Option<Pool<Postgres>> {
    let port = env::var("DB_PORT").unwrap();
    let username = env::var("DB_USER").unwrap();
    let password = env::var("DB_PASS").unwrap();
    let name = env::var("DB_NAME").unwrap();
    let url = format!("postgresql://{username}:{password}@db:{port}/{name}");

    match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(pool) => Some(pool),
        Err(e) => {
            println!("[get_pool] {}", e);
            None
        }
    }
}

pub async fn verify() -> bool {
    let pool_or_none = get_pool().await;

    match pool_or_none {
        Some(pool) => {
            let response = sqlx::query("SELECT 1 + 1 as sum;").fetch_one(&pool).await;
            let result = match response {
                Ok(_row) => true,
                Err(e) => {
                    println!("[verify] {}", e);
                    false
                }
            };
            pool.close().await;

            result
        }
        None => false,
    }
}
