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
        pub vocabulary: Option<VocabularyDefinition>,
        pub oxford: Option<OxfordDefinition>,
    }

    impl From<rpc::dictionary::GetWordDefinitionsResponse> for Response {
        fn from(value: rpc::dictionary::GetWordDefinitionsResponse) -> Self {
            Response {
                word: value.word,
                vocabulary: value.vocabulary_definition.map(|v| v.into()),
                oxford: value.oxford_definition.map(|x| x.into()),
            }
        }
    }

    impl From<rpc::dictionary::VocabularyDefinition> for VocabularyDefinition {
        fn from(v: rpc::dictionary::VocabularyDefinition) -> Self {
            VocabularyDefinition {
                header: v.header,
                short_description: v.short_description,
                long_description: v.long_description,
                other_forms: v.other_forms,
                pronunciations: v.pronunciations.into_iter().map(|p| p.into()).collect(),
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
            }
        }
    }

    #[derive(Serialize)]
    pub struct VocabularyDefinition {
        pub header: String,
        pub pronunciations: Vec<Pronunciation>,
        pub other_forms: Vec<String>,
        pub short_description: String,
        pub long_description: String,
        pub definitions: Vec<Definition>,
        pub examples: Vec<Example>,
    }

    #[derive(Serialize)]
    pub struct OxfordDefinition {
        pub id: String,
        pub header: String,
        pub oxford_ref: String,
        pub inflections: String,
        pub note: String,
        pub word_variant: String,
        pub word_origin: String,
        pub similar_results: Vec<WordRef>,
        pub pronunciations: Vec<Pronunciation>,
        pub definitions: Vec<DefinitionGroup>,
        pub see_also: Vec<WordRef>,
        pub idioms: Vec<Idiom>,
        pub phrasal_verbs: Vec<WordRef>,
    }

    impl From<rpc::dictionary::OxfordDefinition> for OxfordDefinition {
        fn from(value: rpc::dictionary::OxfordDefinition) -> Self {
            OxfordDefinition {
                id: value.id,
                header: value.header,
                oxford_ref: value.oxford_ref,
                inflections: value.inflections,
                note: value.note,
                word_variant: value.word_variant,
                word_origin: value.word_origin,
                similar_results: value
                    .similar_results
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
                pronunciations: value.pronunciations.into_iter().map(|x| x.into()).collect(),
                definitions: value.definitions.into_iter().map(|x| x.into()).collect(),
                see_also: value.see_also.into_iter().map(|x| x.into()).collect(),
                idioms: value.idioms.into_iter().map(|x| x.into()).collect(),
                phrasal_verbs: value.phrasal_verbs.into_iter().map(|x| x.into()).collect(),
            }
        }
    }

    impl From<rpc::dictionary::WordRef> for WordRef {
        fn from(value: rpc::dictionary::WordRef) -> Self {
            WordRef {
                oxford_ref: value.oxford_ref,
                word: value.word,
            }
        }
    }

    #[derive(Serialize)]
    pub struct Idiom {
        pub idiom: String,
        pub description: String,
        pub notes: Vec<String>,
        pub synonyms: Vec<WordRef>,
        pub examples: Vec<String>,
    }

    impl From<rpc::dictionary::Idiom> for Idiom {
        fn from(value: rpc::dictionary::Idiom) -> Self {
            Idiom {
                idiom: value.idiom,
                description: value.description,
                notes: value.notes,
                synonyms: value.synonyms.into_iter().map(|x| x.into()).collect(),
                examples: value.examples,
            }
        }
    }

    #[derive(Serialize)]
    pub struct DefinitionGroup {
        pub group_title: String,
        pub definitions: Vec<SubDefinition>,
    }

    impl From<rpc::dictionary::DefinitionGroup> for DefinitionGroup {
        fn from(value: rpc::dictionary::DefinitionGroup) -> Self {
            DefinitionGroup {
                group_title: value.group_title,
                definitions: value.definitions.into_iter().map(|x| x.into()).collect(),
            }
        }
    }

    #[derive(Serialize)]
    pub struct SubDefinition {
        pub description: String,
        pub use_note: String,
        pub examples: Vec<String>,
        pub see_also: Vec<WordRef>,
        pub synonyms: Vec<WordRef>,
        pub extra_examples: Vec<String>,
        pub extra_synonyms: Vec<String>,
    }

    impl From<rpc::dictionary::SubDefinition> for SubDefinition {
        fn from(value: rpc::dictionary::SubDefinition) -> Self {
            SubDefinition {
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

    #[derive(Serialize)]
    pub struct WordRef {
        pub oxford_ref: String,
        pub word: String,
    }

    #[derive(Serialize)]
    pub struct Pronunciation {
        pub variant: PronunciationVariant,
        pub ipa_str: String,
        pub audio_id: Option<String>,
    }

    impl From<rpc::dictionary::Pronunciation> for Pronunciation {
        fn from(value: rpc::dictionary::Pronunciation) -> Self {
            Pronunciation {
                variant: value.variant.into(),
                ipa_str: value.ipa_str,
                audio_id: value.audio_id,
            }
        }
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
