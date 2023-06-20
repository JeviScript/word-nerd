use std::sync::OnceLock;

use crate::{account_service::AccountService, db::Db};
use dotenv::dotenv;
use rpc::account::account_server::AccountServer;
use tonic::transport::Server;

mod account_service;
mod auth;
mod db;
mod google;
mod utils;

// global vars
static ENV: OnceLock<Env> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let _ = ENV.set(Env::new());

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<AccountServer<AccountService>>()
        .await;

    let db = Db::new(Env::get().db_connection_uri.clone(), "account").await;

    let service = AccountService::new(db);

    let addr = "0.0.0.0:80".parse()?;

    println!("Account service is listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(AccountServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug)]
struct Env {
    pub db_connection_uri: String,
    pub jwt_secret: String,
}

impl Env {
    // Can panic !
    fn new() -> Self {
        use std::env;
        let to_panic =
            |env_var_name: &str| panic!("Missing requried env variable: {}", env_var_name);
        Env {
            db_connection_uri: env::var("DB_CONNECTION_URI")
                .unwrap_or_else(|_| to_panic("DB_CONNECTION_URI")),

            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| to_panic("JWT_SECRET")),
        }
    }

    fn get() -> &'static Self {
        ENV.get().expect(
            "ENV has to be initiallized at this point.
            Forgot to initialize in the main function?",
        )
    }
}
