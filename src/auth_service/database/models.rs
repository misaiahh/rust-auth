use sqlx::FromRow;

#[derive(FromRow)]
pub struct Data {
    pub sum: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RegistrationKeys {
    pub id: i32,
    pub registration_key: String,
    // pub owned: bool,
    // pub owned_by: i64,
}
