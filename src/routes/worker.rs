use axum::{Extension, Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Result as AxumResult};
use axum::routing::get;
use sea_query::Alias;
use crate::ServerContext;
use crate::types::Worker;

#[derive(serde::Deserialize)]
struct Params {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub role_id: Option<i64>,
    pub join: Option<bool>
}

async fn worker(
    Query(params): Query<Params>,
    Extension(ctx): Extension<ServerContext>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::prelude::*;
    use crate::types::WorkerIden;

    let (sql, values) = sea_query::Query::select()
        .column((WorkerIden::Table, WorkerIden::Id))
        .column((WorkerIden::Table, WorkerIden::Name))
        .columns([
            WorkerIden::Surname,
            WorkerIden::LastName,
            WorkerIden::Email,
            WorkerIden::PhoneNumber,
            WorkerIden::RoleId
        ])
        .from(WorkerIden::Table)
        .pipe(|query| {
            match params.id {
                Some(id) => query.and_where(Expr::tbl(WorkerIden::Table, WorkerIden::Id).eq(id)),
                None => query
            }
        })
        .pipe(|query| {
            match params.name {
                Some(name) => query.and_where(Expr::tbl(WorkerIden::Table, WorkerIden::Name).eq(name)),
                None => query
            }
        })
        .pipe(|query| {
            match params.surname {
                Some(surname) => query.and_where(Expr::col(WorkerIden::Surname).eq(surname)),
                None => query
            }
        })
        .pipe(|query| {
            match params.last_name {
                Some(last_name) => query.and_where(Expr::col(WorkerIden::LastName).eq(last_name)),
                None => query
            }
        })
        .pipe(|query| {
            match params.email {
                Some(email) => query.and_where(Expr::col(WorkerIden::Email).eq(email)),
                None => query
            }
        })
        .pipe(|query| {
            match params.phone_number {
                Some(phone_number) => query.and_where(Expr::col(WorkerIden::PhoneNumber).eq(phone_number)),
                None => query
            }
        })
        .pipe(|query| {
            match params.role_id {
                Some(role_id) => query.and_where(Expr::col(WorkerIden::RoleId).eq(role_id)),
                None => query
            }
        })
        .pipe(|query| {
            use crate::types::WorkerRoleIden;

            match params.join.unwrap_or(false) {
                true => query
                    .expr_as(Expr::tbl(WorkerRoleIden::Table, WorkerRoleIden::Name), Alias::new("role_name"))
                    .inner_join(
                        WorkerRoleIden::Table,
                        Expr::tbl(WorkerRoleIden::Table, WorkerRoleIden::Id).eq(Expr::tbl(WorkerIden::Table, WorkerIden::RoleId))
                    ),
                false => query
            }
        })
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, Worker, _>(&sql, values)
        .fetch_all(&ctx.pool).await.expect("Failed to execute");

    Json(rows)
}

#[derive(serde::Deserialize)]
struct PutParams {
    pub name: String,
    pub surname: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub role_id: i64,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn put_worker(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PutParams>
) -> AxumResult<Json<Worker>> {
    use sea_query::PostgresQueryBuilder;
    use sea_query_binder::SqlxBinder;
    use crate::types::WorkerIden;

    crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await?;

    let (sql, values) = sea_query::Query::insert()
        .into_table(WorkerIden::Table)
        .columns([
            WorkerIden::Name,
            WorkerIden::Surname,
            WorkerIden::LastName,
            WorkerIden::Email,
            WorkerIden::PhoneNumber,
            WorkerIden::RoleId
        ])
        .values([
            params.name.into(),
            params.surname.into(),
            params.last_name.into(),
            params.email.into(),
            params.phone_number.into(),
            params.role_id.into()
        ]).map_err(|_| StatusCode::BAD_REQUEST)?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, Worker, _>(&sql, values)
        .fetch_one(&ctx.pool).await.expect("Failed to execute");

    Ok(Json(rows))
}

#[derive(serde::Deserialize)]
struct PatchParams {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub role_id: Option<i64>,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn patch_worker(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PatchParams>,
    Path((id,)): Path<(i32,)>
) -> AxumResult<Json<Worker>> {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::Pipe;
    use crate::types::WorkerIden;

    crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await?;

    let (sql, values) = sea_query::Query::update()
        .table(WorkerIden::Table)
        .and_where(Expr::col(WorkerIden::Id).eq(id))
        .pipe(|query| {
            match params.name {
                Some(name) => query.values([(WorkerIden::Name, name.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.surname {
                Some(surname) => query.values([(WorkerIden::Surname, surname.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.last_name {
                Some(last_name) => query.values([(WorkerIden::LastName, last_name.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.email {
                Some(email) => query.values([(WorkerIden::Email, email.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.phone_number {
                Some(phone_number) => query.values([(WorkerIden::PhoneNumber, phone_number.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.role_id {
                Some(role_id) => query.values([(WorkerIden::RoleId, role_id.into())]),
                None => query
            }
        })
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, Worker, _>(&sql, values)
        .fetch_optional(&ctx.pool).await.expect("Failed to execute");

    match rows {
        Some(rows) => Ok(Json(rows)),
        None => Err(StatusCode::BAD_REQUEST.into())
    }
}

async fn get_worker(
    Path((id,)): Path<(i64,)>
) -> impl IntoResponse {
    Redirect::to(&format!("/worker?id={id}"))
}

#[derive(serde::Deserialize)]
struct DeleteParams {
    pub db_user_email: String,
    pub db_user_password: String
}

async fn delete_worker(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<DeleteParams>,
    Path((id,)): Path<(i64,)>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use crate::types::WorkerIden;

    if let Err(code) = crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await {
        return code.into_response();
    }

    let (sql, values) = sea_query::Query::delete()
        .from_table(WorkerIden::Table)
        .and_where(Expr::col(WorkerIden::Id).eq(id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, Worker, _>(&sql, values)
        .fetch_optional(&ctx.pool).await;

    crate::utils::give_hint_on_relation_error(result).into_response()
}

pub fn route(router: Router) -> Router {
    router
        .route("/worker", get(worker).put(put_worker))
        .route("/worker/:id", get(get_worker).patch(patch_worker).delete(delete_worker))
}