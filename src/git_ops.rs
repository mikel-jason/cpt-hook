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
    PreApplyPatch,
    PostApplyPatch,
    PreCommit,
    PreMergeCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    PreRebase,
    PostCheckout,
    PostMerge,
    PrePush,
    PreReceive,
    Update,
    ProcReceive,
    PostReceive,
    PostUpdate,
    ReferenceTransaction,
    PushToCheckout,
    PreAutoGc,
    PostRewrite,
    SendEmailValidate,
    FsMonitorWatchman,
    FsMonitorWatchmanV2,
    P4Changelist,
    P4PrepareChangelist,
    P4PostChangelist,
    P4PreSubmit,
    PostIndexChange,
}

impl Hook {
    pub fn from(hook: &str) -> Result<Self> {
        match hook {
            "applypatch-msg" => Ok(Hook::ApplyPatchMsg),
            "pre-applypatch" => Ok(Hook::PreApplyPatch),
            "post-applypatch" => Ok(Hook::PostApplyPatch),
            "pre-commit" => Ok(Hook::PreCommit),
            "pre-merge-commit" => Ok(Hook::PreMergeCommit),
            "prepare-commit-msg" => Ok(Hook::PrepareCommitMsg),
            "commit-msg" => Ok(Hook::CommitMsg),
            "post-commit" => Ok(Hook::PostCommit),
            "pre-rebase" => Ok(Hook::PreRebase),
            "post-checkout" => Ok(Hook::PostCheckout),
            "post-merge" => Ok(Hook::PostMerge),
            "pre-push" => Ok(Hook::PrePush),
            "pre-receive" => Ok(Hook::PreReceive),
            "update" => Ok(Hook::Update),
            "proc-receive" => Ok(Hook::ProcReceive),
            "post-receive" => Ok(Hook::PostReceive),
            "post-update" => Ok(Hook::PostUpdate),
            "reference-transaction" => Ok(Hook::ReferenceTransaction),
            "push-to-checkout" => Ok(Hook::PushToCheckout),
            "pre-auto-gc" => Ok(Hook::PreAutoGc),
            "post-rewrite" => Ok(Hook::PostRewrite),
            "sendemail-validate" => Ok(Hook::SendEmailValidate),
            "fsmonitor-watchman" => Ok(Hook::FsMonitorWatchman),
            "fsmonitor-watchmanv2" => Ok(Hook::FsMonitorWatchmanV2),
            "p4-changelist" => Ok(Hook::P4Changelist),
            "p4-prepare-changelist" => Ok(Hook::P4PrepareChangelist),
            "p4-post-changelist" => Ok(Hook::P4PostChangelist),
            "p4-pre-submit" => Ok(Hook::P4PreSubmit),
            "post-index-change" => Ok(Hook::PostIndexChange),
            _ => Err(Box::from(format!("Unknown git hook: {}", hook)))
        }
    }

    pub fn all() -> Vec<Hook> {
        vec![
            Hook::ApplyPatchMsg,
            Hook::PreApplyPatch,
            Hook::PostApplyPatch,
            Hook::PreCommit,
            Hook::PreMergeCommit,
            Hook::PrepareCommitMsg,
            Hook::CommitMsg,
            Hook::PostCommit,
            Hook::PreRebase,
            Hook::PostCheckout,
            Hook::PostMerge,
            Hook::PrePush,
            Hook::PreReceive,
            Hook::Update,
            Hook::ProcReceive,
            Hook::PostReceive,
            Hook::PostUpdate,
            Hook::ReferenceTransaction,
            Hook::PushToCheckout,
            Hook::PreAutoGc,
            Hook::PostRewrite,
            Hook::SendEmailValidate,
            Hook::FsMonitorWatchman,
            Hook::FsMonitorWatchmanV2,
            Hook::P4Changelist,
            Hook::P4PrepareChangelist,
            Hook::P4PostChangelist,
            Hook::P4PreSubmit,
            Hook::PostIndexChange,
        ]
    }
}

impl ToString for Hook {
    fn to_string(&self) -> String {
        match self {
            // Find:        "(.*)" => Ok\(Hook::(.*)\),
            // Replace:     Hook::$2 => String::from("$1"),
            Hook::ApplyPatchMsg => String::from("applypatch-msg"),
            Hook::PreApplyPatch => String::from("pre-applypatch"),
            Hook::PostApplyPatch => String::from("post-applypatch"),
            Hook::PreCommit => String::from("pre-commit"),
            Hook::PreMergeCommit => String::from("pre-merge-commit"),
            Hook::PrepareCommitMsg => String::from("prepare-commit-msg"),
            Hook::CommitMsg => String::from("commit-msg"),
            Hook::PostCommit => String::from("post-commit"),
            Hook::PreRebase => String::from("pre-rebase"),
            Hook::PostCheckout => String::from("post-checkout"),
            Hook::PostMerge => String::from("post-merge"),
            Hook::PrePush => String::from("pre-push"),
            Hook::PreReceive => String::from("pre-receive"),
            Hook::Update => String::from("update"),
            Hook::ProcReceive => String::from("proc-receive"),
            Hook::PostReceive => String::from("post-receive"),
            Hook::PostUpdate => String::from("post-update"),
            Hook::ReferenceTransaction => String::from("reference-transaction"),
            Hook::PushToCheckout => String::from("push-to-checkout"),
            Hook::PreAutoGc => String::from("pre-auto-gc"),
            Hook::PostRewrite => String::from("post-rewrite"),
            Hook::SendEmailValidate => String::from("sendemail-validate"),
            Hook::FsMonitorWatchman => String::from("fsmonitor-watchman"),
            Hook::FsMonitorWatchmanV2 => String::from("fsmonitor-watchmanv2"),
            Hook::P4Changelist => String::from("p4-changelist"),
            Hook::P4PrepareChangelist => String::from("p4-prepare-changelist"),
            Hook::P4PostChangelist => String::from("p4-post-changelist"),
            Hook::P4PreSubmit => String::from("p4-pre-submit"),
            Hook::PostIndexChange => String::from("post-index-change"),
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
