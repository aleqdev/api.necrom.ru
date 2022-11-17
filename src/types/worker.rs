#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Worker {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub role_id: i32,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(sea_query::Iden)]
pub enum WorkerIden {
    #[iden = "worker"]
    Table,
    Id,
    Name,
    Surname,
    LastName,
    PhoneNumber,
    Email,
    RoleId
}
