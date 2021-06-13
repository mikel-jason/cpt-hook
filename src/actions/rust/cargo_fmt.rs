use crate::actions::rust::validate_rust;
use crate::actions::{run, Action};
use crate::git_ops::Hook;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CargoFmt {}

impl Action for CargoFmt {
    fn validate(&self, repository_path: &PathBuf, _hook: &Hook) -> bool {
        validate_rust(&repository_path)
    }

    fn execute(&self, _repository_path: &PathBuf, _hook: &Hook) -> Result<(), &'static str> {
        run("cargo fmt -- --check")
    }
}

impl Display for CargoFmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rust: Check formatting with cargo fmt")
    }
}
