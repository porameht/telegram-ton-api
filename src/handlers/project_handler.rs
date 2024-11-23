use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::oid::ObjectId;

use crate::{models::project::Project, repository::project_repository::ProjectRepository, error::ApiError};

pub async fn create_project(
    State(repo): State<ProjectRepository>,
    Json(mut project): Json<Project>,
) -> Result<Json<Project>, ApiError> {
    project.created_at = chrono::Utc::now();
    project.updated_at = chrono::Utc::now();
    let project = repo.create(project).await?;
    Ok(Json(project))
}

pub async fn update_project(
    State(repo): State<ProjectRepository>,
    Path(id): Path<String>,
    Json(mut project): Json<Project>,
) -> Result<Json<Project>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    project.updated_at = chrono::Utc::now();
    let project = repo.update(&object_id, project).await?;
    Ok(Json(project))
}

pub async fn delete_project(
    State(repo): State<ProjectRepository>,
    Path(id): Path<String>,
) -> Result<Json<bool>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let result = repo.delete(&object_id).await?;
    Ok(Json(result))
}

pub async fn get_project(
    State(repo): State<ProjectRepository>,
    Path(id): Path<String>,
) -> Result<Json<Project>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    println!("Getting project with ID: {}", id);
    let project = repo.get_by_id(&object_id).await?;

    println!("Project: {:?}", project);
    Ok(Json(project))
}

pub async fn get_all_projects(
    State(repo): State<ProjectRepository>,
) -> Result<Json<Vec<Project>>, ApiError> {
    println!("Getting all projects");
    let projects = repo.get_all().await?;
    println!("Projects: {:?}", projects);
    Ok(Json(projects))
} 