use mongodb::options::ClientOptions;
use mongodb::bson::Document;
use mongodb::{Client, Database};


pub trait FindOneFilter {
    fn find_one_filter(&self) -> Document;
}

#[derive(Debug)]
pub enum DbErr {
    QueryErr(mongodb::error::Error),
    ParseBsonErr(mongodb::bson::oid::Error),
    Unexpected
}

impl From<DbErr> for String {
    fn from(value: DbErr) -> Self {
        format!("{:?}", value)
    }
}

#[derive(Debug)]
pub struct Db {
    pub db: Database,
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

}
