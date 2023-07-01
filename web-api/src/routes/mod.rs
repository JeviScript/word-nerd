use crate::middleware::auth_guard;
use axum::{middleware, Router};

mod auth;
mod dict;

pub fn api_routes() -> Router {
    Router::new()
        .nest("/", protected_routes())
        .nest("/auth", auth::routes())
}

fn protected_routes() -> Router {
    Router::new()
        .nest("/dict", dict::routes())
        .route_layer(middleware::from_fn(auth_guard))
}
