
use axum::{Extension, Router};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use crate::ServerContext;

#[derive(serde::Deserialize)]
struct Params {
    pub name: Option<String>,
    pub id: Option<i64>
}

async fn country(
    Query(params): Query<Params>,
    Extension(ctx): Extension<ServerContext>
) -> impl IntoResponse {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use tap::prelude::*;
    use crate::types::CountryIden;

    let (sql, values) = sea_query::Query::select()
        .columns([
            CountryIden::Id,
            CountryIden::Name
        ])
        .from(CountryIden::Table)
        .pipe(|query| {
            match params.id {
                Some(id) => query.and_where(Expr::col(CountryIden::Id).eq(id)),
                None => query
            }
        })
        .pipe(|query| {
            match params.name {
                Some(name) => query.and_where(Expr::col(CountryIden::Name).eq(name)),
                None => query
            }
        })
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, crate::types::Country, _>(&sql, values)
        .fetch_all(&ctx.pool).await;

    crate::utils::give_hint_on_relation_error_all(rows).into_response()
}

pub fn route(router: Router) -> Router {
    router.route("/country", get(country))
}