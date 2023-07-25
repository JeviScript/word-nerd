use crate::{account_service::AccountService, db::Db};
use common_rs::{env, EnvStore};
use dotenv::dotenv;
use rpc::account::account_server::AccountServer;
use std::sync::OnceLock;
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

    let db = Db::new(Env::vars().db_connection_uri.clone(), "account").await;

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

#[derive(Debug, Clone)]
pub struct Env {
    pub db_connection_uri: String,
    pub jwt_secret: String,
}

impl EnvStore for Env {
    fn new() -> Self {
        Env {
            db_connection_uri: env::required("DB_CONNECTION_URI"),
            jwt_secret: env::required("JWT_SECRET"),
        }
    }
}
