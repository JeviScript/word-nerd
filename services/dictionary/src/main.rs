use crate::env::Env;
use db::{models::Definition, Db};
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
}

impl DictionaryService {
    pub fn new(db: Db) -> DictionaryService {
        DictionaryService { db }
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

        let definition = Definition {
            word: word.to_string(),
            vocabulary: voc,
            ..Default::default()
        };

        self.db
            .insert_or_replace(definition)
            .await
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

    let service = DictionaryService::new(db);

    let addr = "0.0.0.0:80".parse()?;
    Server::builder()
        .add_service(health_service)
        .add_service(DictionaryServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
