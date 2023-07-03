use crate::vocabulary;

use super::database::FindOneFilter;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

// TODO explore type safety for field names when doing filtering
/*  e.g.
    let filter = User::filter();
    filter.name = "Hoid";
    User::update(filter);
*/

pub enum CollectionName {
    Definitions,
}

impl From<CollectionName> for &str {
    fn from(c: CollectionName) -> Self {
        match c {
            CollectionName::Definitions => "definitions",
        }
    }
}

pub trait DbCollection {
    fn get_collection_name(&self) -> CollectionName;
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Definition {
    pub word: String,
    pub vocabulary: vocabulary::Word,
    pub oxford: Oxford,
    pub wordnik: Wordnik,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Oxford {}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Wordnik {}

impl DbCollection for Definition {
    fn get_collection_name(&self) -> CollectionName {
        CollectionName::Definitions
    }
}

impl FindOneFilter for Definition {
    fn find_one_filter(&self) -> Document {
        doc! {"word": &self.word}
    }
}
