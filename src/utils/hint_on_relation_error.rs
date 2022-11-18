use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::json;
use sqlx::Error;
use sqlx::Error::Database;

pub fn give_hint_on_relation_error<T: Serialize>(
    result: Result<Option<T>, Error>
) -> impl IntoResponse {
    match result {
        Ok(row) => {
            match row {
                None => StatusCode::BAD_REQUEST.into_response(),
                Some(row) => (StatusCode::OK, Json(row)).into_response()
            }
        }
        Err(err) => {
            if let Database(err) = err {
                if Some("23503") == err.code().as_ref().map(|c| c.as_ref()) {
                    use regex::Regex;

                    let r = Regex::new("\"([^\"]*)\"").unwrap();
                    let mut caps = r.captures_iter(err.message());
                    let target = caps.next().unwrap();
                    let _ = caps.next().unwrap();
                    let issue = caps.next().unwrap();

                    (StatusCode::BAD_REQUEST, Json(json!({
                        "error_hint": "existing_relation",
                        "target": target[1],
                        "issue": issue[1]
                    }))).into_response()
                } else {
                    StatusCode::BAD_REQUEST.into_response()
                }
            } else {
                StatusCode::BAD_REQUEST.into_response()
            }
        }
    }
}
