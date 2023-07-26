use common_rs::{env, EnvStore};

#[derive(Debug, Clone)]
pub struct Env {
    pub meili_master_key: String,
    pub meili_connection_uri: String,
}

impl EnvStore for Env {
    fn new() -> Self {
        Env {
            meili_master_key: env::required("MEILI_MASTER_KEY"),
            meili_connection_uri: env::required("MEILISEARCH_CONNECTION_URI")
        }
    }
}
