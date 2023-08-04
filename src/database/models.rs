use sqlx::FromRow;

#[derive(FromRow)]
pub struct Data {
    pub sum: i32,
}
