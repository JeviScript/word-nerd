use crate::models;
use crate::models::oxford::{DefinitionDoc as OxfordDefinitionDoc, DefinitionGroup, SubDefinition};
use crate::models::shared::PronunciationDoc;
use crate::models::{
    oxford::WordRef,
    shared::PronunciationVariant,
    vocabulary::{DefinitionDoc as VocDefinitionDoc, WordVariant},
};
use rpc::dictionary::GetWordDefinitionsResponse;

pub struct GetWordDefinitionsResponseBuilder {
    pub word: String,
    pub vocabulary: Option<VocDefinitionDoc>,
    pub oxford: Option<OxfordDefinitionDoc>,
}

impl OxfordDefinitionDoc {
    pub fn into_response(self) -> rpc::dictionary::OxfordDefinition {
        rpc::dictionary::OxfordDefinition {
            id: self.id.unwrap_or_default().to_string(),
            header: self.header,
            oxford_ref: self.oxford_ref,
            inflections: self.inflections,
            note: self.note,
            word_variant: self.word_variant,
            word_origin: self.word_origin,
            similar_results: self.similar_results.into_iter().map(|x| x.into()).collect(),
            pronunciations: self.pronunciations.into_iter().map(|x| x.into()).collect(),
            definitions: self.definitions.into_iter().map(|x| x.into()).collect(),
            see_also: self.see_also.into_iter().map(|x| x.into()).collect(),
            idioms: self.idioms.into_iter().map(|x| x.into()).collect(),
            phrasal_verbs: self.phrasal_verbs.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl VocDefinitionDoc {
    pub fn into_response(self) -> rpc::dictionary::VocabularyDefinition {
        rpc::dictionary::VocabularyDefinition {
            id: self.id.unwrap_or_default().to_string(),
            header: self.header,
            long_description: self.long_description,
            short_description: self.short_description,
            pronunciations: self.pronunciations.into_iter().map(|p| p.into()).collect(),
            definitions: self
                .definitions
                .into_iter()
                .map(|d| rpc::dictionary::VocabularySubDefinition {
                    description: d.description,
                    short_examples: d.short_examples,
                    synonyms: d.synonyms,
                    word_variant: Some(d.variant.into()),
                })
                .collect(),
            examples: self
                .examples
                .into_iter()
                .map(|e| rpc::dictionary::VocabularyExample {
                    author: e.author,
                    sentence: e.sentence,
                    source_title: e.source_title,
                })
                .collect(),
            other_forms: self.other_forms,
        }
    }
}

impl GetWordDefinitionsResponseBuilder {
    pub fn new(
        word: &str,
        voc: Option<VocDefinitionDoc>,
        oxford: Option<OxfordDefinitionDoc>,
    ) -> Self {
        Self {
            word: word.to_string(),
            vocabulary: voc,
            oxford,
        }
    }

    pub fn build(self) -> GetWordDefinitionsResponse {
        GetWordDefinitionsResponse {
            word: self.word.clone(),
            vocabulary_definition: self.vocabulary.map(|v| v.into_response()),
            oxford_definition: self.oxford.map(|v| v.into_response()),
        }
    }
}

impl From<WordVariant> for rpc::dictionary::vocabulary_sub_definition::WordVariant {
    fn from(value: WordVariant) -> Self {
        use rpc::dictionary::vocabulary_sub_definition as Prost;
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

impl From<WordRef> for rpc::dictionary::WordRef {
    fn from(value: WordRef) -> Self {
        rpc::dictionary::WordRef {
            oxford_ref: value.oxford_ref,
            word: value.word,
        }
    }
}

impl From<DefinitionGroup> for rpc::dictionary::DefinitionGroup {
    fn from(value: DefinitionGroup) -> Self {
        rpc::dictionary::DefinitionGroup {
            group_title: value.group_title.unwrap_or_default(),
            definitions: value.definitions.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<SubDefinition> for rpc::dictionary::SubDefinition {
    fn from(value: SubDefinition) -> Self {
        rpc::dictionary::SubDefinition {
            description: value.description,
            use_note: value.use_note,
            examples: value.examples,
            see_also: value.see_also.into_iter().map(|x| x.into()).collect(),
            synonyms: value.synonyms.into_iter().map(|x| x.into()).collect(),
            extra_examples: value.extra_examples,
            extra_synonyms: value.extra_synonyms,
        }
    }
}

impl From<models::oxford::Idiom> for rpc::dictionary::Idiom {
    fn from(value: models::oxford::Idiom) -> Self {
        rpc::dictionary::Idiom {
            idiom: value.idiom,
            description: value.description,
            notes: value.notes,
            synonyms: value.synonyms.into_iter().map(|x| x.into()).collect(),
            examples: value.examples,
        }
    }
}

impl From<PronunciationDoc> for rpc::dictionary::Pronunciation {
    fn from(p: PronunciationDoc) -> Self {
        rpc::dictionary::Pronunciation {
            variant: rpc::dictionary::pronunciation::PronunciationVariant::from(p.variant) as i32,
            ipa_str: p.ipa_str,
            audio_id: p.audio_id.map(|id| id.to_string()),
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
