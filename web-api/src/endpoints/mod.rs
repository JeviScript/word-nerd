use actix_web::{http::header::ContentType, web, HttpResponse, HttpResponseBuilder};
use serde::Serialize;

mod auth;
mod dict;

pub fn web_response(mut response: HttpResponseBuilder, r: impl Serialize) -> HttpResponse {
    response.content_type(ContentType::json());
    response.json(r)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("auth").configure(auth::config))
        .service(web::scope("health").route("", web::get().to(|| HttpResponse::Ok())))
        .service(web::scope("dict").configure(dict::config));
}
