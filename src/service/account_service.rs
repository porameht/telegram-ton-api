use mongodb::bson::oid::ObjectId;
use crate::{
    models::account::Account,
    repository::account_repository::AccountRepository,
    error::ApiError,
};

#[derive(Clone)]
pub struct AccountService {
    repository: AccountRepository,
}

impl AccountService {
    pub fn new(repository: AccountRepository) -> Self {
        Self { repository }
    }

    pub async fn create_account(&self, mut account: Account) -> Result<Account, ApiError> {
        // Add business logic here
        account.created_at = chrono::Utc::now();
        account.updated_at = chrono::Utc::now();
        
        // Validation
        if account.email.is_empty() {
            return Err(ApiError::BadRequest("Email cannot be empty".to_string()));
        }
        if account.wallet_address.is_empty() {
            return Err(ApiError::BadRequest("Wallet address cannot be empty".to_string()));
        }
        
        // Additional business logic could go here
        // For example, check if email is unique
        if let Ok(existing_accounts) = self.repository.get_all().await {
            if existing_accounts.iter().any(|a| a.email == account.email) {
                return Err(ApiError::BadRequest("Email already exists".to_string()));
            }
        }
        
        self.repository.create(account).await
    }

    pub async fn update_account(&self, id: &ObjectId, mut account: Account) -> Result<Account, ApiError> {
        account.updated_at = chrono::Utc::now();
        
        // Validation
        if account.email.is_empty() {
            return Err(ApiError::BadRequest("Email cannot be empty".to_string()));
        }
        if account.wallet_address.is_empty() {
            return Err(ApiError::BadRequest("Wallet address cannot be empty".to_string()));
        }
        
        self.repository.update(id, account).await
    }

    pub async fn delete_account(&self, id: &ObjectId) -> Result<bool, ApiError> {
        // Add any deletion-specific business logic here
        // For example, check if the account has any associated projects
        self.repository.delete(id).await
    }

    pub async fn get_account(&self, id: &ObjectId) -> Result<Account, ApiError> {
        self.repository.get_by_id(id).await
    }

    pub async fn get_all_accounts(&self) -> Result<Vec<Account>, ApiError> {
        self.repository.get_all().await
    }
} 