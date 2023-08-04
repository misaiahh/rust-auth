mod models;
mod utils;

use models::Data;
pub use utils::get_pool;

pub async fn get_sum() -> i32 {
    let pool = get_pool().await;
    let response = sqlx::query_as::<_, Data>("SELECT 1 + 1 as sum;")
        .fetch_one(&pool)
        .await;
    let result = match response {
        Ok(row) => {
            println!("SUM IS {}", row.sum);
            row.sum
        }
        Err(e) => {
            println!("[verify] {}", e);
            panic!();
        }
    };
    pool.close().await;

    result
}
