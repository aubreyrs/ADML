use std::{collections::HashMap, process::ExitCode};

fn print_usage() {
    print!("Usage:\n\n  adml [options] -s <path-to-adml-project> -b <path-to-output-directory>\n\n");
}

fn exit_on_improper_usage() -> ExitCode {
    print_usage();
    ExitCode::from(1)
}

#[derive(Debug, Default)]
struct RunOptions {
    path_to_project: String,
    path_to_output_dir: String,
}

type ConfigMutatorFunc = fn(&mut RunOptions);
type ConfigMutatorWithParamFunc = fn(&mut RunOptions, &str);

struct OptionParamInfo {
    display_name: String
}

enum ConfigMutatorWithPotentialParam {
    NoParam(ConfigMutatorFunc),
    Param(ConfigMutatorWithParamFunc, OptionParamInfo),
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
                    }
                )
            },
        );
    };

    addparam(
        's',
        "path-to-adml-project",
        |run_config, arg| {
            run_config.path_to_project = arg.to_string();
        }
    );

    addparam(
        'b',
        "path-to-output-directory",
        |run_config, arg| {
            run_config.path_to_output_dir = arg.to_string();
        },
    );

    map
}

fn main() -> ExitCode {
    let cmd_line_args: Vec<String> = std::env::args().collect();

    let option_infos = create_option_infos();

    let mut run_options = RunOptions::default();

    let mut parsing_param: Option<&ConfigMutatorWithParamFunc> = None;

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
                if !option_infos.contains_key(&option_specifier) {
                    return exit_on_improper_usage();
                }
                let option_info = option_infos.get(&option_specifier).unwrap();
                match &option_info.mutator_with_potential_param {
                    ConfigMutatorWithPotentialParam::NoParam(mutator) => mutator(&mut run_options),
                    ConfigMutatorWithPotentialParam::Param(mutator, _param_info) => parsing_param = Some(&mutator),
                }
            },
            Some(mutator) => {
                mutator(&mut run_options, &token);
                parsing_param = None;
            },
        }
    }

    if parsing_param.is_some() {
        return exit_on_improper_usage();
    }

    println!("{:?}", run_options);

    // if run_options.path_to_project.is_empty() || run_options.path_to_output_dir.is_empty() {
    //     return exit_on_improper_usage();
    // }

    ExitCode::from(0)
}
