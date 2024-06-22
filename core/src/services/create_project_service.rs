use crate::{
    interfaces::{
        ports::{CreateProjectCommand, CreateProjectCommandHandler, CreateProjectRepository},
        use_cases::CreateProjectUseCase,
    },
    models::{dto::ProjectDraft, errors::CreateProjectError},
    use_cases::projects::CreateProject,
};

pub struct CreateProjectService<T: CreateProjectRepository> {
    create_project: CreateProject<T>,
}

impl<T: CreateProjectRepository> CreateProjectService<T> {
    pub fn new(repository: T) -> Self {
        Self {
            create_project: CreateProject::new(repository),
        }
    }
}

impl<T: CreateProjectRepository> CreateProjectCommandHandler for CreateProjectService<T> {
    async fn handle(&self, command: CreateProjectCommand) -> Result<(), CreateProjectError> {
        match self
            .create_project
            .execute(ProjectDraft::from(command))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
