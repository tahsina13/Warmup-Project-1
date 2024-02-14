use axum::{
    body::Body,
    extract::{Host, Query, Request},
    handler::HandlerWithoutStateExt,
    http::{HeaderMap, HeaderName, HeaderValue, Method, Response, StatusCode, Uri},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    BoxError, Router,
};
use axum_server::tls_rustls::RustlsConfig;

use chrono::{DateTime, Local, Utc};
use config::Config;

use once_cell::sync::Lazy;

use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

use std::net::SocketAddr;
use std::path::PathBuf;

use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[derive(Debug)]
struct ServerConfig {
    ip: [u8; 4],
    http_port: u16,
    https_port: u16,
    submission_id: String,
}

static CONFIG: Lazy<ServerConfig> = Lazy::new(|| {
    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    dbg!(ServerConfig {
        ip: config.get::<[u8; 4]>("ip").unwrap(),
        submission_id: config.get_string("submission_id").unwrap(),
        http_port: config.get::<u16>("http_port").unwrap(),
        https_port: config.get::<u16>("https_port").unwrap(),
    })
});

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let ports = Ports {
        http: CONFIG.http_port,
        https: CONFIG.https_port,
    };

    tokio::join!(serve(ports));
}

async fn serve(ports: Ports) {

    let app = Router::new()
        //.nest_service("/assets", ServeDir::new("assets"))
        .route_service("/ttt.php", get(ttt_php))
        .route_service("/ttt.css", get(ttt_css));
        //.route_service("/hw0/:p1/:p2", serve_hw0);

    let addr = SocketAddr::from((CONFIG.ip, ports.http));
    tracing::debug!("server listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// https://github.com/tokio-rs/axum/blob/main/examples/query-params-with-empty-strings/src/main.rs

#[derive(Debug, Deserialize)]
struct NameParam {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    name: Option<String>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}


async fn ttt_php(Query(params): Query<NameParam>, req: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-CSE356",
        HeaderValue::from_str(CONFIG.submission_id.as_str())
            .expect("Failed to convert submission ID to HeaderValue"),
    );

    tracing::debug!("{:?}", req);
    tracing::debug!("{:?}", params);

    let html_body =
    if req.method() == Method::GET && params.name.is_some() {
        let current_local: DateTime<Local> = Local::now();
        let time_formatted = current_local.format("%m/%d/%Y");
        format!("Hello {}, {}", params.name.unwrap(), time_formatted)
    } else {
        r#"
    <form action="ttt.php" method="get">
    <label for="name">Name:</label>
    <input type="text" id="name" name="name"/>
    <input type="submit" value="Submit"/>
    </form>
"#.to_owned()
    };

    let body = Html(format!(r#"
<!DOCTYPE html>
<html>
<head>
    <link ref="stylesheet" href="ttt.css"/>
<head>
<body>
{html_body}
</body>
</html>
"#));
    (headers, body)
}

async fn ttt_css() -> &'static str {
    r#"
body {
    background-color: #b0a0f0;
}
}
"#
}
