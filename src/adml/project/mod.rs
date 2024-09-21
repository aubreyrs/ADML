use std::{io::{self, ErrorKind}, vec::Vec};

mod config_json_obj;
use config_json_obj::ConfigJsonObj;

mod item_json_obj;
use item_json_obj::ItemJsonObj;
use serde::de::DeserializeOwned;

use std::{fs, path::{Path, PathBuf}};
use crate::adml::errors;
use errors::Result;

// all path constants declared here are relative to the project directory
const PROJECT_CONFIG_PATH: &str = "adml.json";
const ITEM_DIR_PATH: &str = "item/";
const SOURCE_DIR_PATH: &str = "src/";

// in-memory representation of the source project
pub struct Project {
    config_json_obj: ConfigJsonObj,
}

impl Project {

    // project_path is either absolute or relative to the working directory
    pub fn parse_project<T: AsRef<Path>>(project_dir_path: T) -> Result<Project> {
        let config_json_obj = Self::parse_project_config(&project_dir_path)?;
        let item_json_objs = Self::parse_item_dir(&project_dir_path)?;
        for item_json_obj in &item_json_objs {
            println!("{:?}", item_json_obj);
        }
        
        Ok(Self {
            config_json_obj,
        })
    }

    fn parse_project_config<T: AsRef<Path>>(project_dir_path: T) -> Result<ConfigJsonObj> {
        let config_path_full = make_path(&project_dir_path, PROJECT_CONFIG_PATH);
        read_json_file(&config_path_full)
    }

    fn parse_item_dir<T: AsRef<Path>>(project_dir_path: T) -> Result<Vec<ItemJsonObj>> {
        let item_dir_path_full = make_path(&project_dir_path, ITEM_DIR_PATH);
        
        get_paths_from_dir(&item_dir_path_full)?.iter()
            .map(|path| read_json_file(&path))
            .collect()
    }

    fn parse_src_dir<T: AsRef<Path>>(project_dir_path: T) {

    }

}

fn read_text_file<T: AsRef<Path>>(path: T) -> Result<String> {
    match fs::read_to_string(&path) {
        Ok(s) => Ok(s),
        Err(e) => make_io_error(path.as_ref().to_owned(), e),
    }
}

fn read_json_file<T: AsRef<Path>, U: DeserializeOwned>(path: T) -> Result<U> {
    let content = read_text_file(&path)?;
    match serde_json::from_str(&content) {
        Ok(obj) => Ok(obj),
        Err(e) => make_configuration_error(path.as_ref().to_owned(), e),
    }
}

fn get_paths_from_dir<T: AsRef<Path>>(path: T) -> Result<Vec<PathBuf>> {
    let mut paths = match fs::read_dir(&path) {
        Ok(read_dir) => {
            match read_dir
                .map(|result| result.map(|e| e.path()))
                .collect::<std::result::Result<Vec<_>, io::Error>>()
            {
                Ok(paths) => paths,
                Err(e) => return make_io_error(path.as_ref().to_owned(), e),
            }
        },
        Err(e) =>
            return if e.kind() == ErrorKind::NotFound { Ok(Vec::new()) }
            else { make_io_error(path.as_ref().to_owned(), e) },
    };
    paths.sort();
    Ok(paths)
}

fn make_path<T: AsRef<Path>, U: AsRef<Path>>(parent: T, child: U) -> PathBuf {
    let mut path = parent.as_ref().to_owned();
    path.push(child);
    path
}

fn make_io_error<T: Into<PathBuf>, U>(path: T, error: std::io::Error) -> Result<U> {
    Err(Box::new(errors::IO::new(path, error.to_string())))
}

fn make_configuration_error<T: Into<PathBuf>, U>(path: T, error: serde_json::Error) -> Result<U> {
    Err(Box::new(errors::Configuration::new(path, error.to_string())))
}