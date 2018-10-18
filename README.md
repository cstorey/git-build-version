# git-build-version

Makes it easy to include a version (as provided by `git describe`) in your crate. For example:

In `Cargo.toml`:

```
[package]
name = "my-lovely-package"
# ...
build = "build.rs"

[build-dependencies]
git-build-version = "*"
```

## Using compile-time environment variable (since Rust 1.19)


In `build.rs`:

```
extern crate git_build_version;

const PACKAGE_TOP_DIR : &'static str = ".";

fn main() {
    git_build_version::print_version(PACKAGE_TOP_DIR).expect("Cannot get git revision");
}
```
In your source files, eg: in your `src/main.rs`:
```
fn main() {
    println!("Version: {} Build: {}", env!("CARGO_PKG_VERSION"), env!("GIT_SHA_SHORT"));
}
```

## Using generated version.rs


In `build.rs`:

```
extern crate git_build_version;

const PACKAGE_TOP_DIR : &'static str = ".";

fn main() {
    git_version::write_version(PACKAGE_TOP_DIR).expect("Saving git version failed");
}
```

This will write out a file named `version.rs` that can be included into your source as follows. Eg: in your `src/main.rs`:

```
include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
    println!("Version: {}", VERSION);
}
```
