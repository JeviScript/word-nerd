use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::shared::{Pronunciation, PronunciationDoc};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DefinitionDoc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub id_ref: String,
    pub searched_word: String,
    pub header: String,
    pub inflections: String,
    pub note: String,
    pub grammar_hint: String,
    pub word_variant: String,
    pub similar_results: Vec<SimilarResult>,
    pub pronunciations: Vec<PronunciationDoc>,
    pub definitions: Vec<DefinitionGroup>,
    pub see_also: Vec<WordRef>,
    pub word_origin: String,
    pub idioms: Vec<Idiom>,
    pub phrasal_verbs: Vec<WordRef>,
    pub veb_forms: Vec<VebForm>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SimilarResult {
    pub id: String,
    pub word: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DefinitionGroup {
    pub group_title: Option<String>,
    pub definitions: Vec<SubDefinition>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SubDefinition {
    pub description: String,
    pub use_note: String,
    pub examples: Vec<String>,
    pub see_also: Vec<WordRef>,
    pub synonyms: Vec<WordRef>,
    pub extra_examples: Vec<String>,
    pub extra_synonyms: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct WordRef {
    pub id_ref: String,
    pub word: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Idiom {
    pub idiom: String,
    pub description: String,
    pub notes: Vec<String>,
    pub synonyms: Vec<WordRef>,
    pub examples: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct VebForm {
    pub note: String,
    pub word: String,
    pub pronunciations: Vec<Pronunciation>,
}
