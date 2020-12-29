# rfd

[![version](https://img.shields.io/crates/v/rfd.svg)](https://crates.io/crates/rfd)
[![Documentation](https://docs.rs/rfd/badge.svg)](https://docs.rs/rfd)
[![dependency status](https://deps.rs/crate/rfd/0.0.2/status.svg)](https://deps.rs/crate/rfd/0.0.2)

WIP native file dialogs for Windows, Linux (GTK), MacOS.

# Example

```rust
let res = rfd::Dialog::pick_files()
    .add_filter("text", &["txt"])
    .add_filter("rust", &["rs", "toml"])
    .starting_directory(&"/home")
    .open();

let file = res.first();
```

# State

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/PolyMeilex/rfd/Rust/master?style=flat-square)

| API Stability  |
| -------------- |
| :construction: |

| Feature      | Linux              | Windows            | MacOS [1]          | Wasm32         |
| ------------ | ------------------ | ------------------ | ------------------ | -------------- |
| SingleFile   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :construction: |
| MultipleFile | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                |
| PickFolder   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                |
| SaveFile     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                |
|              |                    |                    |                    |                |
| Filters      | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| StartingPath | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                |
| Async        |                    |                    |                    |                |

[1] Freezes when used with winit (same way as `nfd`) [#1779](https://github.com/rust-windowing/winit/issues/1779)

# rfd-extras

AKA features that will be either in a separate `rfd-extras` crate, or behind a feature flag

| Feature       | Linux | Windows | MacOS |
| ------------- | ----- | ------- | ----- |
| MessageDialog |       |         |       |
| PromptDialog  |       |         |       |
| ColorPicker   |       |         |       |
