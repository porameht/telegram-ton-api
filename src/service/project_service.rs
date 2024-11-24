use mongodb::bson::oid::ObjectId;
use crate::{
    models::project::Project,
    repository::project_repository::ProjectRepository,
    error::ApiError,
};

#[derive(Clone)]
pub struct ProjectService {
    repository: ProjectRepository,
}

impl ProjectService {
    pub fn new(repository: ProjectRepository) -> Self {
        Self { repository }
    }

    pub async fn create_project(&self, mut project: Project) -> Result<Project, ApiError> {
        // Add business logic here
        project.created_at = chrono::Utc::now();
        project.updated_at = chrono::Utc::now();
        
        // Additional validation could go here
        if project.name.is_empty() {
            return Err(ApiError::BadRequest("Project name cannot be empty".to_string()));
        }
        
        self.repository.create(project).await
    }

    pub async fn update_project(&self, id: &ObjectId, mut project: Project) -> Result<Project, ApiError> {
        // Add business logic here
        project.updated_at = chrono::Utc::now();
        
        // Additional validation could go here
        if project.name.is_empty() {
            return Err(ApiError::BadRequest("Project name cannot be empty".to_string()));
        }
        
        self.repository.update(id, project).await
    }

    pub async fn delete_project(&self, id: &ObjectId) -> Result<bool, ApiError> {
        self.repository.delete(id).await
    }

    pub async fn get_project(&self, id: &ObjectId) -> Result<Project, ApiError> {
        self.repository.get_by_id(id).await
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, ApiError> {
        self.repository.get_all().await
    }
} 