use crate::rpc::Rpc;
use env::Env;
use routes::api_routes;
use std::sync::OnceLock;
use tower_http::cors::CorsLayer;

mod env;
mod middleware;
mod routes;
mod rpc;
mod utils;

// global vars
static ENV: OnceLock<Env> = OnceLock::new();
static RPC: OnceLock<Rpc> = OnceLock::new();

#[tokio::main]
async fn main() {
    // TODO tracing

    Rpc::init(
        Env::vars().account_service_uri,
        Env::vars().dict_service_uri,
    );

    let app = api_routes().layer(CorsLayer::very_permissive());

    let addr = &"0.0.0.0:80".parse().unwrap();

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
