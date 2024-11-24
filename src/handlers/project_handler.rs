use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::oid::ObjectId;

use crate::{
    models::project::Project, 
    service::project_service::ProjectService,
    error::ApiError
};

pub async fn create_project(
    State(service): State<ProjectService>,
    Json(project): Json<Project>,
) -> Result<Json<Project>, ApiError> {
    let project = service.create_project(project).await?;
    Ok(Json(project))
}

pub async fn update_project(
    State(service): State<ProjectService>,
    Path(id): Path<String>,
    Json(project): Json<Project>,
) -> Result<Json<Project>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let project = service.update_project(&object_id, project).await?;
    Ok(Json(project))
}

pub async fn delete_project(
    State(service): State<ProjectService>,
    Path(id): Path<String>,
) -> Result<Json<bool>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let result = service.delete_project(&object_id).await?;
    Ok(Json(result))
}

pub async fn get_project(
    State(service): State<ProjectService>,
    Path(id): Path<String>,
) -> Result<Json<Project>, ApiError> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;
    let project = service.get_project(&object_id).await?;
    Ok(Json(project))
}

pub async fn get_all_projects(
    State(service): State<ProjectService>,
) -> Result<Json<Vec<Project>>, ApiError> {
    let projects = service.get_all_projects().await?;
    Ok(Json(projects))
} 