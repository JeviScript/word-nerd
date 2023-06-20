use mongodb::options::{ClientOptions, ReplaceOptions};
use mongodb::{bson::Document, Collection};
use mongodb::{Client, Database};
use serde::{de::DeserializeOwned, Serialize};

use super::models::{CollectionName, DbCollection};

pub trait FindOneFilter {
    fn find_one_filter(&self) -> Document;
}

#[derive(Debug)]
pub enum DbErr {
    QueryErr(mongodb::error::Error),
}

pub struct Db {
    db: Database,
}

impl Db {
    pub async fn new(db_connection_uri: String, db_name: &str) -> Db {
        let client_options = ClientOptions::parse(&db_connection_uri)
            .await
            .unwrap_or_else(|err| {
                panic!(
                    "Could not parse db_connection_uri: {}. Err: {}",
                    &db_connection_uri, err
                )
            });

        let client = Client::with_options(client_options).unwrap_or_else(|err| {
            panic!(
                "Could not create client with db_connection_uri: {}, Err: {}",
                &db_connection_uri, err
            )
        });

        let db = client.database(db_name);
        Db { db }
    }

    async fn get_collection<T: Serialize>(&self, collection_name: CollectionName) -> Collection<T> {
        self.db.collection::<T>(collection_name.into())
    }

    /// inserts if the record does not exist, replaces otherwise
    pub async fn insert_or_replace<
        Doc: Serialize + FindOneFilter + DbCollection + DeserializeOwned + Send + Sync + Unpin,
    >(
        &self,
        doc: Doc,
    ) -> Result<(), DbErr> {
        let collection = self.get_collection(doc.get_collection_name()).await;

        let mut replace_options = ReplaceOptions::default();
        // inserts when finds None
        replace_options.upsert = Some(true);
        collection
            .replace_one(doc.find_one_filter(), doc, replace_options)
            .await
            .map_err(DbErr::QueryErr)?;

        Ok(())
    }
}
