pub trait ListProjectsUseCase: Send + Sync {
    fn list_projects(
        &self,
    ) -> impl std::future::Future<Output = Result<NewProject, ListProjectError>> + Send;
}
