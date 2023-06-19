use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};

use super::database::FindOneFilter;

pub enum CollectionName {
    GoogleUsers,
    Auth,
}

impl From<CollectionName> for &str {
    fn from(c: CollectionName) -> Self {
        match c {
            CollectionName::GoogleUsers => "google_users",
            CollectionName::Auth => "auth",
        }
    }
}

pub trait DbCollection {
    fn get_collection_name(&self) -> CollectionName;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleUser {
    // unique per google account
    pub google_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    // unique per google account
    pub user_id: String,
    pub token: String,
    pub valid_until: DateTime,
}

impl DbCollection for Auth {
    fn get_collection_name(&self) -> CollectionName {
        CollectionName::Auth
    }
}
