use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DefinitionDoc {
    pub word: String,
    pub vocabulary_id: Option<ObjectId>,
    pub oxford_id: Option<ObjectId>,
}
