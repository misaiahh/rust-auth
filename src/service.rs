use crate::database::get_sum;

pub async fn verify() -> bool {
    let sum = get_sum().await;

    sum > 0
}
