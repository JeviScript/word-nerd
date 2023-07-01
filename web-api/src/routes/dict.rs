use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::Rpc;

pub fn routes() -> Router {
    Router::new().route("/", get(get_word))
}

async fn get_word(Path(word): Path<String>) -> impl IntoResponse {
    let mut client = Rpc::get_dictionary_client().await?;

    let request = tonic::Request::new(rpc::dictionary::HelloRequest { name: word });

    match client.say_hello(request).await {
        Ok(res) => Ok((StatusCode::OK, res.into_inner().message)),
        Err(_status) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
