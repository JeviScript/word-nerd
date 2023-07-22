use crate::vocabulary;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Definition {
    pub word: String,
    pub vocabulary: VocabularyWord,
    pub oxford: Oxford,
    pub wordnik: Wordnik,
}

impl Definition {
    pub fn new(word: String, vocabulary: VocabularyWord) -> Self {
        Definition {
            word,
            vocabulary,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct VocabularyWord {
    pub header: String,
    pub pronunciations: Vec<Pronunciation>,
    pub other_forms: Vec<String>,
    pub short_description: String,
    pub long_description: String,
    pub definitions: Vec<vocabulary::Definition>,
    pub examples: Vec<vocabulary::Example>,
}

impl VocabularyWord {
    pub fn new(word: vocabulary::Word, pronunciations: Vec<Pronunciation>) -> Self {
        VocabularyWord {
            header: word.header,
            pronunciations,
            other_forms: word.other_forms,
            short_description: word.short_description,
            long_description: word.long_description,
            definitions: word.definitions,
            examples: word.examples
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Pronunciation {
    pub variant: vocabulary::PronunciationVariant,
    pub ipa_str: String,
    pub audio_id: Option<ObjectId>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Oxford {}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Wordnik {}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Audio { // TODO experiment with local storage systems
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub word: String,
    pub content_type: String, // video/mp4 , audio/mpeg
    pub bytes: Vec<u8>,
}
