use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

use crate::Rpc;

pub fn routes() -> Router {
    Router::new().route("/", get(get_word))
}

#[derive(Deserialize)]
struct WordQuery {
    word: String,
}

async fn get_word(query: Query<WordQuery>) -> impl IntoResponse {
    let mut client = Rpc::get_dictionary_client().await?;

    let request = tonic::Request::new(rpc::dictionary::HelloRequest { name: query.0.word });

    match client.say_hello(request).await {
        Ok(res) => Ok((StatusCode::OK, res.into_inner().message)),
        Err(_status) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
