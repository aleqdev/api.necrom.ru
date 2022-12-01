use axum::body::Body;
use axum::{Extension, Router};
use axum::http::{HeaderValue, Request, StatusCode, Uri};
use axum::response::{IntoResponse};
use axum::routing::get;
use crate::ServerContext;

async fn db(Extension(ctx): Extension<ServerContext>, mut req: Request<Body>) -> impl IntoResponse {
    let Some(Ok(db_user_email)) = req.headers().get("DB-User-Email").map(|x| x.to_str()) else {
        return StatusCode::UNAUTHORIZED.into_response()
    };

    let Some(Ok(db_user_password)) = req.headers().get("DB-User-Password").map(|x| x.to_str()) else {
        return StatusCode::UNAUTHORIZED.into_response()
    };

    if let Err(code) = crate::utils::verify_auth(
        db_user_email,
        db_user_password,
        &ctx
    ).await {
        return code.into_response()
    };

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let Some(path_query) = path_query.split("db").skip(1).next() else {
        return StatusCode::BAD_REQUEST.into_response()
    };

    let uri = format!("{}{}", ctx.postgrest_url, path_query);

    tracing::info!("Proxying: {}", uri);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    let resp = ctx.client.request(req).await;

    return match resp {
        Ok(mut resp) => {
            resp.headers_mut().insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
            resp.into_response()
        },
        Err(err) => {
            tracing::info!("error: {}", err);
            return StatusCode::BAD_REQUEST.into_response()
        }
    }
}

pub fn route(router: Router) -> Router {
    router.route("/db/*path", get(db).post(db).put(db).patch(db).delete(db))
}
