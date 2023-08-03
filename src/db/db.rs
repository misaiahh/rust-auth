// X provides connection(s) and access to a postgress database
// pull connection string from env
// provide a method to verify if a registration code is valid
// provide a method to claim the registion code
// provide a method to save a users: email and password
// hash and salt the password
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// struct RegistrationKey {
//     created_at: String,
//     modified_on: String,
//     modified_by: String,
//     claimed: bool,
//     owned_by: Option<String>,
// }

async fn get_pool() -> Option<Pool<Postgres>> {
    let url = "postgresql://postgres:postgres@db:5432/postgres";

    match PgPoolOptions::new().max_connections(5).connect(url).await {
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
