use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use rpc::{
    account::account_client::AccountClient, dictionary::dictionary_client::DictionaryClient,
};
use tonic::transport::Channel;

mod endpoints;

#[derive(Clone)]
struct Rpc {
    account_service_url: String,
    dictionary_service_url: String,
}

impl Rpc {
    async fn get_account_connection(&self) -> AccountClient<Channel> {
        let url = &self.account_service_url;
        AccountClient::connect(url.clone())
            .await
            .expect(format!("failed connect to {}", url).as_str())
    }

    async fn get_dictionary_connection(&self) -> DictionaryClient<Channel> {
        let url = &self.dictionary_service_url;
        DictionaryClient::connect(url.clone())
            .await
            .expect(format!("failed connect to {}", url).as_str())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let rpc = Rpc {
        account_service_url: format!("http://account"),
        dictionary_service_url: format!("http://dictionary"),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(rpc.clone()))
            .service(web::scope("").configure(endpoints::config))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
