use core::domain::{
    errors::GetAllProjectsError,
    models::{GetAllProjectsQuery, GetAllProjectsQueryResult},
};
use infra::app::App;

pub async fn run() {
    let app = App::new("sqlite://.backlog.db").await;
    let projects: Result<Vec<GetAllProjectsQueryResult>, GetAllProjectsError> =
        app.get_all_projects(GetAllProjectsQuery {}).await;

    match projects {
        Ok(projects) => print_projects(projects).await,
        Err(e) => eprintln!("Failed to get projects: {:?}", e),
    }
}

async fn print_projects(projects: Vec<GetAllProjectsQueryResult>) {
    projects.into_iter().for_each(|project| {
        println!("{}-{}, {}", project.key, project.name, project.id);
    });
}
