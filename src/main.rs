mod error;
mod handlers;
mod models;
mod repository;
mod logger;
mod service;

use log::info;
use axum::{
    routing::{get, post, delete, put},
    Router,
};
use dotenv::dotenv;
use mongodb::{Client, Database};
use std::env;
use tower_http::cors::CorsLayer;

use crate::handlers::project_handler::{
    create_project, delete_project, get_all_projects, get_project, update_project,
};
use crate::handlers::account_handler::{
    create_account, delete_account, get_all_accounts, get_account, update_account,
};
use crate::repository::project_repository::ProjectRepository;
use crate::repository::account_repository::AccountRepository;
use crate::service::project_service::ProjectService;
use crate::service::account_service::AccountService;

async fn create_db_client() -> Database {
    let mongodb_uri = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to create MongoDB client");
    
    client.database(&database_name)
}

#[tokio::main]
async fn main() {
    logger::init_logger();
    dotenv().ok();
    info!("Environment variables loaded");
    
    let db = create_db_client().await;
    info!("Database connection established");
    
    let project_repository = ProjectRepository::new(db.clone());
    let project_service = ProjectService::new(project_repository);
    
    let account_repository = AccountRepository::new(db.clone());
    let account_service = AccountService::new(account_repository);

    let cors = CorsLayer::permissive();

    let project_routes = Router::new()
        .route("/projects", post(create_project))
        .route("/projects", get(get_all_projects))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id", put(update_project))
        .route("/projects/:id", delete(delete_project))
        .with_state(project_service);

    let account_routes = Router::new()
        .route("/accounts", post(create_account))
        .route("/accounts", get(get_all_accounts))
        .route("/accounts/:id", get(get_account))
        .route("/accounts/:id", put(update_account))
        .route("/accounts/:id", delete(delete_account))
        .with_state(account_service);

    let app = project_routes
        .merge(account_routes)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}