use crate::ENV;
use std::env;

#[derive(Debug, Clone)]
pub struct Env {
    pub vocabulary_url: String,
    pub db_connection_uri: String,
}

// TODO reuse logic for setting env variables with services somehow
impl Env {
    // Can panic !
    pub fn init() -> Self {
        let env = Env {
            vocabulary_url: Env::required("VOCABULARY_URL"),
            db_connection_uri: Env::required("DB_CONNECTION_URI"),
        };

        _ = ENV.set(env.clone());
        env
    }

    fn required(env_var: &str) -> String {
        env::var(env_var).unwrap_or_else(|_| panic!("Missing required env variable: {}", env_var))
    }

    pub fn vars() -> Self {
        match ENV.get() {
            Some(val) => val.clone(),
            None => Self::init(),
        }
    }
}
