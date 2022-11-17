use axum::response::Redirect;
use axum::Router;
use axum::routing::get;

pub fn route(router: Router) -> Router {
    router.route("/", get(|| async { Redirect::to("/user") }))
}
