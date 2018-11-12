
#[macro_use]
extern crate git_build_version;

git_version!(VERSION);

#[test]
fn should_have_version() {
    println!("VERSION: {}", VERSION);
    assert_ne!("", VERSION)
}