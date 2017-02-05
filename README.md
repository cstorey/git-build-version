# git-build-version

Makes it easy to include a version (as provided by `git describe`) in your crate. For example:

In `Cargo.toml`:

```toml
[package]
name = "my-lovely-package"
# ...
build = "build.rs"

[build-dependencies]
git-build-version = "*"
```

In `build.rs`:

```rust
extern crate git_build_version;

const PACKAGE_TOP_DIR : &'static str = ".";

fn main() {
    git_version::write_version(PACKAGE_TOP_DIR).expect("Saving git version");
}
```

This will write out a file named `version.rs` that can be included into your source as follows. Eg: in your `src/main.rs`:

```rust
include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
    println!("Version: {}", VERSION);
}
```
