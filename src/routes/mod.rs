use axum::Router;

mod hotel;
mod worker;
mod worker_role;
mod restore;
mod user;
mod root;
mod country;
mod region;
mod city;

pub fn router() -> Router {
    let router = Router::new();
    let router = hotel::route(router);
    let router = worker::route(router);
    let router = worker_role::route(router);
    let router = restore::route(router);
    let router = user::route(router);
    let router = root::route(router);
    let router = country::route(router);
    let router = region::route(router);
    let router = city::route(router);
    return router;
}