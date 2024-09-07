pub mod errors;
use std::{fs, path::PathBuf};
use colored::Colorize;


use errors::Result;

#[derive(Debug, Default)]
pub struct BuildOptions {
    pub path_to_project: PathBuf,
    pub path_to_output_dir: PathBuf,
}

pub fn build(build_options: &BuildOptions) -> Result<()> {
    println!("{}", "Build starting...".green());

    let project_path = &build_options.path_to_project;
    let project_config_path = project_path.join("adml.json");
    let project_config_content = match fs::read_to_string(&project_config_path) {
        Ok(s) => s,
        Err(e) => {
            return Err(Box::new(errors::File::new(&project_config_path)));
        },
    };

    

    Ok(())
}