use crate::ENV;
use std::env;

#[derive(Debug, Clone)]
pub struct Env {
    pub account_service_uri: String,
    pub dict_service_uri: String,
}

// TODO reuse logic for setting env variables with services somehow
impl Env {
    // Can panic !
    fn init() -> Self {
        let env = Env {
            account_service_uri: Env::required("ACCOUNT_SERVICE_URI"),
            dict_service_uri: Env::required("DICTIONARY_SERVICE_URI"),
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
