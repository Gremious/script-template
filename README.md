# script-template
Simple template repository for creating projects you can both build as a standalone binary, or, run with [rust-script](https://rust-script.org/#installation) or [cargo-play](https://crates.io/crates/cargo-play).

Note that if you want to use the latter, you'll have to change the dependency format from

```rust
//! ```cargo
//! [dependencies]
//! structopt = "0.3"
//! log = "0.4"
//! simple_logger = "1.11"
//! ```
```
to 
```rust
//# [dependencies]
//# structopt = "0.3"
//# log = "0.4"
//# simple_logger = "1.11"
```
