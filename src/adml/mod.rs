pub mod errors;
pub mod file_utils;
mod project;
mod abstract_datapack;
use project::Project;
use abstract_datapack::AbstractDatapack;

use std::path::PathBuf;
use colored::Colorize;

use errors::Result;

#[derive(Debug, Default)]
pub struct BuildOptions {
    pub path_to_project: PathBuf,
    pub path_to_output_dir: PathBuf,
}

pub fn run_build(build_options: &BuildOptions) -> Result<()> {
    println!("{}", "Build starting...".green());

    // project path rel to working dir
    let project_dir_path = &build_options.path_to_project;
    
    let proj = Project::parse_project(project_dir_path)?;

    Ok(())
}