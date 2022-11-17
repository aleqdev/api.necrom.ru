#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Hotel {
    pub id: i32,
    pub name: String,
    pub city_id: i64,
    pub owner_id: i32,
    pub description: String,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<i64>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_id: Option<i64>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_role_id: Option<i32>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_surname: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_last_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_phone_number: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_email: Option<String>,
}

#[derive(sea_query::Iden)]
pub enum HotelIden {
    #[iden = "hotel"]
    Table,
    Id,
    Name,
    CityId,
    OwnerId,
    Description
}
