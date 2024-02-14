use std::net::SocketAddr;

pub mod routers;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .nest("/connect", routers::connect_router::new_connect_router());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
