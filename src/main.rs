use axum::Extension;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;

pub mod routes;
pub mod types;
pub mod utils;

#[derive(Clone)]
pub struct ServerContext {
    pub pool: PgPool,
    pub restore_key: String
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("DOCKER_TEST_PORT").expect("DOCKER_TEST_PORT env is not set.");
    let pg_name = std::env::var("DOCKER_TEST_PG_HOST").expect("DOCKER_TEST_PG_HOST env is not set.");
    let pg_user = std::env::var("DOCKER_TEST_PG_USER").expect("DOCKER_TEST_PG_USER env is not set.");
    let pg_pass = std::env::var("DOCKER_TEST_PG_PASSWORD").expect("DOCKER_TEST_PG_PASSWORD env is not set.");
    let pg_port = std::env::var("DOCKER_TEST_PG_PORT").expect("DOCKER_TEST_PG_PORT env is not set.");
    let pg_db = std::env::var("DOCKER_TEST_PG_DATABASE").expect("DOCKER_TEST_PG_DATABASE env is not set.");
    let pg_restore_key = std::env::var("DOCKER_TEST_PG_RESTORE_KEY").expect("DOCKER_TEST_PG_RESTORE_KEY env is not set.");
    let addr = format!("0.0.0.0:{port}");
    let conn_str = format!("postgres://{pg_user}:{pg_pass}@{pg_name}:{pg_port}/{pg_db}");


    tracing::info!("Using restore key: {pg_restore_key}");
    tracing::info!("Using port: {port}");
    tracing::info!("Using addr: {addr}");
    tracing::info!("Using conn_str: {conn_str}");


    let db = sqlx::postgres::PgPool::connect(&conn_str)
        .await.expect("Failed to connect to database");

    let app = routes::router()
        .layer(Extension(ServerContext {
            pool: db,
            restore_key: pg_restore_key
        }))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    tracing::info!("Starting server");

    axum::Server::bind(&addr.parse().expect(&format!("DOCKER_TEST_PORT is not valid port ({addr})")))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
