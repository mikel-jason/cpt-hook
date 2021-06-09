use crate::actions::rust::validate_rust;
use crate::actions::{run, Action};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CargoTest {}

impl Action for CargoTest {
    fn validate(&self, repository_path: &PathBuf, _hook: &str) -> bool {
        validate_rust(&repository_path)
    }

    fn execute(&self, _repository_path: &PathBuf, _hook: &str) -> Result<(), &'static str> {
        run("cargo test")
    }
}

impl Display for CargoTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rust: Run tests")
    }
}
