use axum::Extension;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use hyper::{client::HttpConnector, Body};

pub mod routes;
pub mod types;
pub mod utils;

type Client = hyper::client::Client<HttpConnector, Body>;

#[derive(Clone)]
pub struct ServerContext {
    pub pool: PgPool,
    pub restore_key: String,
    pub postgrest_url: String,
    pub client: Client
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("API_PORT").expect("API_PORT env is not set.");
    let pg_name = std::env::var("API_PG_HOST").expect("API_PG_HOST env is not set.");
    let pg_user = std::env::var("API_PG_USER").expect("API_PG_USER env is not set.");
    let pg_pass = std::env::var("API_PG_PASSWORD").expect("API_PG_PASSWORD env is not set.");
    let pg_port = std::env::var("API_PG_PORT").expect("API_PG_PORT env is not set.");
    let pg_db = std::env::var("API_PG_DATABASE").expect("API_PG_DATABASE env is not set.");
    let pg_restore_key = std::env::var("API_PG_RESTORE_KEY").expect("API_PG_RESTORE_KEY env is not set.");
    let postgrest_url = std::env::var("API_POSTGREST_URL").expect("API_POSTGREST_URL env is not set.");

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
            restore_key: pg_restore_key,
            postgrest_url,
            client: Client::new()
        }))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    tracing::info!("Starting server");

    axum::Server::bind(&addr.parse().expect(&format!("API_PORT is not valid port ({addr})")))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
