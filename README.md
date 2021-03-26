# rfd

[![version](https://img.shields.io/crates/v/rfd.svg)](https://crates.io/crates/rfd)
[![Documentation](https://docs.rs/rfd/badge.svg)](https://docs.rs/rfd)
[![dependency status](https://deps.rs/crate/rfd/0.2.2/status.svg)](https://deps.rs/crate/rfd/0.2.2)

Rusty file dialogs for Windows, Linux (GTK), MacOS And WASM32.

# Why RFD?

- It uses 100% native API on all platforms, it does not spawn any processes in the background.
- It supports async/await syntax
- And if one day you decide to port your program to browser, WASM support is there for you!

# Example

```rust
// Sync Dialog
let files = FileDialog::new()
    .add_filter("text", &["txt", "rs"])
    .add_filter("rust", &["rs", "toml"])
    .set_directory(&"/")
    .pick_files();

// Async Dialog
let file = AsyncFileDialog::new()
    .add_filter("text", &["txt", "rs"])
    .add_filter("rust", &["rs", "toml"])
    .set_directory(&"/")
    .pick_file()
    .await;

let data = file.read().await;
```

# State

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/PolyMeilex/rfd/Rust/master?style=flat-square)

| API Stability |
| ------------- |
| 🚧            |

| Feature      | Linux | Windows | MacOS [1] | Wasm32 |
| ------------ | ----- | ------- | --------- | ------ |
| SingleFile   | ✔     | ✔       | ✔         | ✔      |
| MultipleFile | ✔     | ✔       | ✔         | ✔      |
| PickFolder   | ✔     | ✔       | ✔         | ✖      |
| SaveFile     | ✔     | ✔       | ✔         | ✖      |
|              |       |         |           |        |
| Filters      | ✔     | ✔       | ✔         | ✔      |
| StartingPath | ✔     | ✔       | ✔         | ✖      |
| Async        | ✔     | ✔       | ✔         | ✔      |

[1] Macos Sync dialog freezes when used with winit (same way as `nfd`) [Caused by winit #1779](https://github.com/rust-windowing/winit/issues/1779)

### Diference bettwen `MacOS Windowed App` and `MacOS NonWindowed App`

- Macos async dialog requires an started `NSApplication` instance, so dialog is truly async only when opened in windowed env like `winit`,`SDL2`, etc. otherwise it will fallback to sync dialog.
- It is also recomended to spawn dialogs on main thread, RFD can run dialogs from any thread but it is only posible in windowed app and it adds a lite bit of overhead. So it is recomended to: [spawn on main and await in other thread](https://github.com/PolyMeilex/rfd/blob/master/examples/async.rs)
- NonWindowed apps will never be able to spawn dialogs from threads diferent than main
- NonWindowed apps will never be able to spawn async dialogs

# rfd-extras

AKA features that are not file related

| Feature       | Linux | Windows | MacOS | Wasm32 |
| ------------- | ----- | ------- | ----- | ------ |
| MessageDialog | ✔     | ✔       | ✔     | ✔      |
| PromptDialog  |       |         |       |        |
| ColorPicker   |       |         |       |        |
