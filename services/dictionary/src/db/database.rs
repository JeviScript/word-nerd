use mongodb::options::ClientOptions;
use mongodb::{Client, Database};

#[derive(Debug)]
pub enum DbErr {
    QueryErr(mongodb::error::Error),
    ParseBsonErr(mongodb::bson::oid::Error),
    Unexpected,
}

impl From<DbErr> for String {
    fn from(value: DbErr) -> Self {
        format!("{:?}", value)
    }
}

pub async fn get_database_client(db_connection_uri: String, db_name: &str) -> Database {
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

    client.database(db_name)
}
