use crate::env::Env;
use axum::http::StatusCode;
use common_rs::EnvStore;
use rpc::{
    account::account_client::AccountClient, dictionary::dictionary_client::DictionaryClient,
};
use std::sync::OnceLock;
use tonic::transport::Channel;

static RPC: OnceLock<Rpc> = OnceLock::new();

#[derive(Clone)]
pub struct Rpc {
    account_service_url: String,
    dictionary_service_url: String,

    // attempt to reuse clients if possible
    account_client: OnceLock<AccountClient<Channel>>,
    dict_client: OnceLock<DictionaryClient<Channel>>,
}

impl Rpc {
    pub fn new() -> Rpc {
        Rpc {
            account_service_url: Env::vars().account_service_uri,
            dictionary_service_url: Env::vars().dict_service_uri,
            account_client: OnceLock::new(),
            dict_client: OnceLock::new(),
        }
    }

    fn get() -> &'static Rpc {
        if RPC.get().is_none() {
            _ = RPC.set(Self::new());
        }

        RPC.get().expect("Should be set")
    }

    pub async fn get_account_client() -> Result<AccountClient<Channel>, StatusCode> {
        if let Some(client) = Self::get().account_client.get() {
            Ok(client.clone())
        } else {
            let url = &Self::get().account_service_url;
            let client = AccountClient::connect(url.clone())
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
            if let Ok(client) = client.clone() {
                _ = Self::get().account_client.set(client);
            }
            client
        }
    }

    pub async fn get_dictionary_client() -> Result<DictionaryClient<Channel>, StatusCode> {
        if let Some(client) = Self::get().dict_client.get() {
            Ok(client.clone())
        } else {
            let url = &Self::get().dictionary_service_url;
            let client = DictionaryClient::connect(url.clone())
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
            if let Ok(client) = client.clone() {
                _ = Self::get().dict_client.set(client);
            }
            client
        }
    }
}
