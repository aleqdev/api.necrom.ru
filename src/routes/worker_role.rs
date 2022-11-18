use axum::{Extension, Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Result as AxumResult};
use axum::routing::get;
use crate::types::WorkerRole;
use crate::ServerContext;

#[derive(serde::Deserialize)]
struct GetParams {
    pub name: Option<String>,
    pub id: Option<i64>
}

async fn role(
    Query(params): Query<GetParams>,
    Extension(ctx): Extension<ServerContext>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::prelude::*;
    use crate::types::WorkerRoleIden;

    let (sql, values) = sea_query::Query::select()
        .columns([
            WorkerRoleIden::Id,
            WorkerRoleIden::Name,
        ])
        .from(WorkerRoleIden::Table)
        .pipe(|query| {
            match params.id {
                Some(id) => query.and_where(Expr::col(WorkerRoleIden::Id).eq(id)),
                None => query
            }
        })
        .pipe(|query| {
            match params.name {
                Some(name) => query.and_where(Expr::col(WorkerRoleIden::Name).eq(name)),
                None => query
            }
        })
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, WorkerRole, _>(&sql, values)
        .fetch_all(&ctx.pool).await.expect("Failed to execute");

    Json(rows)
}

#[derive(serde::Deserialize)]
struct PutParams {
    pub name: String,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn put_role(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PutParams>
) -> AxumResult<Json<WorkerRole>> {
    use sea_query::PostgresQueryBuilder;
    use sea_query_binder::SqlxBinder;
    use crate::types::WorkerRoleIden;

    crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await?;

    let (sql, values) = sea_query::Query::insert()
        .into_table(WorkerRoleIden::Table)
        .columns([
            WorkerRoleIden::Name,
        ])
        .values([params.name.into()]).map_err(|_| StatusCode::BAD_REQUEST)?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, WorkerRole, _>(&sql, values)
        .fetch_one(&ctx.pool).await.expect("Failed to execute");

    Ok(Json(rows))
}

#[derive(serde::Deserialize)]
struct PatchParams {
    pub name: Option<String>,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn patch_role(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PatchParams>,
    Path((id,)): Path<(i32,)>
) -> AxumResult<Json<WorkerRole>> {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::Pipe;
    use crate::types::WorkerRoleIden;

    crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await?;

    let (sql, values) = sea_query::Query::update()
        .table(WorkerRoleIden::Table)
        .and_where(Expr::col(WorkerRoleIden::Id).eq(id))
        .pipe(|query| {
            match params.name {
                Some(name) => query.values([(WorkerRoleIden::Name, name.into())]),
                None => query
            }
        })
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, WorkerRole, _>(&sql, values)
        .fetch_optional(&ctx.pool).await.expect("Failed to execute");

    match rows {
        Some(rows) => Ok(Json(rows)),
        None => Err(StatusCode::BAD_REQUEST.into())
    }
}

async fn get_role(
    Path((id,)): Path<(i32,)>
) -> impl IntoResponse {
    Redirect::to(&format!("/worker_role?id={id}"))
}

#[derive(serde::Deserialize)]
struct DeleteParams {
    pub db_user_email: String,
    pub db_user_password: String
}

async fn delete_role(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<DeleteParams>,
    Path((id,)): Path<(i64,)>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use crate::types::WorkerRoleIden;

    if let Err(code) = crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await {
        return code.into_response();
    }

    let (sql, values) = sea_query::Query::delete()
        .from_table(WorkerRoleIden::Table)
        .and_where(Expr::col(WorkerRoleIden::Id).eq(id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, WorkerRole, _>(&sql, values)
        .fetch_optional(&ctx.pool).await;

    crate::utils::give_hint_on_relation_error(result).into_response()
}

pub fn route(router: Router) -> Router {
    router
        .route("/worker_role", get(role).put(put_role))
        .route("/worker_role/:id", get(get_role).patch(patch_role).delete(delete_role))
}
