use axum::http::StatusCode;
use crate::ServerContext;

pub async fn verify_auth(
    email: &str,
    password: &str,
    ctx: &ServerContext
) -> Result<(), StatusCode> {
    use sea_query::{Expr, PostgresQueryBuilder};
    use sea_query_binder::SqlxBinder;
    use bcrypt::verify;
    use crate::types::DbUserIden;

    let (sql, values) = sea_query::Query::select()
        .from(DbUserIden::Table)
        .columns([
            DbUserIden::Id,
            DbUserIden::Email,
            DbUserIden::PasswordHash,
        ])
        .and_where(Expr::col(DbUserIden::Email).eq(email))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, crate::types::DbUser, _>(&sql, values)
        .fetch_optional(&ctx.pool).await.expect("Failed to execute");

    let Some(user) = user else {
        return Err(StatusCode::UNAUTHORIZED)
    };

    match verify(&password, &user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        true => Ok(()),
        false => Err(StatusCode::UNAUTHORIZED)
    }
}