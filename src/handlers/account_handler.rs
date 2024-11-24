use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::oid::ObjectId;

use crate::{models::account::Account, repository::account_repository::AccountRepository, error::ApiError};

pub async fn create_account(
    State(repo): State<AccountRepository>,
    Json(mut account): Json<Account>,
) -> Result<Json<Account>, ApiError> {
    account.created_at = chrono::Utc::now();
    account.updated_at = chrono::Utc::now();
    let account = repo.create(account).await?;
    Ok(Json(account))
}

pub async fn update_account(
    State(repo): State<AccountRepository>,
    Path(id): Path<String>,
    Json(mut account): Json<Account>,
) -> Result<Json<Account>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    account.updated_at = chrono::Utc::now();
    let account = repo.update(&object_id, account).await?;
    Ok(Json(account))
}

pub async fn delete_account(
    State(repo): State<AccountRepository>,
    Path(id): Path<String>,
) -> Result<Json<bool>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let result = repo.delete(&object_id).await?;
    Ok(Json(result))
}

pub async fn get_account(
    State(repo): State<AccountRepository>,
    Path(id): Path<String>,
) -> Result<Json<Account>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    let account = repo.get_by_id(&object_id).await?;
    Ok(Json(account))
}

pub async fn get_all_accounts(
    State(repo): State<AccountRepository>,
) -> Result<Json<Vec<Account>>, ApiError> {
    println!("Getting all accounts");
    let accounts = repo.get_all().await?;
    println!("Accounts: {:?}", accounts);
    Ok(Json(accounts))
}