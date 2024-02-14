use std::net::SocketAddr;
use axum::{
    extract::Request, 
    middleware::Next, 
    response::IntoResponse
};

pub mod routers;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .nest("/connect.php", routers::connect_router::new_connect_router())
        .layer(axum::middleware::from_fn(append_headers));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn append_headers(
    request: Request,
    next: Next
) -> impl IntoResponse {
    let mut response = next.run(request).await; 
    let headers = response.headers_mut(); 
    let submission_id = "65b54162aa2cfc5a3dea55fe";
    headers.insert("x-cse356", submission_id.parse().unwrap());
    response  
}
