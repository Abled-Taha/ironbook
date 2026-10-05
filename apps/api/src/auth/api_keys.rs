use crate::{db::api_keys::ApiKeyRecord, errors::AppError, state::AppState};
use hmac::{Hmac, KeyInit, Mac};
use rand::distr::{Alphanumeric, SampleString};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

const KEY_ID_LENGTH: usize = 16;
const SECRET_LENGTH: usize = 48;
const KEY_PREFIX: &str = "ibk";

#[derive(Debug)]
pub struct GeneratedApiKey {
    pub key_id: String,
    pub token: String,
    pub secret_hash: String,
}

#[derive(Debug)]
pub struct ParsedApiKey<'a> {
    pub key_id: &'a str,
    pub secret: &'a str,
}

pub fn generate() -> GeneratedApiKey {
    let mut rng = rand::rng();

    let key_id = Alphanumeric.sample_string(&mut rng, KEY_ID_LENGTH);
    let secret = Alphanumeric.sample_string(&mut rng, SECRET_LENGTH);

    let token = format!("{KEY_PREFIX}_{key_id}.{secret}");
    let secret_hash = hash_secret(&secret);

    GeneratedApiKey {
        key_id,
        token,
        secret_hash,
    }
}

pub fn hash_secret(secret: &str) -> String {
    let hmac_secret = std::env::var("API_KEY_HMAC_SECRET")
        .expect("API_KEY_HMAC_SECRET environment variable is not set");

    let mut mac = HmacSha256::new_from_slice(hmac_secret.as_bytes())
        .expect("HMAC accepts arbitrary key lengths");

    mac.update(secret.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

pub fn verify_secret(secret: &str, expected_hash: &str) -> bool {
    let hmac_secret = match std::env::var("API_KEY_HMAC_SECRET") {
        Ok(value) => value,
        Err(_) => return false,
    };

    let mut mac = match HmacSha256::new_from_slice(hmac_secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => return false,
    };

    mac.update(secret.as_bytes());

    let expected = match hex::decode(expected_hash) {
        Ok(value) => value,
        Err(_) => return false,
    };

    mac.verify_slice(&expected).is_ok()
}

pub fn parse(token: &str) -> Option<ParsedApiKey<'_>> {
    let token = token.strip_prefix("ibk_")?;

    let (key_id, secret) = token.split_once('.')?;

    if key_id.is_empty() || secret.is_empty() {
        return None;
    }

    Some(ParsedApiKey { key_id, secret })
}

pub async fn authenticate(state: &AppState, token: &str) -> Result<ApiKeyRecord, AppError> {
    let parsed = parse(token).ok_or(AppError::InvalidApiKey)?;

    let record = crate::db::api_keys::get_by_key_id(state, parsed.key_id)
        .await?
        .ok_or(AppError::InvalidApiKey)?;

    if record.revoked_at.is_some() {
        return Err(AppError::InvalidApiKey);
    }

    if let Some(expires_at) = record.expires_at
        && expires_at <= chrono::Utc::now()
    {
        return Err(AppError::InvalidApiKey);
    }

    if !verify_secret(parsed.secret, &record.secret_hash) {
        return Err(AppError::InvalidApiKey);
    }

    Ok(record)
}
