use crate::{
    project::{DeletedProject, Project, ProjectBuilder, ProjectDraft, ProjectId, ProjectPatch},
    storage::traits::{ProjectQuery, ProjectStorage, Storage},
    ProjectStorageError,
};

pub struct ProjectRepository<T>
where
    T: Storage,
{
    storage: T,
}

impl<T> ProjectRepository<T>
where
    T: Storage,
{
    pub fn new(storage: T) -> Self {
        Self { storage }
    }
}

impl<T> ProjectStorage for ProjectRepository<T>
where
    T: Storage,
{
    fn get_project(mut self, id: ProjectId) -> Result<Project, ProjectStorageError> {
        self.storage.query_one(ProjectQuery::GetProject(id))
    }

    fn get_projects(mut self) -> Result<Vec<Project>, ProjectStorageError> {
        self.storage.query_many(ProjectQuery::GetProjects)
    }

    fn create_project(&mut self, draft: ProjectDraft) -> Result<ProjectId, ProjectStorageError> {
        let draft = ProjectBuilder::new()
            .key(draft.key.to_string())
            .name(draft.name.to_string())
            .description(draft.description.to_string())
            .build()
            .map_err(|_| ProjectStorageError::ProjectCreationFailed)?;
        self.storage.create(ProjectQuery::CreateProject(draft))
    }

    fn update_project(
        &mut self,
        id: ProjectId,
        patch: ProjectPatch,
    ) -> Result<Project, ProjectStorageError> {
        match self.storage.query_one(ProjectQuery::GetProject(id)) {
            Ok(existing) => {
                let builder = ProjectBuilder::from_project(existing.to_owned());
                let project = builder
                    .key(patch.key.unwrap_or(existing.key().to_owned()).to_string())
                    .name(patch.name.unwrap_or(existing.name().to_owned()).to_string())
                    .description(
                        patch
                            .description
                            .unwrap_or(existing.description().to_owned())
                            .to_string(),
                    )
                    .build()
                    .map_err(|_| ProjectStorageError::ProjectUpdateFailed)?;
                self.storage
                    .update(ProjectQuery::UpdateProject(id, project))
            }
            Err(e) => Err(e),
        }
    }

    fn delete_project(&mut self, id: ProjectId) -> Result<DeletedProject, ProjectStorageError> {
        self.storage.delete(ProjectQuery::DeleteProject(id))
    }
}
