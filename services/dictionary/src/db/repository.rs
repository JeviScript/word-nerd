use super::{
    models::{Audio, Definition, Pronunciation},
    DbErr,
};
use crate::vocabulary;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ReplaceOptions,
    Collection, Database,
};

#[derive(Debug)]
pub struct Repository {
    pub definitions: Collection<Definition>,
    pub audio: Collection<Audio>,
}

impl Repository {
    pub fn new(db: &Database) -> Self {
        Repository {
            definitions: db.collection("definitions"),
            audio: db.collection("audio"),
        }
    }

    pub async fn get_definition(&self, word: &str) -> Option<Definition> {
        let filter = doc! {"word" : word};

        match self.definitions.find_one(filter, None).await {
            Ok(val) => val,
            Err(_) => None,
        }
    }

    pub async fn replace_definition(&self, definition: Definition) -> Result<(), DbErr> {
        let filter = doc! {"word" : &definition.word};

        let mut replace_options = ReplaceOptions::default();
        // inserts when finds None
        replace_options.upsert = Some(true);
        self.definitions
            .replace_one(filter, definition, replace_options)
            .await
            .map_err(DbErr::QueryErr)?;
        Ok(())
    }

    pub async fn replace_vocabulary_audio(
        &self,
        word: String,
        data: Vec<vocabulary::Pronunciation>,
    ) -> Result<Vec<Pronunciation>, DbErr> {
        let filter = doc! {"word": word.as_str()};

        self.audio
            .delete_many(filter, None)
            .await
            .map_err(DbErr::QueryErr)?;

        let data: Vec<_> = data
            .into_iter()
            .map(|val| async {
                let mut result = Pronunciation {
                    variant: val.variant,
                    ipa_str: val.ipa_str,
                    audio_id: None,
                };

                match val.audio {
                    Some(audio) => {
                        let audio = Audio {
                            id: None,
                            word: word.clone(),
                            content_type: audio.content_type,
                            bytes: audio.bytes,
                        };

                        let audio_id: Option<ObjectId> =
                            match self.audio.insert_one(audio, None).await {
                                Ok(val) => val.inserted_id.as_object_id(),
                                Err(_) => None,
                            };
                        result.audio_id = audio_id;
                        result
                    }
                    None => result,
                }
            })
            .collect();

        let res = futures::future::join_all(data).await;
        Ok(res)
    }
}
