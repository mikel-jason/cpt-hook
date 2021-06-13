# Cpt. Hook &emsp; ![status] ![license] ![crate]

[status]: https://img.shields.io/badge/status-unstable-red
[license]: https://img.shields.io/crates/l/cpt-hook.svg
[crate]: https://img.shields.io/crates/v/cpt-hook.svg
[docs]: https://docs.rs/cpt-hook/badge.svg

**Out-of-the-box hooks for your Git repositories** 

---

`cpt-hook` is intended to manage your Git hooks on the fly and transparent to remote repositories. On every hook configured, `cpt-hook` is called and provides a multiple-choice selection of implemented hooks. Choose from the list or just skip all hooks and proceed with whatever you do. 

## Usage

Prerequisites: Rust & cargo

Install from source: Clone repository and run `cargo install --path <PATH_TO_CLONED_REPO>`

Set up a Git repo with `cpt-hook --repository <PATH_TO_TARGET_REPOSITORY> init` (short `-r`). Without `--repository`, the current directory is selected. The target could point to any directory within a Git workspace. Choose hooks in the selection with arrow keys and space, submit with return. To unset or update, re-run `init`.

## Implemented actions

- Rust
    - Format checker with `cargo fmt`
    - Run tests

## Contribution

Help is more than welcome! To provide a comprehensive collection of actions, competence and experience in different development environments is required. The maintainer(s) cannot and serve all environments you use.

If `cpt-hook` continues to grow and provide a good list of actions, other publishing channels (e.g. AUR) would be interesting to pursue. The maintainer(s) don't have reasonable knowledge in this field and are happy to get a helping hand.
