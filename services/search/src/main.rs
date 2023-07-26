use crate::env::Env;
use common_rs::EnvStore;
use meilisearch_sdk::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::time::Duration;

mod env;

#[derive(Debug)]
enum Error {
    DbErr(meilisearch_sdk::errors::Error),
    LoadWordsErr,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new(
        Env::vars().meili_connection_uri,
        Some(Env::vars().meili_master_key),
    );

    let indexes = client
        .list_all_indexes()
        .await
        .map_err(Error::DbErr)?
        .results;

    if !indexes.iter().any(|i| i.uid == "words") {
        load_words(&client).await?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Word {
    pub id: usize,
    pub word: String,
}

async fn load_words(client: &Client) -> Result<(), Error> {
    let content = include_str!("../assets/words.json");
    let words: Map<String, Value> =
        serde_json::from_str(content).expect("Could not parse words.json");

    let words: Vec<Word> = words
        .keys()
        .enumerate()
        .map(|(index, key)| Word {
            id: index + 1,
            word: key.clone(),
        })
        .collect();

    let result = client
        .index("words")
        .add_or_replace(&words, None)
        .await
        .map_err(Error::DbErr)?
        .wait_for_completion(client, None, Some(Duration::from_secs(60)))
        .await
        .map_err(Error::DbErr)?;

    if result.is_failure() {
        Err(Error::LoadWordsErr)
    } else {
        println!("Words added to meili successfully");
        Ok(())
    }
}
