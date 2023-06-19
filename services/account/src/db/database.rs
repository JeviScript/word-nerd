use mongodb::options::ClientOptions;
use mongodb::{bson::Document, Collection};
use mongodb::{Client, Database};
use serde::{de::DeserializeOwned, Serialize};

use super::models::{CollectionName, DbCollection};

pub trait FindOneFilter {
    fn find_one_filter(&self) -> Document;
}

#[derive(Debug)]
pub enum DbErr {
    QueryErr(String),
}

pub struct Db {
    db: Database,
}

impl Db {
    pub async fn new(db_path: String, db_name: &str) -> Db {
        let client_options = ClientOptions::parse(&db_path)
            .await
            .unwrap_or_else(|err| panic!("Could not parse db_path: {}. Err: {}", &db_path, err));

        let client = Client::with_options(client_options).unwrap_or_else(|err| {
            panic!(
                "Could not create client with db_path: {}, Err: {}",
                &db_path, err
            )
        });

        let db = client.database(db_name);
        Db { db }
    }

    async fn get_collection<T: Serialize>(&self, collection_name: CollectionName) -> Collection<T> {
        self.db.collection::<T>(collection_name.into())
    }

    // inserts if the record does not exist, does nothing otherwise
    pub async fn insert_if_new<
        D: Serialize + FindOneFilter + DbCollection + DeserializeOwned + Send + Sync + Unpin,
    >(
        &self,
        doc: D,
    ) -> Result<(), DbErr> {
        let collection = self.get_collection(doc.get_collection_name()).await;

        let existing = collection
            .find_one(doc.find_one_filter(), None)
            .await
            .map_err(|err| DbErr::QueryErr(format!("Could not fetch from db: {}", err)))?;

        if existing.is_none() {
            collection.insert_one(doc, None).await.unwrap();
        }

        Ok(())
    }
}
