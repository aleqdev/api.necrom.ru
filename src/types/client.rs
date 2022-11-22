#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Client {
    pub id: i32,
    pub person_id: i32,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_surname: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_last_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_phone_number: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_email: Option<String>,
    pub role_id: i32,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(sea_query::Iden)]
pub enum ClientIden {
    #[iden = "client"]
    Table,
    Id,
    Name,
    Surname,
    LastName,
    PhoneNumber,
    Email,
    RoleId
}
