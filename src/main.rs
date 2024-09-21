use std::{collections::HashMap, fs, process::ExitCode};
use colored::Colorize;

mod adml;
use adml::{BuildOptions, run_build};

fn print_usage() {
    eprintln!("Usage:\n\n  adml [options] -s <path-to-adml-project> -b <path-to-output-directory>");
}

fn exit_on_improper_usage() -> ExitCode {
    print_usage();
    ExitCode::from(1)
}

trait ArgError {
    fn name(&self) -> String;
    fn info(&self) -> String;
}

fn print_arg_error(e: &dyn ArgError, arg: &str) {
    eprintln!("\"{}\" <--- {}: {}", arg, e.name(), e.info());
}

struct PathNotExist;

impl ArgError for PathNotExist {
    fn name(&self) -> String {
        "PathNotExist".to_string()
    }

    fn info(&self) -> String {
        "No such path exists".to_string()
    }
}

type ConfigMutatorFunc = fn(&mut BuildOptions) -> Result<(), Box<dyn ArgError>>;
type ConfigMutatorWithParamFunc = fn(&mut BuildOptions, &str) -> Result<(), Box<dyn ArgError>>;

struct OptionParamInfo {
    display_name: String
}

enum ConfigMutatorWithPotentialParam {
    NoParam(ConfigMutatorFunc),
    Param(ConfigMutatorWithParamFunc, OptionParamInfo, bool),
}

struct OptionInfo {
    specifier: char,
    mutator_with_potential_param: ConfigMutatorWithPotentialParam,
}

fn create_option_infos() -> HashMap<char, OptionInfo> {
    let mut map = HashMap::new();

    let mut add = |
        specifier: char,
        mutator: ConfigMutatorFunc
    | {
        map.insert(
            specifier,
            OptionInfo {
                specifier,
                mutator_with_potential_param: ConfigMutatorWithPotentialParam::NoParam(
                    mutator
                ),
            }
        )
    };

    let mut addparam = |
        specifier: char,
        param_name: &str,
        required: bool,
        mutator: ConfigMutatorWithParamFunc,
    | {
        map.insert( 
            specifier,
            OptionInfo {
                specifier,
                mutator_with_potential_param: ConfigMutatorWithPotentialParam::Param(
                    mutator,
                    OptionParamInfo {
                        display_name: String::from(param_name)
                    },
                    required,
                )
            },
        );
    };

    addparam(
        's',
        "path-to-adml-project",
        true,
        |build_config, arg| {
            build_config.path_to_project = match fs::canonicalize(arg) {
                Ok(path) => path,
                Err(_) => return Err(Box::new(PathNotExist)),
            };
            Ok(())
        }
    );

    addparam(
        'b',
        "path-to-output-directory",
        true,
        |build_config, arg| {
            build_config.path_to_output_dir = match fs::canonicalize(arg) {
                Ok(path) => path,
                Err(_) => return Err(Box::new(PathNotExist)),
            };
            Ok(())
        },
    );

    map
}

fn main() -> ExitCode {
    let cmd_line_args: Vec<String> = std::env::args().collect();

    let option_infos = create_option_infos();

    let mut required_options: Vec<char> = Vec::new();
    for (specifier, option_info) in &option_infos {
        if let ConfigMutatorWithPotentialParam::Param(_, _, required) = option_info.mutator_with_potential_param {
            if required {
                required_options.push(*specifier);
            }
        }
    }
    let required_options = required_options;

    let mut build_options = BuildOptions::default();

    let mut parsing_param: Option<&ConfigMutatorWithParamFunc> = None;

    let mut parsed_options: Vec<char> = Vec::new();

    for token in cmd_line_args.iter().skip(1) {
        match &parsing_param {
            None => {
                if !token.starts_with('-') {
                    return exit_on_improper_usage();
                }
                if token.chars().count() != 2 {
                    return exit_on_improper_usage();
                }
                let option_specifier = token.chars().nth(1).unwrap();
                if parsed_options.contains(&option_specifier) {
                    return exit_on_improper_usage();
                }
                if !option_infos.contains_key(&option_specifier) {
                    return exit_on_improper_usage();
                }
                let option_info = option_infos.get(&option_specifier).unwrap();
                match &option_info.mutator_with_potential_param {
                    ConfigMutatorWithPotentialParam::NoParam(mutator) => {
                        if let Err(e) = mutator(&mut build_options) {
                            print_arg_error(&*e, &token);
                            return 1.into();
                        }
                    },
                    ConfigMutatorWithPotentialParam::Param(mutator, _, _) =>{
                        parsing_param = Some(&mutator)
                    },
                }
                parsed_options.push(option_specifier);
            },
            Some(mutator) => {
                if token.starts_with('-') {
                    return exit_on_improper_usage();
                }
                if let Err(e) = mutator(&mut build_options, &token) {
                    print_arg_error(&*e, &token);
                    return 1.into();
                }
                parsing_param = None;
            },
        }
    }
    if parsing_param.is_some() {
        return exit_on_improper_usage();
    }
    for specifier in &required_options {
        if !parsed_options.contains(specifier) {
            return exit_on_improper_usage();
        }
    }

    let result = run_build(&build_options);
    if let Err(e) = result {
        eprintln!("{}\n  {}\n", e.name().red().bold(), e.info().yellow());
    }

    0.into()
}
