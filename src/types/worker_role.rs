#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct WorkerRole {
    pub id: i32,
    pub name: String,
}

#[derive(sea_query::Iden)]
pub enum WorkerRoleIden {
    #[iden = "worker_role"]
    Table,
    Id,
    Name
}
