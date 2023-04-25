use std::collections::BTreeMap;

use base64::Engine;
use jwt::{Error, Header, PKeyWithDigest, Token, VerifyWithKey};
use openssl::{bn::BigNum, hash::MessageDigest, pkey::PKey, rsa::Rsa};
use reqwest;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

use crate::db::GoogleUser;

const GOOGLE_DISCOVERY_DOC_URL: &str =
    "https://accounts.google.com/.well-known/openid-configuration";

#[derive(Deserialize, Debug)]
struct DiscoveryDocument {
    // there are more fields, but I need only few
    pub jwks_uri: String,
}

#[derive(Deserialize, Debug)]
struct OAuthCert {
    keys: Vec<OAuthCertKey>,
}

#[derive(Deserialize, Debug)]
struct OAuthCertKey {
    pub alg: String,
    pub n: String,
    pub e: String,
}

async fn get<T: DeserializeOwned>(url: &str) -> T {
    reqwest::get(url)
        .await
        .expect(format!("failed to get from {url}").as_str())
        .json::<T>()
        .await
        .expect("failed to parse")
}

async fn get_discovery_document() -> DiscoveryDocument {
    get(GOOGLE_DISCOVERY_DOC_URL).await
}

pub async fn verify_token(token: String) -> Option<GoogleUser> {
    let doc = get_discovery_document().await;
    let cert = get::<OAuthCert>(&doc.jwks_uri).await;

    cert.keys.iter().find_map(|cert_key| {
        if cert_key.alg != "RS256" {
            panic!("Error: unsupported {} alg", cert_key.alg);
        }

        let e = BigNum::from_slice(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(&cert_key.e)
                .expect(format!("failed to decode e value {}", cert_key.e).as_str()),
        )
        .expect("failed to turn 'e' into BigNum");
        let n = BigNum::from_slice(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(&cert_key.n)
                .expect(format!("failed to decode n value {}", cert_key.n).as_str()),
        )
        .expect("failed to turn 'n' into BigNum");
        let rsa = Rsa::from_public_components(n, e).expect("failed to create Rsa from n and e");
        let pkey = PKey::from_rsa(rsa).expect("failed to create pkey");

        let rs256_public_key = PKeyWithDigest {
            digest: MessageDigest::sha256(),
            key: pkey,
        };

        let verify_result: Result<Token<Header, BTreeMap<String, Value>, _>, Error> =
            token.verify_with_key(&rs256_public_key);

        match verify_result {
            Ok(val) => {
                let claims = val.claims();
                let google_user = GoogleUser {
                    google_id: claims.get("sub").unwrap().to_string(),
                    first_name: claims.get("given_name").unwrap().to_string(),
                    last_name: claims.get("family_name").unwrap().to_string(),
                    email: claims.get("email").unwrap().to_string(),
                };
                Some(google_user)
            }
            Err(_) => None,
        }
    })
}
