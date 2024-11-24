use chrono::Utc;
use mongodb::{
    bson::oid::ObjectId,
    Client,
    Database,
};
use dotenv::dotenv;

use crate::{
    models::account::Account,
    repository::account_repository::AccountRepository,
};

async fn setup_test_db() -> Database {
    // Load environment variables from .env file
    dotenv().ok();
    
    let mongodb_uri = std::env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .expect("Failed to create MongoDB client");
    
    // Use a test database to avoid conflicts with production
    client.database("test_database")
}

fn create_test_account() -> Account {
    Account {
        id: None,
        wallet_address: "0x123456789".to_string(),
        email: "test@example.com".to_string(),
        account_name: "Test Account".to_string(),
        project_ids: vec![],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[tokio::test]
async fn test_crud_operations() {
    // Setup
    let db = setup_test_db().await;
    let repo = AccountRepository::new(db.clone());
    
    // Clean up any existing test data
    db.collection::<mongodb::bson::Document>("accounts")
        .drop(None)
        .await
        .expect("Failed to drop collection");

    // Test Create
    let test_account = create_test_account();
    let created_account = repo.create(test_account.clone())
        .await
        .expect("Failed to create account");
    
    assert!(created_account.id.is_some());
    assert_eq!(created_account.email, test_account.email);
    assert_eq!(created_account.wallet_address, test_account.wallet_address);

    // Test Get by ID
    let account_id = created_account.id.unwrap();
    let retrieved_account = repo.get_by_id(&account_id)
        .await
        .expect("Failed to get account by ID");
    
    assert_eq!(retrieved_account.id, Some(account_id));
    assert_eq!(retrieved_account.email, test_account.email);

    // Test Update
    let mut updated_account = retrieved_account.clone();
    updated_account.account_name = "Updated Test Account".to_string();
    
    let result = repo.update(&account_id, updated_account.clone())
        .await
        .expect("Failed to update account");
    
    assert_eq!(result.account_name, "Updated Test Account");

    // Test Get All
    let all_accounts = repo.get_all()
        .await
        .expect("Failed to get all accounts");
    
    assert_eq!(all_accounts.len(), 1);
    assert_eq!(all_accounts[0].id, Some(account_id));

    // Test Delete
    let delete_result = repo.delete(&account_id)
        .await
        .expect("Failed to delete account");
    
    assert!(delete_result);

    // Verify deletion
    let all_accounts_after_delete = repo.get_all()
        .await
        .expect("Failed to get all accounts after delete");
    
    assert!(all_accounts_after_delete.is_empty());
}

#[tokio::test]
async fn test_get_nonexistent_account() {
    let db = setup_test_db().await;
    let repo = AccountRepository::new(db);
    
    let nonexistent_id = ObjectId::new();
    let result = repo.get_by_id(&nonexistent_id).await;
    
    assert!(matches!(result, Err(crate::error::ApiError::NotFound)));
}

#[tokio::test]
async fn test_update_nonexistent_account() {
    let db = setup_test_db().await;
    let repo = AccountRepository::new(db);
    
    let nonexistent_id = ObjectId::new();
    let test_account = create_test_account();
    
    let result = repo.update(&nonexistent_id, test_account).await;
    
    assert!(matches!(result, Err(crate::error::ApiError::NotFound)));
}

#[tokio::test]
async fn test_delete_nonexistent_account() {
    let db = setup_test_db().await;
    let repo = AccountRepository::new(db);
    
    let nonexistent_id = ObjectId::new();
    let result = repo.delete(&nonexistent_id).await;
    
    assert!(matches!(result, Ok(false)));
} 