use backlog::config::{BacklogConfig, DatabaseType, FileType};
use thiserror::Error;

use crate::cli::{DbType, FileFormat};

#[derive(Debug, Error)]

pub enum BacklogConfigError {
    #[error("Unable to configure backlog: {}", .0)]
    ConfigFileError(String),
}

pub struct BacklogApp {}

impl BacklogApp {
    pub fn config_file_storage(
        path: impl Into<String>,
        format: FileFormat,
    ) -> Result<(), BacklogConfigError> {
        let config = BacklogConfig::new()
            .use_file_storage(
                match format {
                    FileFormat::Json => FileType::Json,
                    FileFormat::Yaml => FileType::Yaml,
                    FileFormat::Toml => FileType::Toml,
                },
                path.into(),
            )
            .map_err(|e| BacklogConfigError::ConfigFileError(e.to_string()))?;
        config
            .save()
            .map_err(|e| BacklogConfigError::ConfigFileError(e.to_string()))?;
        Ok(())
    }

    pub fn config_db_storage(
        db_url: impl Into<String>,
        db_type: DbType,
    ) -> Result<(), BacklogConfigError> {
        let config = BacklogConfig::new()
            .use_database_storage(
                match db_type {
                    DbType::Surreal => DatabaseType::SurrealDb,
                    DbType::Postgres => DatabaseType::Postgres,
                    DbType::Mysql => DatabaseType::Mysql,
                    DbType::Sqlite => DatabaseType::Sqlite,
                    DbType::Rocks => DatabaseType::RocksDb,
                },
                db_url.into(),
            )
            .map_err(|e| BacklogConfigError::ConfigFileError(e.to_string()))?;
        config
            .save()
            .map_err(|e| BacklogConfigError::ConfigFileError(e.to_string()))?;
        Ok(())
    }
}
