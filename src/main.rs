use std::env;
use std::path::Path;
use std::process::exit;

use clap::{App, Arg, SubCommand};
use dialoguer::{theme::ColorfulTheme, MultiSelect};

mod actions;
mod git_ops;

use actions::rust::{CargoFmt, CargoTest, CargoInstall, CargoPublish};
use actions::Action;
use colored::Colorize;
use git_ops::*;

fn main() {
    let clap = App::new("cpt-hook")
        .version("dev")
        .author("sarcaustech")
        .about("Interactive management of hooks in your Git repositories")
        .arg(
            Arg::with_name("repository")
                .short("r")
                .long("repository")
                .help("Target repository. Default ist current directory.")
                .takes_value(true)
                .multiple(false),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Sets up the current Git repository for using cpt-hook")
                .arg(Arg::with_name("all-hooks")
                    .long("all-hooks")
                    .short("a")
                    .help("Select from all available hooks (see git docs)")
                ),
        )
        .subcommand(
            SubCommand::with_name("run").about("Runs hook handler").arg(
                Arg::with_name("hook")
                    .long("hook")
                    .help("Specifies the incoming hook")
                    .takes_value(true)
                    .multiple(false)
                    .required(true),
            ),
        )
        .get_matches();

    let input_path = match clap.value_of("repository") {
        Some(path_str) => Path::new(path_str).to_path_buf(),
        None => env::current_dir().unwrap(),
    };

    let repository_path = match get_git_root(&input_path) {
        Ok(path) => path,
        Err(_) => {
            eprintln!(
                "{} {}\n{}",
                "cpt-hook was not launched in a git workdir root:"
                    .bright_red()
                    .bold(),
                &input_path.to_str().unwrap().bright_magenta().italic(),
                "Does the directory exist? Is it part of a git repository? Also try --hook option"
                    .bright_red()
                    .bold()
            );
            exit(1);
        }
    };

    if let Some(clap_init) = clap.subcommand_matches("init") {
        #[cfg(debug_assertions)]
        println!("{}", "Initializing hooks!");

        let hooks_available: Vec<String> = match clap_init.is_present("all-hooks"){
            true => Hook::all(),
            false => Hook::simple()
        }.iter().map(|h| h.to_string()).collect();

        let hooks_selected = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose hooks")
            .items(&hooks_available)
            .interact()
            .unwrap();

        let hooks_clone = hooks_available.clone(); // TODO remove clone
        let hooks_to_set: Vec<&str> = hooks_selected
            .iter()
            .map(|&i| hooks_clone[i].as_str())
            .collect();

        #[cfg(debug_assertions)]
        println!("{:?}", hooks_to_set);

        for hook in hooks_available {
            if update_hook(
                &repository_path,
                &Hook::from(&hook).expect("Unknown hook"),
                hooks_to_set.contains(&hook.as_str()),
            )
            .is_err()
            {
                eprintln!(
                    "{} {}",
                    "Cannot update hook script for".bright_red().bold(),
                    hook.bright_red().bold()
                );
            }
        }

        println!("{}", "Setting up hooks finished".bright_green().bold());
    }
    if let Some(run_cmd) = clap.subcommand_matches("run") {
        let mut actions_available: Vec<&dyn Action> = Vec::new();
        let cargo_fmt = CargoFmt {};
        let cargo_test = CargoTest {};
        let cargo_install = CargoInstall {};
        let cargo_publish = CargoPublish {};
        actions_available.push(&cargo_fmt);
        actions_available.push(&cargo_test);
        actions_available.push(&cargo_install);
        actions_available.push(&cargo_publish);

        if let Some(hook) = run_cmd.value_of("hook") {
            #[cfg(debug_assertions)]
            println!("Running hook: {:?}", hook);

            let the_hook = match Hook::from(hook) {
                Ok(hook) => hook,
                Err(_) => {
                    eprintln!("{} {}", "Unknown hook:".bright_red(), hook.red());
                    exit(1)
                }
            };

            let action_applicable: Vec<&dyn Action> = actions_available
                .into_iter()
                .filter(|&a| a.validate(&repository_path, &the_hook))
                .collect();

            if action_applicable.len() == 0 {
                println!(
                    "{}",
                    "cpt-hook cannot find any applicable actions"
                        .bright_red()
                        .bold()
                );
                exit(0)
            }

            let actions_selected: Vec<&dyn Action> =
                MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose actions")
                    .items(&action_applicable)
                    .interact()
                    .unwrap()
                    .iter()
                    .map(|&i| action_applicable[i])
                    .collect();

            if actions_selected
                .into_iter()
                .map(|action| action.execute(&repository_path, &the_hook))
                .any(|e| e.is_err())
            {
                eprintln!(
                    "{}",
                    "At least one of the selected checkers failed"
                        .bright_red()
                        .bold()
                );
                exit(1)
            }
        } else {
            eprintln!("{}", "No hook specified".bright_red().bold());
            exit(1);
        }
    }
}
