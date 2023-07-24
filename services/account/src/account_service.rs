use mongodb::bson::doc;
use rpc::account::{
    account_server::Account, AuthRequest, AuthResponse, GoogleSignInRequest, GoogleSignInResponse,
    MeRequest, MeResponse,
};
use tonic::{Request, Response, Status};

use crate::{
    auth::{create_jwt, verify},
    db::{
        models::{CollectionName, User},
        Db,
    },
    google,
};

pub struct AccountService {
    pub db: Db,
}

impl AccountService {
    pub fn new(db: Db) -> AccountService {
        AccountService { db }
    }
}

#[tonic::async_trait]
impl Account for AccountService {
    async fn authenticate(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let token = request.into_inner().token;
        let result = verify(token)
            .map(|_claims| Response::new(AuthResponse { success: true }))
            .unwrap_or_else(|_err| Response::new(AuthResponse { success: false }));

        Ok(result)
    }

    async fn google_sign_in(
        &self,
        request: Request<GoogleSignInRequest>,
    ) -> Result<Response<GoogleSignInResponse>, Status> {
        match google::verify_token(request.into_inner().credential).await {
            Ok(google_user) => {
                let user = User {
                    google_id: google_user.google_id,
                    first_name: google_user.first_name,
                    last_name: google_user.last_name,
                    email: google_user.email,
                };

                if let Err(err) = self.db.insert_or_replace(user.clone()).await {
                    println!("{:?}", err);
                    let err = format!("{:?}", err);
                    return Err(Status::new(tonic::Code::Internal, err));
                }

                let token = create_jwt(&user);
                token
                    .map(|t| Response::new(GoogleSignInResponse { token: t }))
                    .map_err(|err| Status::new(tonic::Code::Internal, format!("{:?}", err)))
            }
            Err(err) => {
                println!("{:?}", err);
                let err = format!("{:?}", err);
                return Err(Status::new(tonic::Code::Internal, err));
            }
        }
    }

    async fn me(&self, request: Request<MeRequest>) -> Result<Response<MeResponse>, Status> {
        let token = request.into_inner().token;
        let claims = verify(token).map_err(|e| Status::unauthenticated(format!("{:?}", e)))?;

        let user = self
            .db
            .get_collection::<User>(CollectionName::Users)
            .await
            .find_one(doc! {"google_id": claims.sub}, None)
            .await
            .map_err(|_| Status::internal(""))?
            .ok_or(Status::not_found(""))?;

        let response = Response::new(MeResponse {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        });

        Ok(response)
    }
}
