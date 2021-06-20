use crate::actions::rust::validate_rust;
use crate::actions::{run, Action};
use crate::git_ops::Hook;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CargoPublish {}

impl Action for CargoPublish {
    fn validate(&self, repository_path: &PathBuf, hook: &Hook) -> bool {

        let checks = vec![
            validate_rust(&repository_path),
            hook == &Hook::PostCommit
        ];

        checks.iter().all(|&b| b)
    }

    fn execute(&self, _repository_path: &PathBuf, _hook: &Hook) -> Result<(), &'static str> {
        run("cargo publish --verbose")
    }
}

impl Display for CargoPublish {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rust: Publish on crates.io (default params)")
    }
}
