#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Country {
    pub id: i64,
    pub name: String
}

#[derive(sea_query::Iden)]
pub enum CountryIden {
    #[iden = "country"]
    Table,
    Id,
    Name
}
