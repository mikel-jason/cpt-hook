# Cpt. Hook &emsp; ![status]

[status]: https://img.shields.io/badge/status-unstable-red

**Out-of-the-box hooks for your Git repositories**

---

`cpt-hook` is intended to manage your Git hooks on the fly and transparent to remote repositories. On every hook configured, `cpt-hook` is called and provides a multiple-choice selection of implemented hooks. Choose from the list or just skip all hooks and proceed with whatever you do.

This project is still under development. Please refer to [the GitHub repo](https://github.com/sarcaustech/cpt-hook) for the source code more info and to contribute.

## Implemented actions

- Rust
    - Format checker with `cargo fmt`
    - Run tests

