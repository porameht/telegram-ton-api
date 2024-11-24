use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::oid::ObjectId;

use crate::{
    models::account::Account,
    service::account_service::AccountService,
    error::ApiError,
};

pub async fn create_account(
    State(service): State<AccountService>,
    Json(account): Json<Account>,
) -> Result<Json<Account>, ApiError> {
    let account = service.create_account(account).await?;
    Ok(Json(account))
}

pub async fn update_account(
    State(service): State<AccountService>,
    Path(id): Path<String>,
    Json(account): Json<Account>,
) -> Result<Json<Account>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let account = service.update_account(&object_id, account).await?;
    Ok(Json(account))
}

pub async fn delete_account(
    State(service): State<AccountService>,
    Path(id): Path<String>,
) -> Result<Json<bool>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let result = service.delete_account(&object_id).await?;
    Ok(Json(result))
}

pub async fn get_account(
    State(service): State<AccountService>,
    Path(id): Path<String>,
) -> Result<Json<Account>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let account = service.get_account(&object_id).await?;
    Ok(Json(account))
}

pub async fn get_all_accounts(
    State(service): State<AccountService>,
) -> Result<Json<Vec<Account>>, ApiError> {
    let accounts = service.get_all_accounts().await?;
    Ok(Json(accounts))
}