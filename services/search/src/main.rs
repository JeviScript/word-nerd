use crate::env::Env;
use common_rs::EnvStore;
use meilisearch_sdk::Client;

mod env;

#[tokio::main]
async fn main() {
    let client = Client::new(
        Env::vars().meili_connection_uri,
        Some(Env::vars().meili_master_key),
    );

    let words = client.index("words");

    println!("Hello, world!");
}
