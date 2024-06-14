use std::fs::{read_to_string, write, DirBuilder};

use crate::BacklogError;
use expanduser::expanduser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum StorageType {
    #[default]
    FileStorage,
    DatabaseStorage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum FileType {
    #[default]
    Yaml,
    Json,
    Toml,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub enum DatabaseType {
    #[default]
    SurrealDb,
    Postgres,
    Mysql,
    Sqlite,
    RocksDb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklogConfig {
    storage_type: StorageType,
    file_format: FileType,
    file_storage_path: String,
    database_type: DatabaseType,
    database_url: String,
}

impl Default for BacklogConfig {
    fn default() -> Self {
        Self {
            storage_type: StorageType::FileStorage,
            file_format: FileType::Yaml,
            file_storage_path: "~/.backlog/backlog.yaml".to_string(),
            database_type: DatabaseType::SurrealDb,
            database_url: String::new(),
        }
    }
}

impl BacklogConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn storage_type(&self) -> &StorageType {
        &self.storage_type
    }
    pub fn file_format(&self) -> &FileType {
        &self.file_format
    }
    pub fn file_storage_path(&self) -> &str {
        &self.file_storage_path
    }
    pub fn database_type(&self) -> &DatabaseType {
        &self.database_type
    }
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
    pub fn load() -> Result<Self, BacklogError> {
        let path = expanduser("~/.backlog/config.toml").map_err(|_| {
            BacklogError::BacklogConfigLoadError(format!(
                "Failed to load config: {}",
                "~/.backlog/config.toml"
            ))
        })?;
        read_to_string(path.display().to_string())
            .map(|data| {
                toml::from_str(&data).map_err(|e| {
                    BacklogError::BacklogConfigLoadError(format!("Failed to load config: {}", e))
                })
            })
            .map_err(|e| {
                BacklogError::BacklogConfigLoadError(format!("Failed to load config: {}", e))
            })?
    }

    pub fn save(&self) -> Result<(), BacklogError> {
        let path = expanduser("~/.backlog/config.toml").map_err(|_| {
            BacklogError::BacklogConfigLoadError(format!(
                "Failed to load config: {}",
                "~/.backlog/config.toml"
            ))
        })?;

        let dir_path = path.parent().ok_or_else(|| {
            BacklogError::BacklogConfigLoadError("Failed to get parent directory".to_string())
        })?;

        DirBuilder::new()
            .recursive(true)
            .create(dir_path.display().to_string())
            .map_err(|_| {
                BacklogError::BacklogConfigLoadError(format!(
                    "Failed to create directory: {}",
                    dir_path.display()
                ))
            })?;

        let data = toml::to_string(self).map_err(|_| {
            BacklogError::BacklogConfigSaveError("Failed to serialize config to toml".to_string())
        })?;

        write(&path, data).map_err(|_| {
            BacklogError::BacklogConfigSaveError(format!(
                "Failed to save config to file: {}",
                path.display()
            ))
        })?;

        Ok(())
    }

    pub fn use_file_storage(
        mut self,
        format: FileType,
        path: impl Into<String>,
    ) -> Result<Self, BacklogError> {
        self.storage_type = StorageType::FileStorage;
        self.file_format = format;
        let path_value = Into::<String>::into(path);
        let expanded = expanduser(&path_value).map_err(|_| {
            BacklogError::InvalidPath(format!("Failed to expand path: {}", &path_value))
        })?;
        self.file_storage_path = expanded.display().to_string();
        Ok(self)
    }

    pub fn use_database_storage(
        mut self,
        database_type: DatabaseType,
        url: impl Into<String>,
    ) -> Result<Self, BacklogError> {
        self.storage_type = StorageType::DatabaseStorage;
        self.database_type = database_type;
        self.database_url = Into::<String>::into(url);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::backlog_config::{BacklogConfig, StorageType};
    use crate::config::backlog_config::{DatabaseType, FileType};
    use expanduser::expanduser;

    #[test]
    fn test_backlog_config_default() {
        let config = BacklogConfig::default();
        assert_eq!(config.storage_type, StorageType::FileStorage);
        assert_eq!(config.file_format, FileType::Yaml);
        assert_eq!(config.file_storage_path, "~/.backlog/backlog.yaml");
        assert_eq!(config.database_type, DatabaseType::SurrealDb);
        assert_eq!(config.database_url, "");
    }

    #[test]
    fn test_backlog_config_use_file_json_storage() {
        let homedir = expanduser("~/").unwrap().display().to_string();
        let config = BacklogConfig::default();
        let new_config = config
            .use_file_storage(FileType::Json, "~/.backlog/test/backlog.json")
            .unwrap();
        assert_eq!(new_config.storage_type, StorageType::FileStorage);
        assert_eq!(new_config.file_format, FileType::Json);
        assert_eq!(
            new_config.file_storage_path,
            format!("{}.backlog/test/backlog.json", homedir)
        );
    }

    #[test]
    fn test_backlog_config_use_file_toml_storage() {
        let homedir = expanduser("~/").unwrap().display().to_string();
        let config = BacklogConfig::default();
        let new_config = config
            .use_file_storage(FileType::Toml, "~/.backlog/test/backlog.toml")
            .unwrap();
        assert_eq!(new_config.storage_type, StorageType::FileStorage);
        assert_eq!(new_config.file_format, FileType::Toml);
        assert_eq!(
            new_config.file_storage_path,
            format!("{}.backlog/test/backlog.toml", homedir)
        );
    }

    #[test]
    fn test_backlog_config_use_file_yaml_storage() {
        let homedir = expanduser("~/").unwrap().display().to_string();
        let config = BacklogConfig::default();
        let new_config = config
            .use_file_storage(FileType::Yaml, "~/.backlog/test/backlog.yaml")
            .unwrap();
        assert_eq!(new_config.storage_type, StorageType::FileStorage);
        assert_eq!(new_config.file_format, FileType::Yaml);
        assert_eq!(
            new_config.file_storage_path,
            format!("{}.backlog/test/backlog.yaml", homedir)
        );
    }

    #[test]
    fn test_backlog_config_use_database_postgres_storage() {
        let config = BacklogConfig::default();
        let new_config = config
            .use_database_storage(DatabaseType::Postgres, "postgres://localhost:5432/backlog")
            .unwrap();
        assert_eq!(new_config.storage_type, StorageType::DatabaseStorage);
        assert_eq!(new_config.database_type, DatabaseType::Postgres);
        assert_eq!(new_config.database_url, "postgres://localhost:5432/backlog");
    }

    #[test]
    fn test_backlog_config_use_database_mysql_storage() {
        let config = BacklogConfig::default();
        let new_config = config
            .use_database_storage(DatabaseType::Mysql, "mysql://localhost:3306/backlog")
            .unwrap();
        assert_eq!(new_config.storage_type, StorageType::DatabaseStorage);
        assert_eq!(new_config.database_type, DatabaseType::Mysql);
        assert_eq!(new_config.database_url, "mysql://localhost:3306/backlog");
    }

    #[test]
    fn test_backlog_config_use_database_sqlite_storage() {
        let config = BacklogConfig::default();
        let new_config = config
            .use_database_storage(DatabaseType::Sqlite, "sqlite://localhost:3306/backlog")
            .unwrap();
        assert_eq!(new_config.storage_type, StorageType::DatabaseStorage);
        assert_eq!(new_config.database_type, DatabaseType::Sqlite);
        assert_eq!(new_config.database_url, "sqlite://localhost:3306/backlog");
    }

    #[test]
    fn test_backlog_config_use_database_rocksdb_storage() {
        let config = BacklogConfig::default();
        let new_config = config
            .use_database_storage(DatabaseType::RocksDb, "rocksdb://localhost:3306/backlog")
            .unwrap();
        assert_eq!(new_config.storage_type, StorageType::DatabaseStorage);
        assert_eq!(new_config.database_type, DatabaseType::RocksDb);
        assert_eq!(new_config.database_url, "rocksdb://localhost:3306/backlog");
    }

    #[test]
    fn test_backlog_config_save() {
        let config = BacklogConfig::default();
        let result = config.save();
        assert!(result.is_ok());
    }
}
