#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct EmployeeRole {
    pub id: i32,
    pub name: String,
}

#[derive(sea_query::Iden)]
pub enum EmployeeRoleIden {
    #[iden = "employee_role"]
    Table,
    Id,
    Name
}
