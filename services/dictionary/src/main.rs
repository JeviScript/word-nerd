use crate::env::Env;
use db::{models::{Definition, VocabularyWord}, Db, repository::Repository};
use rpc::dictionary::{
    dictionary_server::{Dictionary, DictionaryServer},
    GetWordDefinitionsReply, GetWordDefinitionsRequest,
};
use std::sync::OnceLock;
use tonic::{transport::Server, Request, Response, Status};

mod cloudflare_bypasser;
mod db;
mod env;
mod vocabulary;

// global vars
static ENV: OnceLock<Env> = OnceLock::new();

#[derive(Debug)]
pub struct DictionaryService {
    pub db: Db,
    pub repository: Repository
}

impl DictionaryService {
    pub fn new(db: Db, repository: Repository) -> DictionaryService {
        DictionaryService { db, repository }
    }
}

#[tonic::async_trait]
impl Dictionary for DictionaryService {
    async fn get_word_definitions(
        &self,
        request: Request<GetWordDefinitionsRequest>,
    ) -> Result<Response<GetWordDefinitionsReply>, Status> {
        let word = request.into_inner().word;
        // TODO CQRS ??
        let voc = vocabulary::scrape(word.as_str())
            .await
            .map_err(|e| {
                println!("{:?}", e);
                e
            })
            .unwrap_or_default();

        let pronunciations = self.repository.replace_vocabulary_audio(word.to_string(), voc.clone().pronunciations).await;

        if let Err(_e) = pronunciations {
            return Ok(Response::new(GetWordDefinitionsReply { success: false }));
        }

        let pronunciations = pronunciations.unwrap();

        let voc = VocabularyWord::new(voc, pronunciations);

        let definition = Definition::new(word, voc);

        self.repository.replace_definition(definition).await
            .map_err(|e| Status::internal(format!("{:?}", e)))?;

        Ok(Response::new(GetWordDefinitionsReply { success: true }))
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

    let service = DictionaryService::new(db, repository);

    let addr = "0.0.0.0:80".parse()?;
    Server::builder()
        .add_service(health_service)
        .add_service(DictionaryServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
