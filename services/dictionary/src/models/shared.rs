use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Pronunciation {
    pub variant: PronunciationVariant,
    pub ipa_str: String,
    pub audio: Option<Audio>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Audio {
    pub content_type: String, // video/mp4 , audio/mpeg
    pub bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PronunciationDoc {
    pub variant: PronunciationVariant,
    pub ipa_str: String,
    pub audio_id: Option<ObjectId>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum PronunciationVariant {
    #[default]
    Uk,
    Usa,
    Other,
}
