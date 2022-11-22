
use axum::{Extension, Router};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use sea_query::Alias;
use crate::ServerContext;

async fn db(
    Query(params): Query<Params>,
    Extension(ctx): Extension<ServerContext>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::prelude::*;
    use crate::types::CityIden;

    let (sql, values) = sea_query::Query::select()
        .column((CityIden::Table, CityIden::Id))
        .column((CityIden::Table, CityIden::Name))
        .columns([
            CityIden::RegionId
        ])
        .from(CityIden::Table)
        .pipe(|query| {
            match params.id {
                Some(id) => query.and_where(Expr::tbl(CityIden::Table, CityIden::Id).eq(id)),
                None => query
            }
        })
        .pipe(|query| {
            match params.name {
                Some(name) => query.and_where(Expr::tbl(CityIden::Table, CityIden::Name).eq(name)),
                None => query
            }
        })
        .pipe(|query| {
            match params.region_id {
                Some(region_id) => query.and_where(Expr::tbl(CityIden::Table,CityIden::RegionId).eq(region_id)),
                None => query
            }
        })
        .pipe(|query| {
            use crate::types::{RegionIden, CountryIden};

            match params.join.unwrap_or(false) {
                true => query
                    .expr_as(Expr::tbl(RegionIden::Table, RegionIden::Name), Alias::new("region_name"))
                    .expr_as(Expr::tbl(CountryIden::Table, CountryIden::Id), Alias::new("country_id"))
                    .expr_as(Expr::tbl(CountryIden::Table, CountryIden::Name), Alias::new("country_name"))
                    .inner_join(
                        RegionIden::Table,
                        Expr::tbl(RegionIden::Table, RegionIden::Id).eq(Expr::tbl(CityIden::Table, CityIden::RegionId))
                    ).inner_join(
                        CountryIden::Table,
                        Expr::tbl(CountryIden::Table, CountryIden::Id).eq(Expr::tbl(RegionIden::Table, RegionIden::CountryId))
                    ),
                false => query
            }
        })
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, crate::types::City, _>(&sql, values)
        .fetch_all(&ctx.pool).await;

    crate::utils::give_hint_on_relation_error_all(rows).into_response()
}

pub fn route(router: Router) -> Router {
    router.route("/city", get(city))
}