use std::{env};
use std::path::Path;
use std::process::exit;

use clap::{Arg, App, SubCommand};
use dialoguer::{MultiSelect, theme::ColorfulTheme};


use crate::git_ops::*;

mod git_ops;

fn main() {
    let clap = App::new("cpt-hook")
        .version("dev")
        .author("sarcaustech")
        .about("Interactive management of hooks in your Git repositories")
        .arg(Arg::with_name("repository")
            .short("r")
            .long("repository")
            .help("Target repository. Default ist current directory.")
            .takes_value(true)
            .multiple(false))
        .subcommand(SubCommand::with_name("init")
            .about("Sets up the current Git repository for using cpt-hook"))
        .subcommand(SubCommand::with_name("run")
            .about("Runs hook handler")
            .arg(Arg::with_name("hook")
                .long("hook")
                .help("Specifies the incoming hook")
                .takes_value(true)
                .multiple(false)
                .required(true)))
        .get_matches();

    let repository_path = match clap.value_of("repository") {
        Some(path_str) => Path::new(path_str).to_path_buf(),
        None => {
            env::current_dir().unwrap()
        }
    };

    if !repository_path.exists() {
        eprintln!("Specified directory does not exist");
        exit(1);
    }

    let mut git_path = repository_path.clone();
    git_path.push(".git");

    if !git_path.exists() {
        eprintln!("Specified directory is no git repository. To use cpt-hook here, initialize the directory as Git repository and run `cpt-hook init`");
        exit(1);
    }

    if let Some(_) = clap.subcommand_matches("init") {
        #[cfg(debug_assertions)]
        println!("Initializing hooks!");

        let hooks_available = vec![
            "pre-commit",
            "pre-push"
        ];

        let hooks_selected = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose hooks")
            .items(&hooks_available)
            .interact().unwrap();

        let hooks_to_set: Vec<&str> = hooks_selected
            .iter()
            .map(|&i| hooks_available[i])
            .collect();

        #[cfg(debug_assertions)]
        println!("{:?}", hooks_to_set);

        for hook in hooks_available {
            if update_hook(&git_path, &hook, hooks_to_set.contains(&hook)).is_err() {
                eprintln!("Cannot update hook script for {:?}", hook);
            }
        }

        println!("Setting up hooks finished");
    }
    if let Some(run_cmd) = clap.subcommand_matches("run") {
        if let Some(hook) = run_cmd.value_of("hook") {
            println!("Running hook: {:?}", hook);
        } else {
            eprintln!("No hook specified");
            exit(1);
        }
    }
}
