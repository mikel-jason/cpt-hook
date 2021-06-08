use std::fs;
use std::path::PathBuf;
use regex::Regex;

// TODO
// - better debugging
// - activating and deactivating adds \n to script -> remove


pub fn update_hook(git_dir_path: &PathBuf, hook: &str, activate: bool) -> std::io::Result<()> {

    #[cfg(debug_assertions)]
    println!("Updating hook {:?}, active: {:?}", hook, activate);

    let mut hook_path = git_dir_path.clone();
    hook_path.push("hooks");
    hook_path.push(hook);

    let mut hook_contents = match hook_path.exists() {
        true => { fs::read_to_string(&hook_path).unwrap() }
        false => { String::from("#!/bin/sh\n\n") }
    };

    hook_contents = prune_hook(hook_contents);

    if activate {
        inject(&mut hook_contents, hook);
    }

    #[cfg(debug_assertions)]
    print!("Wanna write for {:?}: {:?}", &hook, &hook_contents);

    std::fs::write(hook_path, hook_contents)
}

fn prune_hook(content: String) -> String {
    let regex = Regex::new(r".*cpt-hook run.*[[\r]?\n]?(?m)").expect("Cannot create regex");
    String::from(regex.replace_all(&*content, "").into_owned())
}

fn inject(content: &mut String, hook: &str) {
    content.push_str("\n");
    content.push_str(r#"echo "cpt-hook run --hook ""#);
    content.push_str(hook);
    content.push_str(r#"""#);
}
