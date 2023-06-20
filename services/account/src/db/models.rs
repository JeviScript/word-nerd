use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use super::database::FindOneFilter;

pub enum CollectionName {
    Users,
}

impl From<CollectionName> for &str {
    fn from(c: CollectionName) -> Self {
        match c {
            CollectionName::Users => "users",
        }
    }
}

pub trait DbCollection {
    fn get_collection_name(&self) -> CollectionName;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub google_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl DbCollection for User {
    fn get_collection_name(&self) -> CollectionName {
        CollectionName::Users
    }
}

impl FindOneFilter for User {
    fn find_one_filter(&self) -> Document {
        doc! {"email": &self.email}
    }
}
