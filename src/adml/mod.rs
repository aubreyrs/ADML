pub mod errors;
use std::{fs, path::PathBuf};
use colored::Colorize;
use serde::{Serialize, Deserialize};

use errors::Result;

#[derive(Debug, Default)]
pub struct BuildOptions {
    pub path_to_project: PathBuf,
    pub path_to_output_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct ConfigJsonObj {
    project_name: String,
}

pub fn build(build_options: &BuildOptions) -> Result<()> {
    println!("{}", "Build starting...".green());

    let project_path = &build_options.path_to_project;
    let _ = std::env::set_current_dir(project_path);
    
    let config_path = PathBuf::from("adml.json");

    let config_content = match fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(e) => {
            return Err(Box::new(errors::File::new(&config_path)));
        },
    };

    let config_json_obj: ConfigJsonObj = match serde_json::from_str(&config_content) {
        Ok(obj) => obj,
        Err(e) => {
            return Err(Box::new(errors::Configuration::new(&config_path, e.line(), e.column())));
        },
    };

    Ok(())
}