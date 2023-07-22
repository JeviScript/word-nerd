use std::collections::BTreeMap;

use base64::Engine;
use jwt::{Header, PKeyWithDigest, Token, Verified, VerifyWithKey};
use openssl::{bn::BigNum, hash::MessageDigest, pkey::PKey, rsa::Rsa};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

const GOOGLE_DISCOVERY_DOC_URL: &str =
    "https://accounts.google.com/.well-known/openid-configuration";

#[derive(Debug)]
pub struct GoogleUser {
    // unique per google account
    pub google_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

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

enum HttpError {
    RequestErr(String),
    ParseErr(String),
}

impl ToString for HttpError {
    fn to_string(&self) -> String {
        match self {
            HttpError::RequestErr(inner) => format!("RequestErr: {}", inner),
            HttpError::ParseErr(inner) => format!("ParseErr: {}", inner),
        }
    }
}

async fn get<T: DeserializeOwned>(url: &str) -> Result<T, HttpError> {
    let to_err = |url: &str, inner: String| -> String {
        format!("failed for url: {}, Inner: {}", url, inner)
    };

    let res = reqwest::get(url)
        .await
        .map_err(|err| HttpError::RequestErr(to_err(url, err.to_string())))?
        .json::<T>()
        .await
        .map_err(|err| HttpError::ParseErr(to_err(url, err.to_string())))?;
    Ok(res)
}

async fn get_discovery_document() -> Result<DiscoveryDocument, HttpError> {
    get(GOOGLE_DISCOVERY_DOC_URL).await
}

// TODO capture interal errors instead of Strings
#[derive(Debug)]
pub enum VerifyTokenErr {
    GetDiscoveryDocumentErr(String),
    GetCertificatesErr(String),
    NoSupportedAlgCertErr(String),
    DecodeEValueErr(String),
    DecodeNValueErr(String),
    ParseIntoBigNumErr(String),
    CreateRSAErr(String),
    CreatePKeyErr(String),
    VerificationErr(jwt::error::Error),
    NoGoogleId,
}

pub async fn verify_token(token: String) -> Result<GoogleUser, VerifyTokenErr> {
    let doc = get_discovery_document()
        .await
        .map_err(|err| VerifyTokenErr::GetDiscoveryDocumentErr(err.to_string()))?;

    let cert = get::<OAuthCert>(&doc.jwks_uri)
        .await
        .map_err(|err| VerifyTokenErr::GetCertificatesErr(err.to_string()))?;

    let verify_result = cert
        .keys
        .iter()
        .find_map(|cert_key| {
            if cert_key.alg != "RS256" {
                return None;
            };
            try_verify(cert_key, &token).ok()
        })
        .ok_or_else(|| {
            VerifyTokenErr::NoSupportedAlgCertErr(
                "No supported certificare was found with RS256 key algorithm".to_owned(),
            )
        })?;

    let claims = verify_result.claims();
    let google_id = claims.get("sub").ok_or(VerifyTokenErr::NoGoogleId)?;
    let google_user = GoogleUser {
        google_id: google_id.clone().to_string().remove_quotes(),
        first_name: or_default(claims.get("given_name")),
        last_name: or_default(claims.get("family_name")),
        email: or_default(claims.get("email")),
    };

    Ok(google_user)
}

fn try_verify(
    cert_key: &OAuthCertKey,
    token: &str,
) -> Result<Token<Header, BTreeMap<String, Value>, Verified>, VerifyTokenErr> {
    let e_decoded = &base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(&cert_key.e)
        .map_err(|err| {
            VerifyTokenErr::DecodeEValueErr(format!(
                "failed to decode e value {}. Inner: {}",
                cert_key.e, err
            ))
        })?;

    let e = BigNum::from_slice(e_decoded).map_err(|err| {
        VerifyTokenErr::ParseIntoBigNumErr(format!(
            "Failed to parse 'e' value into bignum. Inner: {}",
            err
        ))
    })?;

    let n_decoded = &base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(&cert_key.n)
        .map_err(|err| {
            VerifyTokenErr::DecodeNValueErr(format!(
                "failed to decode n value {}. Inner {}",
                cert_key.n, err
            ))
        })?;

    let n = BigNum::from_slice(n_decoded).map_err(|err| {
        VerifyTokenErr::ParseIntoBigNumErr(format!(
            "Failed to parse 'n' value into bignum. Inner: {}",
            err
        ))
    })?;

    let rsa = Rsa::from_public_components(n, e).map_err(|err| {
        VerifyTokenErr::CreateRSAErr(format!("failed to create Rsa from n and e. {}", err))
    })?;

    let pkey = PKey::from_rsa(rsa)
        .map_err(|err| VerifyTokenErr::CreatePKeyErr(format!("failed to create pkey. {}", err)))?;

    let rs256_public_key = PKeyWithDigest {
        digest: MessageDigest::sha256(),
        key: pkey,
    };

    token
        .verify_with_key(&rs256_public_key)
        .map_err(VerifyTokenErr::VerificationErr)
}

fn or_default(option: Option<&Value>) -> String {
    option.unwrap_or(&"".into()).to_string().remove_quotes()
}

// For some reason JWT lib returns Strings with additional surrounded quotes
trait RemoveQuotes {
    fn remove_quotes(&self) -> String;
}

impl RemoveQuotes for String {
    fn remove_quotes(&self) -> String {
        self.replace('"', "")
    }
}
