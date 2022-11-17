
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct City {
    pub id: i64,
    pub region_id: i64,
    pub name: String,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_id: Option<i64>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>
}

#[derive(sea_query::Iden)]
pub enum CityIden {
    #[iden = "city"]
    Table,
    Id,
    RegionId,
    Name
}
