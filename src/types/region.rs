#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: i64,
    pub country_id: i64,
    pub name: String,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>
}

#[derive(sea_query::Iden)]
pub enum RegionIden {
    #[iden = "region"]
    Table,
    Id,
    CountryId,
    Name
}
