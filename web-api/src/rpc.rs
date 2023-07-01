use axum::http::StatusCode;
use rpc::{
    account::account_client::AccountClient, dictionary::dictionary_client::DictionaryClient,
};
use tonic::transport::Channel;

use crate::RPC;

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

    pub async fn get_account_client() -> Result<AccountClient<Channel>, StatusCode> {
        if let Some(ref client) = Self::get().account_client {
            return Ok(client.clone());
        }

        let url = &Self::get().account_service_url;
        AccountClient::connect(url.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub async fn get_dictionary_client() -> Result<DictionaryClient<Channel>, StatusCode> {
        if let Some(ref client) = Self::get().dict_client {
            return Ok(client.clone());
        }
        let url = &Self::get().dictionary_service_url;
        DictionaryClient::connect(url.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
