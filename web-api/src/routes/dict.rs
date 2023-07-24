use crate::Rpc;
use axum::{
    body::{Bytes, Full},
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/word/:word", get(get_word))
        .route("/audio/:id", get(get_audio))
}

async fn get_word(Path(word): Path<String>) -> impl IntoResponse {
    let mut client = Rpc::get_dictionary_client().await?;

    let request = tonic::Request::new(rpc::dictionary::GetWordDefinitionsRequest { word });

    match client.get_word_definitions(request).await {
        Ok(res) => {
            let response: get_word_response::Response = res.into_inner().into();
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_status) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_audio(Path(id): Path<String>) -> impl IntoResponse {
    let mut client = Rpc::get_dictionary_client().await?;

    let request = tonic::Request::new(rpc::dictionary::GetAudioRequest { id });

    match client.get_audio(request).await {
        Ok(res) => {
            let res = res.into_inner();
            let response = Response::builder()
                .status(200)
                .header("Content-type", res.content_type.as_str())
                .body(Full::new(Bytes::from(res.bytes)));

            response.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(status) => match status.code() {
            tonic::Code::NotFound => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

mod get_word_response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Response {
        pub word: String,
        pub vocabulary: Option<VocabularyWord>,
    }

    impl From<rpc::dictionary::GetWordDefinitionsResponse> for Response {
        fn from(value: rpc::dictionary::GetWordDefinitionsResponse) -> Self {
            Response {
                word: value.word,
                vocabulary: value.vocabulary.map(|v| VocabularyWord {
                    header: v.header,
                    short_description: v.short_description,
                    long_description: v.long_description,
                    other_forms: v.other_forms,
                    pronunciations: v
                        .pronunciations
                        .into_iter()
                        .map(|p| Pronunciation {
                            ipa_str: p.ipa_str,
                            audio_id: p.audio_id,
                            variant: p.variant.into(),
                        })
                        .collect(),
                    definitions: v
                        .definitions
                        .into_iter()
                        .map(|d| Definition {
                            description: d.description,
                            short_examples: d.short_examples,
                            synonyms: d.synonyms,
                            variant: d.word_variant.into(),
                        })
                        .collect(),
                    examples: v
                        .examples
                        .into_iter()
                        .map(|e| Example {
                            sentence: e.sentence,
                            author: e.author,
                            source_title: e.source_title,
                        })
                        .collect(),
                }),
            }
        }
    }

    #[derive(Serialize)]
    pub struct VocabularyWord {
        pub header: String,
        pub pronunciations: Vec<Pronunciation>,
        pub other_forms: Vec<String>,
        pub short_description: String,
        pub long_description: String,
        pub definitions: Vec<Definition>,
        pub examples: Vec<Example>,
    }

    #[derive(Serialize)]
    pub struct Pronunciation {
        pub variant: PronunciationVariant,
        pub ipa_str: String,
        pub audio_id: Option<String>,
    }

    #[derive(Serialize, Default)]
    pub enum PronunciationVariant {
        #[default]
        Uk,
        Usa,
        Other,
    }

    impl From<i32> for PronunciationVariant {
        fn from(value: i32) -> Self {
            type Rpc = rpc::dictionary::pronunciation::PronunciationVariant;
            match Rpc::from_i32(value) {
                Some(Rpc::Uk) => PronunciationVariant::Uk,
                Some(Rpc::Usa) => PronunciationVariant::Usa,
                Some(Rpc::Other) => PronunciationVariant::Other,
                None => PronunciationVariant::Other,
            }
        }
    }

    #[derive(Serialize)]
    pub struct Definition {
        pub variant: WordVariant,
        pub description: String,
        pub short_examples: Vec<String>,
        pub synonyms: Vec<String>,
    }

    #[derive(Serialize)]
    pub struct Image {
        pub bytes: Vec<u8>,
        pub format: String,
    }

    #[derive(Serialize, Default)]
    pub enum WordVariant {
        #[default]
        Noun,
        Verb,
        Adjective,
        Adverb,
        Other(String),
    }

    impl From<Option<rpc::dictionary::vocabulary_sub_definition::WordVariant>> for WordVariant {
        fn from(value: Option<rpc::dictionary::vocabulary_sub_definition::WordVariant>) -> Self {
            use rpc::dictionary::vocabulary_sub_definition as Rpc;
            match value {
                Some(Rpc::WordVariant::WordVariant(known)) => {
                    match Rpc::KnownWordVariant::from_i32(known) {
                        Some(Rpc::KnownWordVariant::Noun) => WordVariant::Noun,
                        Some(Rpc::KnownWordVariant::Verb) => WordVariant::Verb,
                        Some(Rpc::KnownWordVariant::Adjective) => WordVariant::Adjective,
                        Some(Rpc::KnownWordVariant::Adverb) => WordVariant::Adverb,
                        None => WordVariant::Other("Unknown".to_string()),
                    }
                }
                Some(Rpc::WordVariant::OtherWordVariant(val)) => WordVariant::Other(val),
                None => WordVariant::Other("Unknown".to_string()),
            }
        }
    }

    #[derive(Serialize)]
    pub struct Example {
        pub sentence: String,
        pub author: String,
        pub source_title: String,
    }
}
