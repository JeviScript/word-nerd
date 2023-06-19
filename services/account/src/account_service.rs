use rpc::account::{
    account_server::Account, AuthRequest, AuthResponse, GoogleSignInRequest, GoogleSignInResponse,
};
use tonic::{Request, Response, Status};

use crate::{db::Db, google};

pub struct AccountService {
    pub db: Db,
}

impl AccountService {
    pub fn new(db: Db) -> AccountService {
        AccountService { db }
    }

    pub fn gen_token(&self) -> String {
        todo!()
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
                    let err = format!("{:?}", err);
                    return Err(Status::new(tonic::Code::Internal, err));
                }

                let token = self.gen_token();
                Ok(Response::new(GoogleSignInResponse { token }))
            }
            Err(err) => {
                println!("{:?}", err);
                let err = format!("{:?}", err);
                return Err(Status::new(tonic::Code::Internal, err));
            }
        }
    }
}
