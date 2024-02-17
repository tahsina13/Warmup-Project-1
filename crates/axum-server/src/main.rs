use std::net::SocketAddr;

use config::Config;
use once_cell::sync::Lazy;

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};

pub mod routers;
use routers::*; 

#[derive(Debug)]
struct ServerConfig {
    ip: [u8; 4],
    http_port: u16,
    submission_id: String,
}

static CONFIG: Lazy<ServerConfig> = Lazy::new(|| {
    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    dbg!(ServerConfig {
        ip: config.get::<[u8; 4]>("ip").unwrap(),
        http_port: config.get::<u16>("http_port").unwrap(),
        submission_id: config.get_string("submission_id").unwrap(),
    })
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let app = axum::Router::new()
        .nest_service("/", ServeDir::new("static"))
        .nest("/connect.php", connect_router::new_connect_router())
        .nest("/battleship.php", battleship_router::new_battleship_router())
        .layer(axum::middleware::from_fn(append_headers))
        .layer(session_layer);

    let addr = SocketAddr::from((CONFIG.ip, CONFIG.http_port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("Server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn append_headers(request: Request, next: Next) -> impl IntoResponse {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-cse356", CONFIG.submission_id.parse().unwrap());
    response
}
