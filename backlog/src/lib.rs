pub mod builder;
pub mod config;
pub mod error;
pub mod project;
pub mod repository;
pub mod storage;
pub mod prelude {
    pub use crate::error::*;
    pub use crate::project::error::*;
    pub use crate::storage::error::*;
}

use crate::repository::ProjectRepository;
use config::BacklogConfig;
use prelude::*;
use storage::traits::Storage;

pub struct Backlog<T>
where
    T: Storage,
{
    _config: BacklogConfig,
    _projects: ProjectRepository<T>,
}

impl<T> Backlog<T>
where
    T: Storage,
{
    pub fn new(config: BacklogConfig, projects: ProjectRepository<T>) -> Self {
        Self {
            _config: config,
            _projects: projects,
        }
    }
}

#[cfg(test)]
mod tests {
    use expanduser::expanduser;

    #[test]
    fn test_backlog_path_expansion() {
        let path = "~/.backlog/backlog.yaml";
        let expanded = expanduser(path).unwrap();
        assert_eq!(
            expanded.display().to_string(),
            "/Users/sbeardsley/.backlog/backlog.yaml"
        );
    }

    #[test]
    fn test_backlog_path_dirname() {
        let path = "~/.backlog/backlog.yaml";
        let expanded = expanduser(path).unwrap();
        let dirname = expanded.parent().unwrap();
        assert_eq!(dirname.display().to_string(), "/Users/sbeardsley/.backlog");
    }

    #[test]
    fn test_backlog_path_dirname_double_expansion() {
        let path = "~/.backlog/backlog.yaml";
        let expanded = expanduser(path).unwrap();
        let dirname = expanduser(expanded.parent().unwrap().display().to_string()).unwrap();
        assert_eq!(dirname.display().to_string(), "/Users/sbeardsley/.backlog");
    }
}
