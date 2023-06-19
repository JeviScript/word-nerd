use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{bson::Document, Collection};
use mongodb::{Client, Database};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub enum CollectionName {
    GoogleUsers,
}

impl From<CollectionName> for &str {
    fn from(c: CollectionName) -> Self {
        match c {
            CollectionName::GoogleUsers => "google_users",
        }
    }
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

#[derive(Debug)]
pub enum DbErr {
    InvalidDbPath(String, String),
    ClientCreateError(String, String),
    QueryErr(String),
}

pub struct Db {
    db_path: String,
    db_name: String,
}

impl Db {
    pub fn new(db_path: String, db_name: String) -> Db {
        Db { db_path, db_name }
    }

    // Database struct connected
    async fn get_db(&self) -> Result<Database, DbErr> {
        let client_options = ClientOptions::parse(&self.db_path).await.map_err(|err| {
            DbErr::InvalidDbPath(
                format!("Could not parse db_path: {}", self.db_path),
                err.to_string(),
            )
        })?;

        let client = Client::with_options(client_options).map_err(|err| {
            DbErr::ClientCreateError(
                format!("could not create db client with path: {}", &self.db_path),
                err.to_string(),
            )
        })?;

        let db = client.database(&self.db_name);

        Ok(db)
    }

    async fn get_collection<T: Serialize>(
        &self,
        collection_name: CollectionName,
    ) -> Result<Collection<T>, DbErr> {
        let collection = self.get_db().await?.collection::<T>(collection_name.into());
        Ok(collection)
    }

    // inserts if the record does not exist, does nothing otherwise
    pub async fn insert_if_new<
        D: Serialize + FindOneFilter + DbCollection + DeserializeOwned + Send + Sync + Unpin,
    >(
        &self,
        doc: D,
    ) -> Result<(), DbErr> {
        let collection = self.get_collection(doc.get_collection_name()).await?;

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
