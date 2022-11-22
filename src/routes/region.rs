
use axum::{Extension, Router};
use axum::extract::{Path, Json, Query};
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use sea_query::Alias;
use crate::ServerContext;
use crate::types::Region;

#[derive(serde::Deserialize)]
struct Params {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub country_id: Option<i64>,
    pub join: Option<bool>
}

async fn region(
    Query(params): Query<Params>,
    Extension(ctx): Extension<ServerContext>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::prelude::*;
    use crate::types::RegionIden;

    let (sql, values) = sea_query::Query::select()
        .column((RegionIden::Table, RegionIden::Id))
        .column((RegionIden::Table, RegionIden::Name))
        .columns([
            RegionIden::CountryId
        ])
        .from(RegionIden::Table)
        .pipe(|query| {
            match params.id {
                Some(id) => query.and_where(Expr::tbl(RegionIden::Table, RegionIden::Id).eq(id)),
                None => query
            }
        })
        .pipe(|query| {
            match params.name {
                Some(name) => query.and_where(Expr::tbl(RegionIden::Table, RegionIden::Name).eq(name)),
                None => query
            }
        })
        .pipe(|query| {
            match params.country_id {
                Some(country_id) => query.and_where(Expr::col(RegionIden::CountryId).eq(country_id)),
                None => query
            }
        })
        .pipe(|query| {
            use crate::types::CountryIden;

            match params.join.unwrap_or(false) {
                true => query
                    .expr_as(Expr::tbl(CountryIden::Table, CountryIden::Name), Alias::new("country_name"))
                    .inner_join(
                        CountryIden::Table,
                        Expr::tbl(CountryIden::Table, CountryIden::Id).eq(Expr::tbl(RegionIden::Table, RegionIden::CountryId))
                    ),
                false => query
            }
        })
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, crate::types::Region, _>(&sql, values)
        .fetch_all(&ctx.pool).await;

    crate::utils::give_hint_on_relation_error_all(rows).into_response()
}

#[derive(serde::Deserialize)]
struct PutParams {
    pub name: String,
    pub country_id: i64,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn put_region(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PutParams>
) -> impl IntoResponse {
    use sea_query::PostgresQueryBuilder;
    use sea_query_binder::SqlxBinder;
    use crate::types::RegionIden;

    if let Err(code) = crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await {
      return code.into_response()
    }

    let (sql, values) = sea_query::Query::insert()
        .into_table(RegionIden::Table)
        .columns([
            RegionIden::Name,
            RegionIden::CountryId
        ])
        .values_panic([
            params.name.into(),
            params.country_id.into()
        ])
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, Region, _>(&sql, values)
        .fetch_optional(&ctx.pool).await;

    crate::utils::give_hint_on_relation_error_option(result).into_response()
}

#[derive(serde::Deserialize)]
struct PatchParams {
    pub name: Option<String>,
    pub country_id: Option<i64>,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn patch_region(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PatchParams>,
    Path((id,)): Path<(i32,)>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::Pipe;
    use crate::types::RegionIden;

    if let Err(code) = crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await {
      return code.into_response();
    }

    let (sql, values) = sea_query::Query::update()
        .table(RegionIden::Table)
        .and_where(Expr::col(RegionIden::Id).eq(id))
        .pipe(|query| {
            match params.name {
                Some(name) => query.values([(RegionIden::Name, name.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.country_id {
                Some(country_id) => query.values([(RegionIden::CountryId, country_id.into())]),
                None => query
            }
        })
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with::<_, Region, _>(&sql, values)
        .fetch_optional(&ctx.pool).await;

    crate::utils::give_hint_on_relation_error_option(result).into_response()
}

async fn get_region(
    Path((id,)): Path<(i32,)>
) -> impl IntoResponse {
    Redirect::to(&format!("/region?id={id}"))
}

pub fn route(router: Router) -> Router {
    router
        .route("/region", get(region).put(put_region))
        .route("/region/:id", get(get_region).patch(patch_region))
}