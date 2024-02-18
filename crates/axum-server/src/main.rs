use std::net::SocketAddr;

use config::Config;
use once_cell::sync::Lazy;

use axum::{extract::Request, middleware::Next, response::IntoResponse};
use axum::{
    body::{Body, Bytes},
    http::StatusCode,
    response::Response,
};

use http_body_util::BodyExt;

use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

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
        ip: config.get("ip").unwrap(),
        http_port: config.get("http_port").unwrap(),
        submission_id: config.get("submission_id").unwrap(),
    })
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600)));

    let app = axum::Router::new()
        .nest_service("/", ServeDir::new("static"))
        .nest("/connect.php", connect_router::new_connect_router())
        .nest(
            "/battleship.php",
            battleship_router::new_battleship_router(),
        )
        .nest("/ttt.php", ttt_router::new_ttt_router())
        .layer(axum::middleware::from_fn(append_headers))
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(print_request_response))
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

async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    //let (parts, body) = res.into_parts();
    //let bytes = buffer_and_print("response", body).await?;
    //let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
