use rpc::dictionary::{GetWordDefinitionsResponse, Pronunciation, GetAudioResponse};

use crate::{
    db::{
        models::{Definition, VocabularyWord, Audio},
        DbErr,
    },
    vocabulary::{self, PronunciationVariant},
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
                pronunciations: self
                    .vocabulary
                    .pronunciations
                    .clone()
                    .into_iter()
                    .map(|p| Pronunciation {
                        variant: p.variant.to_i32(),
                        ipa_str: p.ipa_str,
                        audio_id: p.audio_id.map(|id| id.to_string()),
                    })
                    .collect(),
            }),
        }
    }
}

impl PronunciationVariant {
    fn to_i32(&self) -> i32 {
        match self {
            PronunciationVariant::Uk => 0,
            PronunciationVariant::Usa => 1,
            PronunciationVariant::Other => 2,
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
