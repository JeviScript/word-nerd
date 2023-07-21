use rpc::dictionary::{GetWordDefinitionsResponse, GetAudioResponse};

use crate::{
    db::{
        models::{Definition, VocabularyWord, Audio},
        DbErr,
    },
    vocabulary::{self, PronunciationVariant, WordVariant},
    DictionaryService,
};

impl DictionaryService {
    pub async fn get_word_definitions(
        &self,
        word: String,
    ) -> Result<GetWordDefinitionsResponse, DbErr> {
        let definition = self.repository.get_definition(word.as_str()).await?;

        let definition = match definition {
            Some(val) => val,
            None => self.create_definition(word).await?,
        };

        Ok(definition.to_response())
    }

    pub async fn invalidate_word(&self, word: String) -> Result<(), DbErr> {
        self.repository.delete_definition(word.as_str()).await
    }

    pub async fn get_audio(&self, id: String) -> Result<Option<GetAudioResponse>, DbErr> {
       self.repository.get_audio(id).await.map(|o| o.map(|audio| audio.to_response()))
    }

    async fn create_definition(&self, word: String) -> Result<Definition, DbErr> {
        let voc = vocabulary::scrape(word.as_str()).await.unwrap_or_default();

        let pronunciations = self
            .repository
            .replace_vocabulary_audio(word.to_string(), voc.clone().pronunciations)
            .await?;

        let voc = VocabularyWord::new(voc, pronunciations);

        let definition = Definition::new(word.clone(), voc);

        self.repository.replace_definition(definition).await?;

        let result = self.repository.get_definition(word.as_str()).await?;

        result.ok_or(DbErr::Unexpected)
    }
}

impl Definition {
    fn to_response(&self) -> GetWordDefinitionsResponse {
        GetWordDefinitionsResponse {
            word: self.word.clone(),
            vocabulary: Some(rpc::dictionary::VocabularyWord {
                header: self.vocabulary.header.clone(),
                long_description: self.vocabulary.long_description.clone(),
                short_description: self.vocabulary.short_description.clone(),
                pronunciations: self
                    .vocabulary
                    .pronunciations
                    .clone()
                    .into_iter()
                    .map(|p| rpc::dictionary::Pronunciation {
                        variant: rpc::dictionary::pronunciation::PronunciationVariant::from(p.variant) as i32,
                        ipa_str: p.ipa_str,
                        audio_id: p.audio_id.map(|id| id.to_string()),
                    })
                    .collect(),
                definitions: self.vocabulary.definitions.clone().into_iter().map(|d| rpc::dictionary::VocabularyDefinition {
                    description: d.description,
                    short_examples: d.short_examples,
                    synonyms: d.synonyms,
                    word_variant: Some(d.variant.into())
                }).collect(),
                examples: self.vocabulary.examples.clone().into_iter().map(|e| rpc::dictionary::VocabularyExample {
                    author: e.author,
                    sentence: e.sentence,
                    source_title: e.source_title
                }).collect(),
                other_forms: self.vocabulary.other_forms.clone(),
            }),
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

impl From<WordVariant> for rpc::dictionary::vocabulary_definition::WordVariant {
    fn from(value: WordVariant) -> Self {
        use rpc::dictionary::vocabulary_definition as Prost;
        match value {
            WordVariant::Noun => Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Noun as i32),
            WordVariant::Verb => Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Verb as i32),
            WordVariant::Adjective => Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Adjective as i32),
            WordVariant::Adverb => Prost::WordVariant::WordVariant(Prost::KnownWordVariant::Adverb as i32),
            WordVariant::Other(val) => Prost::WordVariant::OtherWordVariant(val),
        }
    }
}


impl Audio {
    fn to_response(&self) -> GetAudioResponse {
        GetAudioResponse {
            word: self.word.clone(),
            content_type: self.content_type.clone(),
            bytes: self.bytes.clone()
        }
    }
}
