use crate::{middleware::auth_guard, rpc::Rpc};
use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
    Json, Router, TypedHeader,
};
use serde::Serialize;

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
        .route("/me", get(get_me))
        .route_layer(middleware::from_fn(auth_guard))
}

#[derive(Serialize, Debug)]
struct GetMeRes {
    first_name: String,
    last_name: String,
    email: String,
}

async fn get_me(TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> impl IntoResponse {
    let request = tonic::Request::new(rpc::account::MeRequest {
        token: auth.token().to_string(),
    });

    let mut client = Rpc::get_account_client().await?;

    match client.me(request).await {
        Ok(res) => {
            let res = res.into_inner();
            Ok((
                StatusCode::OK,
                Json(GetMeRes {
                    first_name: res.first_name,
                    last_name: res.last_name,
                    email: res.email,
                }),
            ))
        }
        Err(_status) => Err(StatusCode::UNAUTHORIZED),
    }
}
