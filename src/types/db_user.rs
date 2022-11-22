#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct DatabaseUser {
    pub id: i32,
    pub email: String,
    pub password_hash: String
}

#[derive(sea_query::Iden)]
pub enum DatabaseUserIden {
    #[iden = "database_user"]
    Table,
    Id,
    Email,
    PasswordHash
}
