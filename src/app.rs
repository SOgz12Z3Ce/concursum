use crate::render::{index, page, search};
use axum::{Router, routing::get};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/{group}/{id}", get(page))
        .route("/search", get(search))
        .nest_service("/static", ServeDir::new("static"));

    const ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    let listener = TcpListener::bind(ADDR).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
