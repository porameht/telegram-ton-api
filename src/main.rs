mod error;
mod handlers;
mod models;
mod repository;
mod logger;

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
use crate::repository::project_repository::ProjectRepository;

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
    
    let project_repository = ProjectRepository::new(db);

    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/projects", post(create_project))
        .route("/projects", get(get_all_projects))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id", put(update_project))
        .route("/projects/:id", delete(delete_project))
        .with_state(project_repository)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}