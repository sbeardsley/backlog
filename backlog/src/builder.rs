// Region: BacklogBuilder States

use crate::{
    config::BacklogConfig,
    repository::ProjectRepository,
    storage::{
        file_storage::{yaml::Yaml, FileStorage},
        traits::Filestore,
    },
    Backlog,
};

#[derive(Default, Clone)]
pub struct NoConfig;

#[derive(Default, Clone)]
pub struct Config(BacklogConfig);

#[derive(Default, Clone)]
pub struct NoStorageBuilderType;

#[derive(Default, Clone)]
pub struct FileStorageBuilderType;

#[derive(Default, Clone)]
pub struct DbStorageBuilderType;

#[derive(Default, Clone)]
pub struct NoFileFormatBuilderType;

#[derive(Default, Clone)]
pub struct YamlFileFormatBuilderType(String);

#[derive(Default, Clone)]
pub struct JsonFileFormatBuilderType(String);

#[derive(Default, Clone)]
pub struct TomlFileFormatBuilderType(String);

#[derive(Default, Clone)]
pub struct NoDbBuilderType;

#[derive(Default, Clone)]
pub struct DbBuilderType;

#[derive(Default, Clone)]
pub struct SurrealDbBuilderType;

#[derive(Default, Clone)]
pub struct PostgresDbBuilderType;

#[derive(Default, Clone)]
pub struct MysqlDbBuilderType;

#[derive(Default, Clone)]
pub struct SqliteDbBuilderType;

#[derive(Default, Clone)]
pub struct RocksDbBuilderType;

// EndRegion: BacklogBuilder States
#[derive(Clone)]
pub struct BacklogBuilder<C, S, F, D> {
    config: C,
    storage: S,
    file_format: F,
    db_type: D,
}

impl Default
    for BacklogBuilder<NoConfig, NoStorageBuilderType, NoFileFormatBuilderType, NoDbBuilderType>
{
    fn default() -> Self {
        BacklogBuilder {
            config: NoConfig,
            storage: NoStorageBuilderType,
            file_format: NoFileFormatBuilderType,
            db_type: NoDbBuilderType,
        }
    }
}

impl BacklogBuilder<NoConfig, NoStorageBuilderType, NoFileFormatBuilderType, NoDbBuilderType> {
    pub fn new() -> Self {
        BacklogBuilder::default()
    }

    pub fn with_config(
        self,
        config: BacklogConfig,
    ) -> BacklogBuilder<Config, NoStorageBuilderType, NoFileFormatBuilderType, NoDbBuilderType>
    {
        BacklogBuilder {
            config: Config(config),
            storage: NoStorageBuilderType,
            file_format: NoFileFormatBuilderType,
            db_type: NoDbBuilderType,
        }
    }
}

impl BacklogBuilder<Config, NoStorageBuilderType, NoFileFormatBuilderType, NoDbBuilderType> {
    pub fn with_file_storage(
        self,
    ) -> BacklogBuilder<Config, FileStorageBuilderType, NoFileFormatBuilderType, NoDbBuilderType>
    {
        BacklogBuilder {
            config: self.config,
            storage: FileStorageBuilderType,
            file_format: NoFileFormatBuilderType,
            db_type: NoDbBuilderType,
        }
    }
    pub fn with_db_storage(
        self,
    ) -> BacklogBuilder<Config, DbStorageBuilderType, NoFileFormatBuilderType, NoDbBuilderType>
    {
        BacklogBuilder {
            config: self.config,
            storage: DbStorageBuilderType,
            file_format: NoFileFormatBuilderType,
            db_type: NoDbBuilderType,
        }
    }
}

impl BacklogBuilder<Config, FileStorageBuilderType, NoFileFormatBuilderType, NoDbBuilderType> {
    pub fn yaml_format(
        self,
        path: impl Into<String>,
    ) -> BacklogBuilder<Config, FileStorageBuilderType, YamlFileFormatBuilderType, NoDbBuilderType>
    {
        BacklogBuilder {
            config: self.config,
            storage: FileStorageBuilderType,
            file_format: YamlFileFormatBuilderType(path.into()),
            db_type: NoDbBuilderType,
        }
    }
}

impl BacklogBuilder<Config, FileStorageBuilderType, YamlFileFormatBuilderType, NoDbBuilderType> {
    pub fn build(self) -> Backlog<FileStorage<Yaml>> {
        let yaml = Yaml::new(self.file_format.0.as_str());
        let storage = FileStorage::new(yaml);
        let repository = ProjectRepository::new(storage);

        let backlog = Backlog::new(self.config.0, repository);
        backlog
    }
}
