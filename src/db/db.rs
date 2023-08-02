// provides connection(s) and access to a postgress database
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

async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let url = "postgresql://postgres:postgres@db:5432/postgres";

    match PgPoolOptions::new().max_connections(5).connect(url).await {
        Ok(pool) => Ok(pool),
        Err(error) => {
            println!("[get_pool] {}", error);
            Err(error)
        }
    }
}

pub async fn verify() -> bool {
    let pool = get_pool().await.unwrap();

    let result = sqlx::query("SELECT 1 + 1 as sum;").fetch_one(&pool).await;

    match result {
        Ok(_row) => true,
        Err(_) => false,
    }
}
