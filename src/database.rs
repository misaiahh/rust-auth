mod models;
mod utils;

use models::RegistrationKeys;
use sqlx::query_as;
use utils::get_pool;

pub async fn get_sum() -> i32 {
    let registration_key = String::from("ABCD"); //
    let pool = get_pool().await;
    let response = query_as::<_, RegistrationKeys>(
        r#"SELECT "id", "registration_key" FROM RegistrationKeys WHERE "registration_key" = $1;"#,
    )
    .bind(registration_key)
    .fetch_one(&pool)
    .await;
    let result = match response {
        Ok(row) => {
            println!("SUM IS {}", row.id);
            row.id
        }
        Err(e) => {
            println!("[verify] {}", e);
            panic!();
        }
    };
    pool.close().await;

    result
}
