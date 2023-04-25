use mongodb::{options::ClientOptions, Client, Database};
use rpc::account::{
    account_server::{Account, AccountServer},
    AuthRequest, AuthResponse, GoogleSignInRequest, GoogleSignInResponse,
};

use tonic::{transport::Server, Request, Response, Status};

mod db;
mod google;

pub struct AccountService {
    pub db: Database,
}

impl AccountService {
    fn new(db: Database) -> AccountService {
        AccountService { db }
    }
}

#[tonic::async_trait]
impl Account for AccountService {
    async fn authenticate(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        println!("{}", request.into_inner().token);
        Ok(Response::new(AuthResponse { success: true }))
    }

    async fn google_sign_in(
        &self,
        request: Request<GoogleSignInRequest>,
    ) -> Result<Response<GoogleSignInResponse>, Status> {
        match google::verify_token(request.into_inner().credential).await {
            Some(google_user) => {
                self.insert_if_new(google_user).await;
                Ok(Response::new(GoogleSignInResponse { success: true }))
            }
            None => Ok(Response::new(GoogleSignInResponse { success: false })),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<AccountServer<AccountService>>()
        .await;

    let db_path = "mongodb://root:root@db:27017";

    let client_options = ClientOptions::parse(db_path)
        .await
        .expect(format!("could not parse db_path: {}", db_path).as_str());

    let client = Client::with_options(client_options)
        .expect(format!("could not create db client with path: {}", db_path).as_str());

    let db = client.database("account");

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
