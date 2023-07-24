use crate::db::database::DbErr;
use crate::dtos::GetWordDefinitionsResponseBuilder;
use crate::models::audio::AudioDoc;
use crate::models::definition::DefinitionDoc;
use crate::{oxford, vocabulary, DictionaryService};
use mongodb::bson::oid::ObjectId;
use rpc::dictionary::{GetAudioResponse, GetWordDefinitionsResponse};

impl DictionaryService {
    pub async fn get_word_definitions(
        &self,
        word: String,
    ) -> Result<GetWordDefinitionsResponse, DbErr> {
        let definition = self.repository.get_definition(&word).await?;

        let _definition = match definition {
            Some(val) => val,
            None => self.create_definition(&word).await?,
        };

        let voc_definition = self.repository.get_voc_definition(&word).await;
        let oxford_definition = self.repository.get_ox_definition(&word).await;
        let response =
            GetWordDefinitionsResponseBuilder::new(&word, voc_definition, oxford_definition)
                .build();
        Ok(response)
    }

    pub async fn invalidate_word(&self, word: String) -> Result<(), DbErr> {
        self.repository.delete_definition(word.as_str()).await
    }

    pub async fn get_audio(&self, id: String) -> Result<Option<GetAudioResponse>, DbErr> {
        self.repository
            .get_audio(id)
            .await
            .map(|o| o.map(|audio| audio.to_response()))
    }

    async fn create_definition(&self, word: &str) -> Result<DefinitionDoc, DbErr> {
        let (vocabulary_id, oxford_id) = tokio::join!(
            self.get_voc_definition_id(word),
            self.get_ox_definition_id(word)
        );

        let definition = DefinitionDoc {
            word: word.to_string(),
            vocabulary_id,
            oxford_id,
        };

        self.repository.replace_definition(&definition).await?;

        Ok(definition)
    }

    /// tries to get from a db, if not found => scrape and store
    async fn get_voc_definition_id(&self, word: &str) -> Option<ObjectId> {
        match self.repository.get_voc_definition_id(word).await {
            Some(id) => Some(id),
            None => self.create_voc_definition(word).await,
        }
    }

    /// tries to get from a db, if not found => scrape and store
    async fn get_ox_definition_id(&self, word: &str) -> Option<ObjectId> {
        match self.repository.get_ox_definition_id(word).await {
            Some(id) => Some(id),
            None => self.create_ox_definition(word).await,
        }
    }

    async fn create_voc_definition(&self, word: &str) -> Option<ObjectId> {
        match vocabulary::scrape(word).await {
            Ok(scraped) => self
                .repository
                .save_voc_definition(scraped, word.to_string())
                .await
                .ok()
                .flatten(),
            Err(_) => None,
        }
    }

    async fn create_ox_definition(&self, word: &str) -> Option<ObjectId> {
        match oxford::scrape(word).await {
            Ok(scraped) => self
                .repository
                .save_ox_definition(scraped, word.to_string())
                .await
                .ok()
                .flatten(),
            Err(_) => None,
        }
    }
}

impl AudioDoc {
    fn to_response(&self) -> GetAudioResponse {
        GetAudioResponse {
            word: self.word.clone(),
            content_type: self.content_type.clone(),
            bytes: self.bytes.clone(),
        }
    }
}
