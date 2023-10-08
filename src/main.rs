mod _dev_utils;
mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};
pub use config::config; // This way we can use crate::config

use axum::{middleware, Router};
use model::ModelManager;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    _dev_utils::init_dev,
    web::{mw_auth, mw_res_map::mw_reponse_map, routes_login, static_routes},
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Comment this as the lazy clobal statiocs rightfully locks the db
    init_dev().await?;

    //Get the model manager
    let mm = ModelManager::new().await?;

    let routes = Router::new()
        .merge(routes_login::routes())
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            mw_auth::mw_ctx_resolve,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(static_routes::serve_dir());

    let addr = SocketAddr::new(([127, 0, 0, 1]).into(), 8080);

    info!("{:12} - {addr}\n", "LISTENING");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
