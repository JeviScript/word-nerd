use super::shared::PronunciationDoc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DefinitionDoc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub voc_ref: String,
    pub searched_word: String,
    pub header: String,
    pub pronunciations: Vec<PronunciationDoc>,
    pub other_forms: Vec<String>,
    pub short_description: String,
    pub long_description: String,
    pub definitions: Vec<SubDefinition>,
    pub examples: Vec<Example>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Example {
    pub sentence: String,
    pub author: String,
    pub source_title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SubDefinition {
    pub variant: WordVariant,
    pub description: String,
    pub image: Option<Image>,
    pub short_examples: Vec<String>,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum WordVariant {
    #[default]
    Noun,
    Verb,
    Adjective,
    Adverb,
    Other(String),
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Image {
    pub bytes: Vec<u8>,
    pub format: String,
}
