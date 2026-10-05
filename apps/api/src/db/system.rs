use sqlx::query_scalar;
use sqlx::{Postgres, Transaction};

use crate::errors::AppError;
use crate::services::system;
use crate::state::AppState;

pub async fn is_first_start(state: &AppState) -> Result<bool, AppError> {
    let count = query_scalar!(
        r#"
        SELECT COUNT(*) FROM clients
        "#
    )
    .fetch_one(&state.db)
    .await?;

    Ok(count.unwrap_or(0) == 0)
}

pub async fn get_key_id_by_name(state: &AppState, name: &str) -> Result<Option<String>, AppError> {
    let key_id = sqlx::query_scalar!(
        r#"
        SELECT key_id
        FROM clients
        WHERE name = $1
        "#,
        name.trim()
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(key_id)
}

pub async fn get_key_id_by_owner_email(
    state: &AppState,
    owner_email: &str,
) -> Result<Option<String>, AppError> {
    let key_id = sqlx::query_scalar!(
        r#"
        SELECT key_id
        FROM clients
        WHERE owner_email = $1
        "#,
        owner_email.trim()
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(key_id)
}

pub async fn store_api_key(
    state: &AppState,
    data: system::ApiKeyRequest,
    key_id: &str,
    secret_hash: &str,
) -> Result<bool, AppError> {
    let mut tx: Transaction<'_, Postgres> = state.db.begin().await?;

    sqlx::query!(
        r#"
        INSERT INTO clients (
            name,
            owner_email,
            key_id,
            secret_hash
        )
        VALUES ($1, $2, $3, $4)
        "#,
        data.name,
        data.owner_email,
        key_id,
        secret_hash,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(true)
}
