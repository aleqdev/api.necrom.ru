use axum::{Extension, Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Result as AxumResult};
use axum::routing::get;
use sea_query::Alias;
use crate::ServerContext;
use crate::types::Hotel;

#[derive(serde::Deserialize)]
struct Params {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub city_id: Option<i64>,
    pub owner_id: Option<i64>,
    pub join: Option<bool>
}

async fn hotel(
    Query(params): Query<Params>,
    Extension(ctx): Extension<ServerContext>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::prelude::*;
    use crate::types::HotelIden;

    let (sql, values) = sea_query::Query::select()
        .column((HotelIden::Table, HotelIden::Id))
        .column((HotelIden::Table, HotelIden::Name))
        .column((HotelIden::Table, HotelIden::Description))
        .columns([
            HotelIden::CityId,
            HotelIden::OwnerId,
        ])
        .from(HotelIden::Table)
        .pipe(|query| {
            match params.id {
                Some(id) => query.and_where(Expr::tbl(HotelIden::Table, HotelIden::Id).eq(id)),
                None => query
            }
        })
        .pipe(|query| {
            match params.name {
                Some(name) => query.and_where(Expr::tbl(HotelIden::Table, HotelIden::Name).eq(name)),
                None => query
            }
        })
        .pipe(|query| {
            match params.city_id {
                Some(city_id) => query.and_where(Expr::col(HotelIden::CityId).eq(city_id)),
                None => query
            }
        })
        .pipe(|query| {
            match params.owner_id {
                Some(owner_id) => query.and_where(Expr::col(HotelIden::OwnerId).eq(owner_id)),
                None => query
            }
        })
        .pipe(|query| {
            use crate::types::CityIden;
            use crate::types::RegionIden;
            use crate::types::CountryIden;
            use crate::types::WorkerIden;
            use crate::types::WorkerRoleIden;

            match params.join.unwrap_or(false) {
                true => query
                    .expr_as(Expr::tbl(CityIden::Table, CountryIden::Name), Alias::new("city_name"))
                    .expr_as(Expr::tbl(RegionIden::Table, CountryIden::Id), Alias::new("region_id"))
                    .expr_as(Expr::tbl(RegionIden::Table, CountryIden::Name), Alias::new("region_name"))
                    .expr_as(Expr::tbl(CountryIden::Table, CountryIden::Id), Alias::new("country_id"))
                    .expr_as(Expr::tbl(CountryIden::Table, CountryIden::Name), Alias::new("country_name"))
                    .expr_as(Expr::tbl(WorkerIden::Table, WorkerIden::RoleId), Alias::new("owner_role_id"))
                    .expr_as(Expr::tbl(WorkerIden::Table, WorkerIden::Name), Alias::new("owner_name"))
                    .expr_as(Expr::tbl(WorkerIden::Table, WorkerIden::Surname), Alias::new("owner_surname"))
                    .expr_as(Expr::tbl(WorkerIden::Table, WorkerIden::LastName), Alias::new("owner_last_name"))
                    .expr_as(Expr::tbl(WorkerIden::Table, WorkerIden::PhoneNumber), Alias::new("owner_phone_number"))
                    .expr_as(Expr::tbl(WorkerIden::Table, WorkerIden::Email), Alias::new("owner_email"))
                    .inner_join(
                        CityIden::Table,
                        Expr::tbl(CityIden::Table, CityIden::Id).eq(Expr::tbl(HotelIden::Table, HotelIden::CityId))
                    ).inner_join(
                        RegionIden::Table,
                        Expr::tbl(RegionIden::Table, RegionIden::Id).eq(Expr::tbl(CityIden::Table, CityIden::RegionId))
                    ).inner_join(
                        CountryIden::Table,
                        Expr::tbl(CountryIden::Table, CountryIden::Id).eq(Expr::tbl(RegionIden::Table, RegionIden::CountryId))
                    ).inner_join(
                        WorkerIden::Table,
                        Expr::tbl(WorkerIden::Table, WorkerIden::Id).eq(Expr::tbl(HotelIden::Table, HotelIden::OwnerId))
                    ).inner_join(
                        WorkerRoleIden::Table,
                        Expr::tbl(WorkerRoleIden::Table, WorkerRoleIden::Id).eq(Expr::tbl(WorkerIden::Table, WorkerIden::RoleId))
                    ),
                false => query
            }
        })
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, crate::types::Hotel, _>(&sql, values)
        .fetch_all(&ctx.pool).await.expect("Failed to execute");

    Json(rows)
}

#[derive(serde::Deserialize)]
struct PutParams {
    pub name: String,
    pub city_id: i64,
    pub owner_id: i64,
    pub description: String,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn put_hotel(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PutParams>
) -> AxumResult<Json<Hotel>> {
    use sea_query::PostgresQueryBuilder;
    use sea_query_binder::SqlxBinder;
    use crate::types::HotelIden;

    crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await?;

    let (sql, values) = sea_query::Query::insert()
        .into_table(HotelIden::Table)
        .columns([
            HotelIden::Name,
            HotelIden::CityId,
            HotelIden::OwnerId,
            HotelIden::Description
        ])
        .values([
            params.name.into(),
            params.city_id.into(),
            params.owner_id.into(),
            params.description.into()
        ]).map_err(|_| StatusCode::BAD_REQUEST)?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, Hotel, _>(&sql, values)
        .fetch_one(&ctx.pool).await.expect("Failed to execute");

    Ok(Json(rows))
}

#[derive(serde::Deserialize)]
struct PatchParams {
    pub name: Option<String>,
    pub city_id: Option<i64>,
    pub owner_id: Option<i64>,
    pub description: Option<String>,
    pub db_user_email: String,
    pub db_user_password: String
}

async fn patch_hotel(
    Extension(ctx): Extension<ServerContext>,
    Json(params): Json<PatchParams>,
    Path((id,)): Path<(i32,)>
) -> AxumResult<Json<Hotel>> {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::Pipe;
    use crate::types::HotelIden;

    crate::utils::verify_auth(
        &params.db_user_email,
        &params.db_user_password,
        &ctx
    ).await?;

    let (sql, values) = sea_query::Query::update()
        .table(HotelIden::Table)
        .and_where(Expr::col(HotelIden::Id).eq(id))
        .pipe(|query| {
            match params.name {
                Some(name) => query.values([(HotelIden::Name, name.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.city_id {
                Some(city_id) => query.values([(HotelIden::CityId, city_id.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.owner_id {
                Some(owner_id) => query.values([(HotelIden::OwnerId, owner_id.into())]),
                None => query
            }
        })
        .pipe(|query| {
            match params.description {
                Some(description) => query.values([(HotelIden::Description, description.into())]),
                None => query
            }
        })
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, Hotel, _>(&sql, values)
        .fetch_optional(&ctx.pool).await.expect("Failed to execute");

    match rows {
        Some(rows) => Ok(Json(rows)),
        None => Err(StatusCode::BAD_REQUEST.into())
    }
}

async fn get_hotel(
    Path((id,)): Path<(i32,)>
) -> impl IntoResponse {
    Redirect::to(&format!("/hotel?id={id}"))
}

pub fn route(router: Router) -> Router {
    router
        .route("/hotel", get(hotel).put(put_hotel))
        .route("/hotel/:id", get(get_hotel).patch(patch_hotel))
}