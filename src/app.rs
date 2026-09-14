use crate::{
    data::cs::{Data, DataView},
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

#[derive(Debug)]
pub(crate) struct Resource<'a> {
    pub(crate) data_view: DataView<'a>,
    pub(crate) search_engine: SearchEngine,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let bin_dir = env::current_exe()?;
    let base_dir = bin_dir
        .parent()
        .expect("bin is expected to have a parent directory");
    let static_dir = base_dir.join("static");
    let favicon_path = static_dir.join("images/favicon.ico");

    let data = Box::new(Data::load(&base_dir)?);
    let data = Box::leak(data);
    let data_view = data.view()?;
    let search_engine = search::index(&data_view)?;
    let state = Arc::new(Resource {
        data_view,
        search_engine,
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
