use crate::{account_service::AccountService, db::Db};
use rpc::account::account_server::AccountServer;
use tonic::transport::Server;

mod account_service;
mod db;
mod google;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<AccountServer<AccountService>>()
        .await;

    let db_path = "mongodb://root:root@db:27017".to_owned();

    let db = Db::new(db_path, "account").await;

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
