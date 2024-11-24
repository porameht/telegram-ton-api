use chrono::Utc;
use mongodb::{
    bson::oid::ObjectId,
    Client,
    Database,
};
use std::collections::HashMap;
use dotenv::dotenv;

use crate::{
    models::project::{Project, Package, FacebookCredential},
    repository::project_repository::ProjectRepository,
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

fn create_test_project() -> Project {
    let mut facebook_credentials = HashMap::new();
    facebook_credentials.insert(
        "test_page".to_string(),
        FacebookCredential {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            access_token: "test_token".to_string(),
            ad_account_id: "test_ad_account_id".to_string(),
            account_suffix: "test_suffix".to_string(),
            pixel_id: None,
            link_url: None,
            page_id: Some("test_page_id".to_string()),
            watermark: None,
        },
    );

    Project {
        id: None,
        name: "Test Project".to_string(),
        telegram_chat_id: Some("123456789".to_string()),
        facebook_credentials,
        package: Some(Package {
            name: "Test Package".to_string(),
            description: "Test Description".to_string(),
        }),
        expires_at: Some(Utc::now()),
        is_active: true,
        is_logging: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[tokio::test]
async fn test_crud_operations() {
    // Setup
    let db = setup_test_db().await;
    let repo = ProjectRepository::new(db.clone());
    
    // Clean up any existing test data
    db.collection::<mongodb::bson::Document>("projects")
        .drop(None)
        .await
        .expect("Failed to drop collection");

    // Test Create
    let test_project = create_test_project();
    let created_project = repo.create(test_project.clone())
        .await
        .expect("Failed to create project");
    
    assert!(created_project.id.is_some());
    assert_eq!(created_project.name, test_project.name);

    // Test Get by ID
    let project_id = created_project.id.unwrap();
    let retrieved_project = repo.get_by_id(&project_id)
        .await
        .expect("Failed to get project by ID");
    
    assert_eq!(retrieved_project.id, Some(project_id));
    assert_eq!(retrieved_project.name, test_project.name);

    // Test Update
    let mut updated_project = retrieved_project.clone();
    updated_project.name = "Updated Test Project".to_string();
    
    let result = repo.update(&project_id, updated_project.clone())
        .await
        .expect("Failed to update project");
    
    assert_eq!(result.name, "Updated Test Project");

    // Test Get All
    let all_projects = repo.get_all()
        .await
        .expect("Failed to get all projects");
    
    assert_eq!(all_projects.len(), 1);
    assert_eq!(all_projects[0].id, Some(project_id));

    // Test Delete
    let delete_result = repo.delete(&project_id)
        .await
        .expect("Failed to delete project");
    
    assert!(delete_result);

    // Verify deletion
    let all_projects_after_delete = repo.get_all()
        .await
        .expect("Failed to get all projects after delete");
    
    assert!(all_projects_after_delete.is_empty());
}

#[tokio::test]
async fn test_get_nonexistent_project() {
    let db = setup_test_db().await;
    let repo = ProjectRepository::new(db);
    
    let nonexistent_id = ObjectId::new();
    let result = repo.get_by_id(&nonexistent_id).await;
    
    assert!(matches!(result, Err(crate::error::ApiError::NotFound)));
}

#[tokio::test]
async fn test_update_nonexistent_project() {
    let db = setup_test_db().await;
    let repo = ProjectRepository::new(db);
    
    let nonexistent_id = ObjectId::new();
    let test_project = create_test_project();
    
    let result = repo.update(&nonexistent_id, test_project).await;
    
    assert!(matches!(result, Err(crate::error::ApiError::NotFound)));
}

#[tokio::test]
async fn test_delete_nonexistent_project() {
    let db = setup_test_db().await;
    let repo = ProjectRepository::new(db);
    
    let nonexistent_id = ObjectId::new();
    let result = repo.delete(&nonexistent_id).await;
    
    assert!(matches!(result, Ok(false)));
} 