use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use super::database::FindOneFilter;

// TODO explore type safety for field names when doing filtering
/*  e.g.
    let filter = User::filter();
    filter.name = "Hoid";
    User::update(filter);
*/

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

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
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
