use crate::Rpc;
use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

pub fn routes() -> Router {
    Router::new().route("/login", post(post_login))
}

#[derive(Deserialize)]
struct PostLoginReq {
    credential: String,
}

#[derive(Serialize)]
struct PostLoginRes {
    token: String,
}

async fn post_login(Json(req): Json<PostLoginReq>) -> impl IntoResponse {
    let google_request = tonic::Request::new(rpc::account::GoogleSignInRequest {
        credential: req.credential.clone(),
    });

    let mut client = Rpc::get_account_client().await?;

    match client.google_sign_in(google_request).await {
        Ok(res) => Ok((
            StatusCode::OK,
            Json(PostLoginRes {
                token: res.into_inner().token,
            }),
        )),
        Err(_status) => Err(StatusCode::UNAUTHORIZED),
    }
}
