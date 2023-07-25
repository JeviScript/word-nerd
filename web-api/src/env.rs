use common_rs::{env, EnvStore};

#[derive(Debug, Clone)]
pub struct Env {
    pub account_service_uri: String,
    pub dict_service_uri: String,
}

impl EnvStore for Env {
    fn new() -> Self {
        Env {
            account_service_uri: env::required("ACCOUNT_SERVICE_URI"),
            dict_service_uri: env::required("DICTIONARY_SERVICE_URI"),
        }
    }
}
