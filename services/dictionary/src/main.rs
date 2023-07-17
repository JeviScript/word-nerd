use crate::env::Env;
use db::{repository::Repository, Db};
use rpc::dictionary::{
    dictionary_server::{Dictionary, DictionaryServer},
    GetWordDefinitionsRequest, GetWordDefinitionsResponse, InvalidateWordRequest,
    InvalidateWordResponse, GetAudioRequest, GetAudioResponse,
};
use std::sync::OnceLock;
use tonic::{transport::Server, Request, Response, Status};

mod cloudflare_bypasser;
mod db;
mod env;
mod service;
mod vocabulary;

// global vars
static ENV: OnceLock<Env> = OnceLock::new();

#[derive(Debug)]
pub struct DictionaryService {
    pub repository: Repository,
}

impl DictionaryService {
    pub fn new(repository: Repository) -> DictionaryService {
        DictionaryService { repository }
    }
}

#[tonic::async_trait]
impl Dictionary for DictionaryService {
    async fn get_word_definitions(
        &self,
        request: Request<GetWordDefinitionsRequest>,
    ) -> Result<Response<GetWordDefinitionsResponse>, Status> {
        let word = request.into_inner().word;

        let response = self.get_word_definitions(word).await;

        match response {
            Ok(val) => Ok(Response::new(val)),
            Err(err) => {
                println!("{:?}", err);
                let err = format!("{:?}", err);
                return Err(Status::new(tonic::Code::Internal, err));
            }
        }
    }

    async fn invalidate_word(
        &self,
        request: Request<InvalidateWordRequest>,
    ) -> Result<Response<InvalidateWordResponse>, Status> {
        let word = request.into_inner().word;

        match self.invalidate_word(word).await {
            Ok(_) => Ok(Response::new(InvalidateWordResponse { success: true })),
            Err(_) => Ok(Response::new(InvalidateWordResponse { success: false })),
        }
    }

    async fn get_audio(
        &self,
        request: Request<GetAudioRequest>
    ) -> Result<Response<GetAudioResponse>, Status> {
        let id = request.into_inner().id;

        let audio = self.get_audio(id.clone()).await.map_err(Status::internal)?;

        match audio {
            Some(a) => Ok(Response::new(a)),
            None => Err(Status::not_found(format!("Not found: {}", id)))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<DictionaryServer<DictionaryService>>()
        .await;

    Env::init();

    let db = Db::new(Env::vars().db_connection_uri, "dictionary").await;
    let repository = Repository::new(&db.db);

    let service = DictionaryService::new(repository);

    let addr = "0.0.0.0:80".parse()?;
    Server::builder()
        .add_service(health_service)
        .add_service(DictionaryServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
