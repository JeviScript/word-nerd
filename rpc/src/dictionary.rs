#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWordDefinitionsRequest {
    #[prost(string, tag = "1")]
    pub word: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWordDefinitionsResponse {
    #[prost(string, tag = "1")]
    pub word: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub vocabulary: ::core::option::Option<VocabularyWord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VocabularyWord {
    #[prost(string, tag = "1")]
    pub header: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub pronunciations: ::prost::alloc::vec::Vec<Pronunciation>,
    #[prost(string, repeated, tag = "3")]
    pub other_forms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub short_description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub long_description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub definitions: ::prost::alloc::vec::Vec<VocabularyDefinition>,
    #[prost(message, repeated, tag = "7")]
    pub examples: ::prost::alloc::vec::Vec<VocabularyExample>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pronunciation {
    #[prost(enumeration = "pronunciation::PronunciationVariant", tag = "1")]
    pub variant: i32,
    #[prost(string, tag = "2")]
    pub ipa_str: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub audio_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Pronunciation`.
pub mod pronunciation {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PronunciationVariant {
        Uk = 0,
        Usa = 1,
        Other = 2,
    }
    impl PronunciationVariant {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PronunciationVariant::Uk => "Uk",
                PronunciationVariant::Usa => "Usa",
                PronunciationVariant::Other => "Other",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Uk" => Some(Self::Uk),
                "Usa" => Some(Self::Usa),
                "Other" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VocabularyDefinition {
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "7")]
    pub short_examples: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub synonyms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof = "vocabulary_definition::WordVariant", tags = "1, 2")]
    pub word_variant: ::core::option::Option<vocabulary_definition::WordVariant>,
}
/// Nested message and enum types in `VocabularyDefinition`.
pub mod vocabulary_definition {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum KnownWordVariant {
        Noun = 0,
        Verb = 1,
        Adjective = 2,
        Adverb = 3,
    }
    impl KnownWordVariant {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KnownWordVariant::Noun => "Noun",
                KnownWordVariant::Verb => "Verb",
                KnownWordVariant::Adjective => "Adjective",
                KnownWordVariant::Adverb => "Adverb",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Noun" => Some(Self::Noun),
                "Verb" => Some(Self::Verb),
                "Adjective" => Some(Self::Adjective),
                "Adverb" => Some(Self::Adverb),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WordVariant {
        #[prost(enumeration = "KnownWordVariant", tag = "1")]
        WordVariant(i32),
        #[prost(string, tag = "2")]
        OtherWordVariant(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VocabularyExample {
    #[prost(string, tag = "1")]
    pub sentence: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub author: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub source_title: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidateWordRequest {
    #[prost(string, tag = "1")]
    pub word: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidateWordResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAudioRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAudioResponse {
    #[prost(string, tag = "1")]
    pub word: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod dictionary_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DictionaryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DictionaryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DictionaryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DictionaryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DictionaryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_word_definitions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWordDefinitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWordDefinitionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dictionary.Dictionary/GetWordDefinitions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dictionary.Dictionary", "GetWordDefinitions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn invalidate_word(
            &mut self,
            request: impl tonic::IntoRequest<super::InvalidateWordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InvalidateWordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dictionary.Dictionary/InvalidateWord",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dictionary.Dictionary", "InvalidateWord"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_audio(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAudioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAudioResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dictionary.Dictionary/GetAudio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dictionary.Dictionary", "GetAudio"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dictionary_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DictionaryServer.
    #[async_trait]
    pub trait Dictionary: Send + Sync + 'static {
        async fn get_word_definitions(
            &self,
            request: tonic::Request<super::GetWordDefinitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWordDefinitionsResponse>,
            tonic::Status,
        >;
        async fn invalidate_word(
            &self,
            request: tonic::Request<super::InvalidateWordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InvalidateWordResponse>,
            tonic::Status,
        >;
        async fn get_audio(
            &self,
            request: tonic::Request<super::GetAudioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAudioResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DictionaryServer<T: Dictionary> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Dictionary> DictionaryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DictionaryServer<T>
    where
        T: Dictionary,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/dictionary.Dictionary/GetWordDefinitions" => {
                    #[allow(non_camel_case_types)]
                    struct GetWordDefinitionsSvc<T: Dictionary>(pub Arc<T>);
                    impl<
                        T: Dictionary,
                    > tonic::server::UnaryService<super::GetWordDefinitionsRequest>
                    for GetWordDefinitionsSvc<T> {
                        type Response = super::GetWordDefinitionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWordDefinitionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_word_definitions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWordDefinitionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dictionary.Dictionary/InvalidateWord" => {
                    #[allow(non_camel_case_types)]
                    struct InvalidateWordSvc<T: Dictionary>(pub Arc<T>);
                    impl<
                        T: Dictionary,
                    > tonic::server::UnaryService<super::InvalidateWordRequest>
                    for InvalidateWordSvc<T> {
                        type Response = super::InvalidateWordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InvalidateWordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).invalidate_word(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InvalidateWordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/dictionary.Dictionary/GetAudio" => {
                    #[allow(non_camel_case_types)]
                    struct GetAudioSvc<T: Dictionary>(pub Arc<T>);
                    impl<
                        T: Dictionary,
                    > tonic::server::UnaryService<super::GetAudioRequest>
                    for GetAudioSvc<T> {
                        type Response = super::GetAudioResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAudioRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_audio(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAudioSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Dictionary> Clone for DictionaryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Dictionary> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Dictionary> tonic::server::NamedService for DictionaryServer<T> {
        const NAME: &'static str = "dictionary.Dictionary";
    }
}
