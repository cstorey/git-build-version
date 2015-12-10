extern crate git2;
#[macro_use]
extern crate quick_error;
use git2::{Repository, Tag, Commit, DescribeOptions};
use std::env;
use std::convert::AsRef;
use std::fs::File;
use std::io::{self,Write, BufWriter};
use std::path::Path;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            from()
        }
        Git(err: git2::Error) {
            from()
        }
        MissingEnvVar {
        }
    }
}
pub fn write_version <P: AsRef<Path>>(topdir: P) -> Result<(), Error> {
    let path = try!(env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar));
    let path : &Path = path.as_ref();

    let path = path.join("version.rs");
    let mut file = BufWriter::new(try!(File::create(&path)));

    let repo = try!(Repository::open("../.."));
    let desc = try!(repo.describe(&DescribeOptions::new()));
    try!(writeln!(file, r#"static VERSION: &'static str = {:?};"#, desc.format(None).unwrap()));
    Ok(())
}
