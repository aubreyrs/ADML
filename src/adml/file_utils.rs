use std::{env, path::PathBuf};

pub struct PathManager {
    working_dir_abs: PathBuf,
    project_dir_rel_working_dir: PathBuf,
}

impl PathManager {
    pub fn new<T: Into<PathBuf>>(project_dir_rel_working_dir: T) -> Self {
        Self {
            working_dir_abs: env::current_dir().unwrap(),
            project_dir_rel_working_dir: project_dir_rel_working_dir.into(),
        }
    }
    pub fn path_in_project<T: Into<PathBuf>>(&self, path_rel_project_dir: T) -> Path {
        let path = path_rel_project_dir.into();
        Path {
            rel_to_working_dir: self.working_dir_abs.join(&path),
            rel_to_project_root: path
        }
    }
}

pub struct Path {
    rel_to_working_dir: PathBuf,
    rel_to_project_root: PathBuf,
}

impl Path {
    
}