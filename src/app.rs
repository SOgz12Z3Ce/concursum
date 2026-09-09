use crate::render::{index, page, search};
use axum::{Router, routing::get};
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    env::set_current_dir(env::current_exe()?.parent().unwrap())?;

    let app = Router::new()
        .route("/", get(index))
        .route("/{group}/{id}", get(page))
        .route("/search", get(search))
        .nest_service("/static", ServeDir::new("static"));

    const ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(ADDR).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
