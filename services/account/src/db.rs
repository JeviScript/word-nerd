use mongodb::bson::doc;
use mongodb::{bson::Document, Collection};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::AccountService;

pub enum CollectionName {
    GoogleUsers,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleUser {
    // unique per google account
    pub google_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub trait FindOneFilter {
    fn find_one_filter(&self) -> Document;
}

pub trait DbCollection {
    fn get_collection_name(&self) -> CollectionName;
}

impl DbCollection for GoogleUser {
    fn get_collection_name(&self) -> CollectionName {
        CollectionName::GoogleUsers
    }
}

impl FindOneFilter for GoogleUser {
    fn find_one_filter(&self) -> Document {
        doc! {"google_id": &self.google_id}
    }
}

impl From<CollectionName> for &str {
    fn from(c: CollectionName) -> Self {
        match c {
            CollectionName::GoogleUsers => "google_users",
        }
    }
}

impl AccountService {
    pub async fn get_collection<T: Serialize>(
        &self,
        collection_name: CollectionName,
    ) -> Collection<T> {
        self.db.collection::<T>(collection_name.into())
    }

    // inserts if the record does not exist, does nothing otherwise
    pub async fn insert_if_new<
        D: Serialize + FindOneFilter + DbCollection + DeserializeOwned + Send + Sync + Unpin,
    >(
        &self,
        doc: D,
    ) {
        let collection = self.get_collection(doc.get_collection_name()).await;

        let existing = collection
            .find_one(doc.find_one_filter(), None)
            .await
            .expect("Could not fetch from db");

        if let None = existing {
            collection.insert_one(doc, None).await.unwrap();
        }
    }
}
