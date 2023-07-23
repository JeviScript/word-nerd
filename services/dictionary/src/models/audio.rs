use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct AudioDoc { // TODO experiment with local storage systems
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub word: String,
    pub content_type: String, // video/mp4 , audio/mpeg
    pub bytes: Vec<u8>,
}
