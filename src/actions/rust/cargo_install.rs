use crate::actions::rust::validate_rust;
use crate::actions::{run, Action};
use crate::git_ops::Hook;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CargoInstall {}

impl Action for CargoInstall {
    fn validate(&self, repository_path: &PathBuf, hook: &Hook) -> bool {

        let checks = vec![
            validate_rust(&repository_path),
            (vec![Hook::PostUpdate, Hook::PrePush, Hook::PostCommit]).contains(hook)
        ];
        checks.iter().all(|&b| b)
    }

    fn execute(&self, repository_path: &PathBuf, _hook: &Hook) -> Result<(), &'static str> {
        run(format!("cargo install --path {}", repository_path.to_str().expect("Cannot set path")).as_str())
    }
}

impl Display for CargoInstall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rust: Install to local machine (default params)")
    }
}
