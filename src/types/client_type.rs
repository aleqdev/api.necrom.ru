#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ClientType {
    pub id: i32,
    pub name: String,
}

#[derive(sea_query::Iden)]
pub enum ClientTypeIden {
    #[iden = "client_type"]
    Table,
    Id,
    Name
}
