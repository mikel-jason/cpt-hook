use crate::actions::{Action, run};
use crate::git_ops::Hook;
use std::path::PathBuf;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader};

use serde_json::Value;
use serde_json::value::Value::Object;
use crate::actions::node::validate_node;

#[derive(Debug)]
pub struct NodeScript {
    cmd: String,
}

impl Action for NodeScript {
    fn validate(&self, repository_path: &PathBuf, _hook: &Hook) -> bool {
        validate_node(&repository_path)
    }

    fn execute(&self, _repository_path: &PathBuf, _hook: &Hook) -> Result<(), &'static str> {
        run(&*format!("npm run {}", self.cmd))
    }
}

impl Display for NodeScript {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Node script: {}", self.cmd)
    }
}

pub fn get_node_scripts(repository_path: &PathBuf) -> Vec<Box<dyn Action>> {
    let mut scripts: Vec<Box<dyn Action>> = Vec::new();
    let mut package_json_path = repository_path.clone();
    package_json_path.push("package.json");
    let file: File = match File::open(package_json_path) {
        Ok(file) => file,
        Err(_) => return scripts
    };
    let reader: BufReader<File> = BufReader::new(file);

    let json_contents: Value = serde_json::from_reader(reader).expect("Cannot open package.json");

    if let Object(scripts_map) = &json_contents["scripts"] {
        scripts_map
            .keys()
            .for_each(|key| {
                if !(key.ends_with("before") || key.ends_with("after")) {
                    let cmd = String::from(key.as_str());
                    scripts.push(
                        Box::new(NodeScript { cmd })
                    )
                }
            })
    }

    scripts
}

