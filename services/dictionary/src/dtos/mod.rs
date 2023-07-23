use crate::models::{vocabulary::{DefinitionDoc as VocDefinitionDoc, WordVariant}, shared::PronunciationVariant};
use rpc::dictionary::GetWordDefinitionsResponse;

pub struct GetWordDefinitionsResponseBuilder {
    pub word: String,
    pub vocabulary: Option<VocDefinitionDoc>,
}

impl VocDefinitionDoc {
    pub fn to_response(&self) -> rpc::dictionary::VocabularyWord {
        rpc::dictionary::VocabularyWord {
            header: self.header.clone(),
            long_description: self.long_description.clone(),
            short_description: self.short_description.clone(),
            pronunciations: self
                .pronunciations
                .clone()
                .into_iter()
                .map(|p| rpc::dictionary::Pronunciation {
                    variant: rpc::dictionary::pronunciation::PronunciationVariant::from(p.variant)
                        as i32,
                    ipa_str: p.ipa_str,
                    audio_id: p.audio_id.map(|id| id.to_string()),
                })
                .collect(),
            definitions: self
                .definitions
                .clone()
                .into_iter()
                .map(|d| rpc::dictionary::VocabularyDefinition {
                    description: d.description,
                    short_examples: d.short_examples,
                    synonyms: d.synonyms,
                    word_variant: Some(d.variant.into()),
                })
                .collect(),
            examples: self
                .examples
                .clone()
                .into_iter()
                .map(|e| rpc::dictionary::VocabularyExample {
                    author: e.author,
                    sentence: e.sentence,
                    source_title: e.source_title,
                })
                .collect(),
            other_forms: self.other_forms.clone(),
        }
    }
}

impl GetWordDefinitionsResponseBuilder {
    pub fn new(word: &str, voc: Option<VocDefinitionDoc>) -> Self {
       Self {
            word: word.to_string(),
            vocabulary: voc
        } 
    }

    pub fn build(self) -> GetWordDefinitionsResponse {
        GetWordDefinitionsResponse {
            word: self.word.clone(),
            vocabulary: self.vocabulary.map(|v| v.to_response()),
        }
    }
}

impl From<WordVariant> for rpc::dictionary::vocabulary_definition::WordVariant {
    fn from(value: WordVariant) -> Self {
        use rpc::dictionary::vocabulary_definition as Prost;
        match value {
            WordVariant::Noun => {
                Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Noun as i32)
            }
            WordVariant::Verb => {
                Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Verb as i32)
            }
            WordVariant::Adjective => {
                Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Adjective as i32)
            }
            WordVariant::Adverb => {
                Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Adverb as i32)
            }
            WordVariant::Other(val) => Prost::WordVariant::OtherWordVariant(val),
        }
    }
}

impl From<PronunciationVariant> for rpc::dictionary::pronunciation::PronunciationVariant {
    fn from(value: PronunciationVariant) -> Self {
        type Prost = rpc::dictionary::pronunciation::PronunciationVariant;
        match value {
            PronunciationVariant::Uk => Prost::Uk,
            PronunciationVariant::Usa => Prost::Usa,
            PronunciationVariant::Other => Prost::Other,
        }
    }
}
