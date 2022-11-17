use axum::http::StatusCode;
use axum::Router;
use axum::routing::get_service;
use tower_http::services::ServeDir;

pub fn route(router: Router) -> Router {
    router.nest(
        "/user",
        get_service(ServeDir::new("./user")).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }),
    )
}
