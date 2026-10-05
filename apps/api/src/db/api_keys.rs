use crate::{errors::AppError, state::AppState};

#[derive(Debug)]
pub struct ApiKeyRecord {
    pub id: i64,
    pub key_id: String,
    pub secret_hash: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_by_key_id(
    state: &AppState,
    key_id: &str,
) -> Result<Option<ApiKeyRecord>, AppError> {
    let record = sqlx::query!(
        r#"
        SELECT
            id,
            key_id,
            secret_hash,
            scopes,
            expires_at,
            revoked_at
        FROM clients
        WHERE key_id = $1
        "#,
        key_id
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(record.map(|record| ApiKeyRecord {
        id: record.id,
        key_id: record.key_id,
        secret_hash: record.secret_hash,
        scopes: record.scopes,
        expires_at: record.expires_at,
        revoked_at: record.revoked_at,
    }))
}
