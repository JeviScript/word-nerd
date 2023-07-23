use crate::models::audio::AudioDoc;
use crate::models::oxford::DefinitionDoc as OxDefinitionDoc;
use crate::models::shared::{Pronunciation, PronunciationDoc};
use crate::models::vocabulary::DefinitionDoc as VocDefinitionDoc;
use crate::oxford::Definition as OxScrapedDefinition;
use crate::vocabulary::Definition as VocScrapedDefinition;
use crate::{db::database::DbErr, models::definition::DefinitionDoc};
use mongodb::options::FindOneOptions;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ReplaceOptions,
    Collection, Database,
};
use serde::Deserialize;

#[derive(Debug)]
pub struct Repository {
    pub definitions: Collection<DefinitionDoc>,
    pub audio: Collection<AudioDoc>,
    pub voc_definitions: Collection<VocDefinitionDoc>,
    pub ox_definitions: Collection<OxDefinitionDoc>,
}

#[derive(Deserialize)]
struct Id {
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

impl Repository {
    pub fn new(db: Database) -> Self {
        Repository {
            definitions: db.collection("definitions"),
            audio: db.collection("audio"),
            voc_definitions: db.collection("vocabulary_definitions"),
            ox_definitions: db.collection("oxford_definitions"),
        }
    }

    pub async fn get_definition(&self, word: &str) -> Result<Option<DefinitionDoc>, DbErr> {
        let filter = doc! {"word" : word};

        self.definitions
            .find_one(filter, None)
            .await
            .map_err(DbErr::QueryErr)
    }

    pub async fn get_voc_definition(&self, word: &str) -> Option<VocDefinitionDoc> {
        let filter = doc! {"id_ref" : word};

        self.voc_definitions
            .find_one(filter, None)
            .await
            .ok()
            .flatten()
    }

    pub async fn delete_definition(&self, word: &str) -> Result<(), DbErr> {
        let filter = doc! {"word" : word};

        self.definitions
            .delete_one(filter, None)
            .await
            .map_err(DbErr::QueryErr)?;

        Ok(())
    }

    pub async fn replace_definition(&self, definition: &DefinitionDoc) -> Result<(), DbErr> {
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

    pub async fn save_voc_definition(
        &self,
        def: VocScrapedDefinition,
    ) -> Result<Option<ObjectId>, DbErr> {
        let pros = self.save_audio(&def.id_ref, def.pronunciations).await;
        let def = VocDefinitionDoc {
            id: None,
            id_ref: def.id_ref,
            header: def.header,
            pronunciations: pros,
            other_forms: def.other_forms,
            short_description: def.short_description,
            long_description: def.long_description,
            definitions: def.definitions,
            examples: def.examples,
        };

        let filter = doc! {"id_ref" : &def.id_ref};
        let mut replace_options = ReplaceOptions::default();
        // inserts when finds None
        replace_options.upsert = Some(true);

        self.voc_definitions
            .replace_one(filter.clone(), def, replace_options)
            .await
            .map_err(DbErr::QueryErr)?;

        let object_id = self
            .voc_definitions
            .clone_with_type::<Id>()
            .find_one(
                filter.clone(),
                FindOneOptions::builder()
                    .projection(doc! {"_id": 1})
                    .build(),
            )
            .await
            .ok()
            .flatten()
            .map(|val| val.id);

        Ok(object_id)
    }

    pub async fn save_ox_definition(
        &self,
        def: OxScrapedDefinition,
    ) -> Result<Option<ObjectId>, DbErr> {
        let pros = self.save_audio(&def.id_ref, def.pronunciations).await;
        let def = OxDefinitionDoc {
            id: None,
            id_ref: def.id_ref,
            header: def.header,
            inflections: def.inflections,
            note: def.note,
            grammar_hint: def.grammar_hint,
            word_variant: def.word_variant,
            similar_results: def.similar_results,
            pronunciations: pros,
            definitions: def.definitions,
            see_also: def.see_also,
            word_origin: def.word_origin,
            idioms: def.idioms,
            phrasal_verbs: def.phrasal_verbs,
            veb_forms: def.veb_forms,
        };

        let filter = doc! {"id_ref" : &def.id_ref};
        let mut replace_options = ReplaceOptions::default();
        // inserts when finds None
        replace_options.upsert = Some(true);
        self.ox_definitions
            .replace_one(filter.clone(), def, replace_options)
            .await
            .map_err(DbErr::QueryErr)?;

        let object_id = self
            .ox_definitions
            .clone_with_type::<Id>()
            .find_one(
                filter.clone(),
                FindOneOptions::builder()
                    .projection(doc! {"_id": 1})
                    .build(),
            )
            .await
            .ok()
            .flatten()
            .map(|val| val.id);

        Ok(object_id)
    }

    async fn save_audio(&self, word: &str, data: Vec<Pronunciation>) -> Vec<PronunciationDoc> {
        let data: Vec<_> = data
            .into_iter()
            .map(|val| async {
                let mut result = PronunciationDoc {
                    variant: val.variant,
                    ipa_str: val.ipa_str,
                    audio_id: None,
                };

                match val.audio {
                    Some(audio) => {
                        let audio = AudioDoc {
                            id: None,
                            word: word.to_string(),
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

        futures::future::join_all(data).await
    }

    pub async fn get_audio(&self, id: String) -> Result<Option<AudioDoc>, DbErr> {
        let object_id = ObjectId::parse_str(id).map_err(DbErr::ParseBsonErr)?;
        let filter = doc! {"_id": object_id};
        self.audio
            .find_one(filter, None)
            .await
            .map_err(DbErr::QueryErr)
    }
}
