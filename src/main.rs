use clap::{Arg, Command};
use std::collections::HashMap;
use std::process;

static USAGE_MSG: &str =
    "Usage:\n\n  adml [options] -s <path-to-adml-project> -b <path-to-output-directory>\n\n";

#[derive(Debug, Default)]
struct RunConfig {
    path_to_project: String,
    path_to_output_dir: String,
}

#[derive(Debug)]
struct OptionParamInfo;

#[derive(Debug)]
enum ConfigMutator {
    WithParam(fn(&mut RunConfig, &str)),
}

#[derive(Debug)]
struct OptionInfo {
    mutator: ConfigMutator,
}

fn options() -> HashMap<char, OptionInfo> {
    let mut map = HashMap::new();

    fn addparam(
        map: &mut HashMap<char, OptionInfo>,
        _specifier: char,
        _param_name: &str,
        mutator: fn(&mut RunConfig, &str),
    ) {
        map.insert(
            _specifier,
            OptionInfo {
                mutator: ConfigMutator::WithParam(mutator),
            },
        );
    }

    addparam(&mut map, 's', "path-to-adml-project", |run_config, arg| {
        run_config.path_to_project = arg.to_string();
    });

    addparam(
        &mut map,
        'b',
        "path-to-output-directory",
        |run_config, arg| {
            run_config.path_to_output_dir = arg.to_string();
        },
    );

    map
}

fn main() {
    let option_infos = options();

    let matches = Command::new("adml")
        .version("1.0")
        .about("OwO Whats this?")
        .arg(
            Arg::new("s")
                .short('s')
                .required(true)
                .help("Path to ADML project")
                .value_name("PATH")
                .num_args(1),
        )
        .arg(
            Arg::new("b")
                .short('b')
                .required(true)
                .help("Path to output directory")
                .value_name("PATH")
                .num_args(1),
        )
        .get_matches();

    let mut run_config = RunConfig::default();
    for (specifier, option_info) in &option_infos {
        if let ConfigMutator::WithParam(m) = &option_info.mutator {
            let value = matches
                .get_one::<String>(&specifier.to_string())
                .expect("Required argument missing");
            m(&mut run_config, value);
        }
    }
    if run_config.path_to_project.is_empty() || run_config.path_to_output_dir.is_empty() {
        eprintln!("{}", USAGE_MSG);
        process::exit(1);
    }
    println!("RunConfig: {:?}", run_config);
}
