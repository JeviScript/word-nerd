use db::Db;
use rpc::account::{
    account_server::{Account, AccountServer},
    AuthRequest, AuthResponse, GoogleSignInRequest, GoogleSignInResponse,
};

use tonic::{transport::Server, Request, Response, Status};

mod db;
mod google;

pub struct AccountService {
    pub db: Db,
}

impl AccountService {
    fn new(db_path: String, db_name: String) -> AccountService {
        AccountService {
            db: Db::new(db_path, db_name),
        }
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
            Ok(google_user) => {
                if let Err(err) = self.db.insert_if_new(google_user).await {
                    println!("{:?}", err);
                    return Ok(Response::new(GoogleSignInResponse { success: false }));
                }
                Ok(Response::new(GoogleSignInResponse { success: true }))
            }
            Err(err) => {
                println!("{:?}", err);
                Ok(Response::new(GoogleSignInResponse { success: false }))
            }
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

    let service = AccountService::new(db_path.to_owned(), "account".to_owned());

    let addr = "0.0.0.0:80".parse()?;

    println!("Account service is listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(AccountServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
