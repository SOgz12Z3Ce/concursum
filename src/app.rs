use crate::{
    data::{self, Data},
    page::{index, object, search},
    search::{self, SearchEngine},
};
use axum::{Router, routing};
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, Clone)]
pub(crate) struct Resource {
    pub(crate) cs_data: Data,
    pub(crate) cs_index: SearchEngine,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let bin_dir = env::current_exe()?;
    let base_dir = bin_dir
        .parent()
        .expect("bin is expected to have a parent directory");
    let static_dir = base_dir.join("static");
    let favicon_path = static_dir.join("images/favicon.ico");

    let cs_data = data::load(&base_dir);
    let cs_index = search::index(&cs_data);
    let state = Arc::new(Resource {
        cs_data: cs_data,
        cs_index,
    });

    let service = Router::new()
        .route("/", routing::get(index))
        .route_service("/favicon.ico", ServeFile::new(&favicon_path))
        .nest_service("/static", ServeDir::new(&static_dir))
        .route("/cs/{group}/{id}", routing::get(object))
        .route("/search", routing::get(search))
        .with_state(state);

    // TODO: Make these configurable.
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 8080;
    let addr = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, service).await?;

    Ok(())
}
