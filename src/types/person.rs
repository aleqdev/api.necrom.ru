
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
}

#[derive(sea_query::Iden)]
pub enum PersonIden {
    #[iden = "person"]
    Table,
    Id,
    Name,
    Surname,
    LastName,
    PhoneNumber,
    Email
}
