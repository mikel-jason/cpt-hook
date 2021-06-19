use execute::Execute;
use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::process::Command;

use crate::git_ops::Hook;

pub mod node;
pub mod rust;

pub trait Action: Debug + Display {
    fn validate(&self, repository_path: &PathBuf, hook: &Hook) -> bool;
    fn execute(&self, repository_path: &PathBuf, hook: &Hook) -> Result<(), &'static str>;
}

pub fn check_if_installed(command: &str, argument: &str) -> bool {
    let mut cmd = Command::new(command);
    cmd.arg(argument);

    match cmd.execute_check_exit_status_code(0) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn check_if_file_exists(path: &PathBuf) -> bool {
    path.exists()
}

pub fn run(cmd: &str) -> Result<(), &'static str> {
    match Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status()
        .expect(format!("Cannot run '{:?}'", &cmd).as_str())
        .success()
    {
        true => Ok(()),
        false => {
            Err("Error when running an action") // TODO: provide cmd in error msg
        }
    }
}
