use axum::http::StatusCode;
use routes::api_routes;
use rpc::{
    account::account_client::AccountClient, dictionary::dictionary_client::DictionaryClient,
};
use std::env;
use std::sync::OnceLock;
use tonic::transport::Channel;
use tower_http::cors::CorsLayer;

mod middleware;
mod routes;
mod utils;

// global vars
static ENV: OnceLock<Env> = OnceLock::new();
static RPC: OnceLock<Rpc> = OnceLock::new();

#[derive(Clone)]
pub struct Rpc {
    account_service_url: String,
    dictionary_service_url: String,

    // attempt to reuse clients if possible
    account_client: Option<AccountClient<Channel>>,
    dict_client: Option<DictionaryClient<Channel>>,
}

impl Rpc {
    pub fn init(account_url: String, dict_url: String) -> Rpc {
        let rpc = Rpc {
            account_service_url: account_url,
            dictionary_service_url: dict_url,
            account_client: None,
            dict_client: None,
        };

        _ = RPC.set(rpc.clone());
        rpc
    }

    fn get() -> &'static Rpc {
        RPC.get()
            .unwrap_or_else(|| panic!("Had to be initialized before used!"))
    }

    async fn get_account_client() -> Result<AccountClient<Channel>, StatusCode> {
        if let Some(ref client) = Self::get().account_client {
            return Ok(client.clone());
        }

        let url = &Self::get().account_service_url;
        AccountClient::connect(url.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn get_dictionary_client() -> Result<DictionaryClient<Channel>, StatusCode> {
        if let Some(ref client) = Self::get().dict_client {
            return Ok(client.clone());
        }
        let url = &Self::get().dictionary_service_url;
        DictionaryClient::connect(url.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[tokio::main]
async fn main() {
    // TODO tracing

    Rpc::init(
        Env::vars().account_service_uri,
        Env::vars().dict_service_uri,
    );

    let app = api_routes().layer(CorsLayer::very_permissive());

    let addr = &"0.0.0.0:80".parse().unwrap();

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
struct Env {
    pub account_service_uri: String,
    pub dict_service_uri: String,
}

// TODO reuse logic for setting env variables with services somehow
impl Env {
    // Can panic !
    fn init() -> Self {
        let env = Env {
            account_service_uri: Env::required("ACCOUNT_SERVICE_URI"),
            dict_service_uri: Env::required("DICTIONARY_SERVICE_URI"),
        };

        _ = ENV.set(env.clone());
        env
    }

    fn required(env_var: &str) -> String {
        env::var(env_var).unwrap_or_else(|_| panic!("Missing required env variable: {}", env_var))
    }

    fn vars() -> Self {
        match ENV.get() {
            Some(val) => val.clone(),
            None => Self::init(),
        }
    }
}
