use std::path::{Path, PathBuf};

pub trait Error {
    fn name(&self) -> String;
    fn info(&self) -> String;
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct File {
    path: PathBuf,
}

impl File {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Error for File {
    fn name(&self) -> String {
        "FileError".to_string()
    }

    fn info(&self) -> String {
        "Failed to process ".to_string() +
        &self.path.clone()
            .into_os_string()
            .into_string()
            .unwrap_or("...".to_string())
    }
}

// errors in config files
pub struct Configuration {
    path: PathBuf,
    line: usize,
    column: usize,
}

impl Configuration {
    pub fn new<T: AsRef<Path>>(path: T, line: usize, column: usize) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            line,
            column,
        }
    }
}

impl Error for Configuration {
    fn name(&self) -> String {
        "ConfigurationError".to_string()
    }

    fn info(&self) -> String {
        format!(
            "Configuration error at ({}, {}) in file {}",
            &self.line, &self.column, &self.path.clone().into_os_string().into_string().unwrap_or("...".to_string())
        )
    }
}