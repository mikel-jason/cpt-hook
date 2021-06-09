use crate::actions::{check_if_file_exists, check_if_installed};
use std::path::PathBuf;

mod cargo_fmt;
mod cargo_test;

pub use cargo_fmt::CargoFmt;
pub use cargo_test::CargoTest;

pub fn validate_rust(repository_path: &PathBuf) -> bool {
    let mut toml_path = PathBuf::new();
    toml_path.clone_from(repository_path);
    toml_path.push("Cargo.toml");

    let checks = vec![
        check_if_installed("cargo", "-v"),
        check_if_file_exists(&toml_path),
    ];

    checks.iter().all(|&b| b)
}
