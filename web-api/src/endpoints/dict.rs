use crate::{endpoints::web_response, Rpc};
use actix_web::{get, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_word);
}

#[get("dictionary/{word}")]
async fn get_word(req: web::Path<String>, rpc: web::Data<Rpc>) -> HttpResponse {
    let request = tonic::Request::new(rpc::dictionary::HelloRequest {
        name: req.to_string(),
    });

    let response = rpc
        .get_dictionary_connection()
        .await
        .say_hello(request)
        .await
        .unwrap()
        .into_inner()
        .message;

    web_response(HttpResponse::Ok(), response)
}
