use crate::rpc::Rpc;
use routes::api_routes;
use tower_http::cors::CorsLayer;

mod env;
mod middleware;
mod routes;
mod rpc;
mod utils;

#[tokio::main]
async fn main() {
    // TODO tracing
    let app = api_routes().layer(CorsLayer::very_permissive());

    let addr = &"0.0.0.0:80".parse().unwrap();

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
