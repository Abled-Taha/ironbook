use crate::auth::api_keys;
use crate::db::system;
use crate::errors::AppError;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize)]
pub struct GreetResponse {
    pub message: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct HealthReportResponse {
    pub overall: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyRequest {
    pub name: String,
    pub owner_email: String,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub token: String,
}

pub async fn greet(_state: &AppState) -> Result<GreetResponse, AppError> {
    info!("Serving greet health check");

    Ok(GreetResponse {
        message: String::from("Hello, World!"),
        status: String::from("success"),
    })
}

pub async fn health_report(_state: &AppState) -> Result<HealthReportResponse, AppError> {
    info!("Serving system health report check");

    Ok(HealthReportResponse {
        overall: String::from("All OK!"),
    })
}

pub async fn generate_api_key(
    state: &AppState,
    apk_key_opt: Option<&str>,
    data: ApiKeyRequest,
) -> Result<ApiKeyResponse, AppError> {
    info!(
        token_name = %data.name,
        owner_email = %data.owner_email,
        "Processing API key generation request"
    );

    async fn generate_api_key_inner(
        state: &AppState,
        data: ApiKeyRequest,
    ) -> Result<String, AppError> {
        if data.name.trim().is_empty() {
            return Err(AppError::InvalidName);
        }
        if data.owner_email.trim().is_empty() {
            return Err(AppError::InvalidOwnerEmail);
        }
        if system::get_key_id_by_name(state, &data.name)
            .await?
            .is_some()
        {
            return Err(AppError::ApiKeyNameAlreadyExists);
        }

        let generated = api_keys::generate();

        system::store_api_key(state, data, &generated.key_id, &generated.secret_hash).await?;

        info!(
            key_id = %generated.key_id,
            "Successfully generated and stored new API key"
        );

        Ok(generated.token)
    }

    if system::is_first_start(state).await? {
        info!("Initial setup detected (first start): bypassing API key verification");
        let token = generate_api_key_inner(state, data).await?;
        Ok(ApiKeyResponse { token })
    } else {
        let apk_key = apk_key_opt.unwrap_or_default();

        api_keys::authenticate(state, apk_key).await?;

        let token = generate_api_key_inner(state, data).await?;
        Ok(ApiKeyResponse { token })
    }
}
