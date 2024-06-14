use crate::prelude::*;
use crate::storage::traits::Filestore;
use expanduser::expanduser;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::{read_to_string, write, DirBuilder};

#[cfg_attr(feature = "yaml", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Yaml {
    path: String,
}

// impl<T> Yaml<T> {
//     pub fn new(path: impl Into<String>) -> Self {
//         Self {
//             path: path.into(),
//             _marker: std::marker::PhantomData,
//         }
//     }
// }

impl Filestore for Yaml {
    fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    fn load<T: DeserializeOwned>(&self) -> Result<T, FileSystemStorageError> {
        let path = expanduser(&self.path).map_err(|_| {
            FileSystemStorageError::FileSaveError(format!(
                "Failed to save repository: {}",
                self.path
            ))
        })?;
        match read_to_string(path) {
            Ok(data) => Ok(serde_yaml::from_str(&data)
                .map_err(|e| FileSystemStorageError::FileLoadError(e.to_string())))?,
            Err(e) => Err(FileSystemStorageError::FileLoadError(e.to_string())),
        }
    }

    fn save<T: Serialize>(&self, to_save: T) -> Result<(), FileSystemStorageError> {
        let path = expanduser(&self.path).map_err(|_| {
            FileSystemStorageError::FileSaveError(format!(
                "Failed to save repository: {}",
                self.path
            ))
        })?;

        let dir_path = path.parent().ok_or_else(|| {
            FileSystemStorageError::FileSaveError("Failed to get parent directory".to_string())
        })?;

        DirBuilder::new()
            .recursive(true)
            .create(dir_path.display().to_string())
            .map_err(|_| {
                FileSystemStorageError::FileSaveError(format!(
                    "Failed to create directory: {}",
                    dir_path.display()
                ))
            })?;
        write(
            path,
            serde_yaml::to_string(&to_save)
                .map_err(|e| FileSystemStorageError::FileSaveError(e.to_string()))?,
        )
        .map_err(|e| FileSystemStorageError::FileSaveError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::traits::Filestore;

    #[derive(Debug, Serialize, Deserialize)]
    struct Blah {
        pub name: String,
    }

    #[test]
    fn test_yaml_load() {
        let yaml = Yaml::new("~/.backlog/tests/projects.yaml");
        let loaded = yaml.load::<Blah>();
        println!("{:?}", loaded);
        assert!(loaded.is_ok());
    }

    #[test]
    fn test_yaml_save() {
        let to_save = Blah {
            name: "blah".to_string(),
        };
        let yaml = Yaml::new("~/.backlog/tests/projects.yaml");
        let saved = yaml.save(to_save);
        println!("{:?}", saved);
        assert!(saved.is_ok());
    }
}
