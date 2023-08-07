use sqlx::FromRow;

#[derive(FromRow)]
pub struct Data {
    pub sum: i32,
}

#[derive(Debug, FromRow)]
pub struct RegistrationKeys {
    pub id: i64,
    pub key: String,
    pub owned: bool,
    pub owned_by: i64,
}
