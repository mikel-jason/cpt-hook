use git2::Repository;
use regex::Regex;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

// TODO
// - better debugging
// - activating and deactivating adds \n to script -> remove

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Hook {
    ApplyPatchMsg,
    CommitMsg,
    FsMonitorWatchman,
    PostUpdate,
    PreApplyPatch,
    PreCommit,
    PreMergeCommit,
    PrepareCommitMsg,
    PrePush,
    PreRebase,
    PreReceive,
    PushToCheckout,
    Update,
}

impl Hook {
    pub fn from(hook: &str) -> Result<Self> {
        match hook {
            "applypatch-msg" => Ok(Hook::ApplyPatchMsg),
            "commit-msg" => Ok(Hook::CommitMsg),
            "fsmonitor-watchman" => Ok(Hook::FsMonitorWatchman),
            "post-update" => Ok(Hook::PostUpdate),
            "pre-applypatch" => Ok(Hook::PreApplyPatch),
            "pre-commit" => Ok(Hook::PreCommit),
            "pre-merge-commit" => Ok(Hook::PreMergeCommit),
            "prepare-commit-msg" => Ok(Hook::PrepareCommitMsg),
            "pre-push" => Ok(Hook::PrePush),
            "pre-rebase" => Ok(Hook::PreRebase),
            "pre-receive" => Ok(Hook::PreReceive),
            "push-to-checkout" => Ok(Hook::PushToCheckout),
            "update" => Ok(Hook::Update),
            _ => Err(Box::from(format!("Unknown git hook: {}", hook))),
        }
    }

    pub fn all() -> Vec<Hook> {
        vec![
            Hook::ApplyPatchMsg,
            Hook::CommitMsg,
            Hook::FsMonitorWatchman,
            Hook::PostUpdate,
            Hook::PreApplyPatch,
            Hook::PreCommit,
            Hook::PreMergeCommit,
            Hook::PrepareCommitMsg,
            Hook::PrePush,
            Hook::PreRebase,
            Hook::PreReceive,
            Hook::PushToCheckout,
            Hook::Update,
        ]
    }
}

impl ToString for Hook {
    fn to_string(&self) -> String {
        match self {
            Hook::ApplyPatchMsg => String::from("applypatch-msg"),
            Hook::CommitMsg => String::from("commit-msg"),
            Hook::FsMonitorWatchman => String::from("fsmonitor-watchman"),
            Hook::PostUpdate => String::from("post-update"),
            Hook::PreApplyPatch => String::from("pre-applypatch"),
            Hook::PreCommit => String::from("pre-commit"),
            Hook::PreMergeCommit => String::from("pre-merge-commit"),
            Hook::PrepareCommitMsg => String::from("prepare-commit-msg"),
            Hook::PrePush => String::from("pre-push"),
            Hook::PreRebase => String::from("pre-rebase"),
            Hook::PreReceive => String::from("pre-receive"),
            Hook::PushToCheckout => String::from("push-to-checkout"),
            Hook::Update => String::from("update"),
        }
    }
}

pub fn get_git_root(path: &PathBuf) -> Result<PathBuf> {
    let repo = Repository::discover(path)?;
    match repo.workdir() {
        None => Err(Box::from("Repository has no workdir")),
        Some(path) => Ok(path.to_path_buf()),
    }
}

pub fn update_hook(repository_path: &PathBuf, hook: &Hook, activate: bool) -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    println!("Updating hook {:?}, active: {:?}", hook, activate);

    let mut hook_path = repository_path.clone();
    hook_path.push(".git");
    hook_path.push("hooks");
    hook_path.push(hook.to_string().as_str());

    let mut hook_contents = match hook_path.exists() {
        true => fs::read_to_string(&hook_path).unwrap(),
        false => String::from("#!/bin/sh\n\n"),
    };

    hook_contents = prune_hook(hook_contents);

    if activate {
        inject(&mut hook_contents, hook.to_string().as_str());
    }

    #[cfg(debug_assertions)]
    println!("Wanna write for {:?}: {:?}", &hook, &hook_contents);

    fs::write(&hook_path, hook_contents)?;
    fs::set_permissions(&hook_path, fs::Permissions::from_mode(0o755))
}

fn prune_hook(content: String) -> String {
    let regex = Regex::new(r".*cpt-hook run.*[[\r]?\n]?(?m)").expect("Cannot create regex");
    String::from(regex.replace_all(&*content, "").into_owned())
}

fn inject(content: &mut String, hook: &str) {
    content.push_str("\n");
    content.push_str(r#"cpt-hook run --hook ""#);
    content.push_str(hook);
    content.push_str(r#"""#);
}
