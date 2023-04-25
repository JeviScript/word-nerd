use rpc::dictionary::{
    dictionary_server::{Dictionary, DictionaryServer},
    HelloReply, HelloRequest,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct DictionaryService {}

#[tonic::async_trait]
impl Dictionary for DictionaryService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}! from dictionary-ms", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<DictionaryServer<DictionaryService>>()
        .await;

    let addr = "0.0.0.0:80".parse()?;
    let service = DictionaryService::default();

    Server::builder()
        .add_service(health_service)
        .add_service(DictionaryServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
