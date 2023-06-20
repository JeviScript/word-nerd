use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

use crate::{endpoints::web_response, Rpc};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_account).service(post_login);
}

#[get("account/{friend}")]
async fn get_account(req: web::Path<String>) -> HttpResponse {
    web_response(HttpResponse::Ok(), req.to_string())
}

#[derive(Deserialize)]
struct Login {
    credential: String,
}

#[post("login")]
async fn post_login(req: web::Json<Login>, rpc: web::Data<Rpc>) -> HttpResponse {
    let google_request = tonic::Request::new(rpc::account::GoogleSignInRequest {
        credential: req.credential.clone(),
    });

    let res = rpc
        .get_account_connection()
        .await
        .google_sign_in(google_request)
        .await
        .unwrap()
        .into_inner()
        .token;

    web_response(HttpResponse::Ok(), res.to_string())
}
