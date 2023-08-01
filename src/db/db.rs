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
    // let pool = sqlx::postgres::PgPool::connect(url).await?;
    let pool = PgPoolOptions::new().connect(url).await?;

    Ok(pool)
}

pub async fn verify() -> bool {
    let pool = get_pool().await.unwrap();

    let res = sqlx::query("SELECT 1 + 1 as sum;")
        .fetch_one(&pool)
        .await
        .unwrap();

    false
}
