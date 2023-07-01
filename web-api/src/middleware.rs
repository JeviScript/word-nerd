use axum::middleware::Next;
use axum::{
    extract::TypedHeader,
    headers::authorization::{Authorization, Bearer},
    http::Request,
    http::StatusCode,
    response::Response,
};
use rpc::account::account_client::AccountClient;
use tonic::transport::Channel;

use crate::Rpc;

pub async fn auth_guard<B>(
    // run the `TypedHeader` extractor
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let client = Rpc::get_account_client().await?;

    if token_is_valid(auth.token(), client).await {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn token_is_valid(token: &str, mut account_client: AccountClient<Channel>) -> bool {
    let request = tonic::Request::new(rpc::account::AuthRequest {
        token: token.to_string(),
    });

    match account_client.authenticate(request).await {
        Ok(res) => res.into_inner().success,
        Err(_status) => false,
    }
}
