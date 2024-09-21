use std::path::PathBuf;

use colored::Colorize;

pub trait Error {
    fn name(&self) -> String;
    fn info(&self) -> String;
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct IO {
    path: PathBuf,
    msg: String,
}

// errors in all file related operations
impl IO {
    pub fn new<T: Into<PathBuf>, U: Into<String>>(path: T, msg: U) -> Self {
        Self {
            path: path.into(),
            msg: msg.into(),
        }
    }
}

impl Error for IO {
    fn name(&self) -> String {
        "FileError".to_string()
    }

    fn info(&self) -> String {
        format!(
            "Failed to process {}: {}",
            &self.path.clone().into_os_string().into_string().unwrap().underline(),
            &self.msg
        )
    }
}

// errors in config files
pub struct Configuration {
    path: PathBuf,
    msg: String,
}

impl Configuration {
    pub fn new<T: Into<PathBuf>, U: Into<String>>(path: T, msg: U) -> Self {
        Self {
            path: path.into(),
            msg: msg.into(),
        }
    }
}

impl Error for Configuration {
    fn name(&self) -> String {
        "ConfigurationError".to_string()
    }

    fn info(&self) -> String {
        format!(
            "Configuration error in {}: {}",
            &self.path.clone().into_os_string().into_string().unwrap().underline(),
            &self.msg
        )
    }
}