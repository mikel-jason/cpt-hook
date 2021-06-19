use crate::actions::{check_if_file_exists, check_if_installed};
use std::path::PathBuf;

pub mod npm_scripts;

pub fn validate_node(repository_path: &PathBuf) -> bool {
    let mut toml_path = PathBuf::new();
    toml_path.clone_from(repository_path);
    toml_path.push("package.json");

    let checks = vec![
        check_if_installed("node", "-v"),
        check_if_file_exists(&toml_path),
    ];

    checks.iter().all(|&b| b)
}

pub fn get_package_manager() -> String {
    if check_if_installed("yarn", "-v") {
        String::from("yarn")
    } else {
        String::from("npm")
    }
}
