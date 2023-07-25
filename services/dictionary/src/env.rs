use common_rs::{env, EnvStore};

#[derive(Debug, Clone)]
pub struct Env {
    pub db_connection_uri: String,
}

impl EnvStore for Env {
    fn new() -> Self {
        Env {
            db_connection_uri: env::required("DB_CONNECTION_URI"),
        }
    }
}
