mod ctx;
mod error;
mod log;
mod model;
mod web;

use axum::{middleware, Router};
use model::ModelManager;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

use crate::{
    error::Result,
    web::{mw_auth, mw_res_map::mw_reponse_map, static_routes},
};

#[tokio::main]
async fn main() -> Result<()> {
    //Get the model manager
    let mm = ModelManager::new().await?;

    let routes = Router::new()
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(
            mm.clone(),
            mw_auth::mw_ctx_resolve,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(static_routes::serve_dir());

    let addr = SocketAddr::new(([127, 0, 0, 1]).into(), 8080);

    println!("->>{:12} - {addr}\n", "LISTENING");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
