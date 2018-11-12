extern crate git2;
#[macro_use]
extern crate quick_error;

use git2::{Repository, DescribeOptions};
use std::env;
use std::convert::AsRef;
use std::fs::{File, create_dir_all};
use std::io::{Write, Read, BufWriter};
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


fn same_content_as<P: AsRef<Path>>(path: P, content: &str) -> Result<bool, Error> {

    let mut f = try!(File::open(path));
    let mut current = String::new();
    try!(f.read_to_string(&mut current));

    Ok(current == content)
}

pub fn write_version <P: AsRef<Path>>(topdir: P) -> Result<(), Error> {
    let path = try!(env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar));
    let path : &Path = path.as_ref();
    write_version_into(topdir.as_ref(), path.as_ref())
}


pub fn write_version_into(topdir: &Path, path: &Path) -> Result<(), Error> {
    try!(create_dir_all(path));

    let verfile = path.join("version.rs");
    let repo = try!(Repository::discover(topdir));
    let desc = try!(repo.describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true)));


    let content = format!("static VERSION: &'static str = {:?};\n", try!(desc.format(None)));

    let is_fresh = if verfile.exists() {
        try!(same_content_as(&verfile, &content))
    } else {
        false
    };

    if !is_fresh {
      let mut file = BufWriter::new(try!(File::create(&verfile)));

      try!(write!(file, "{}", content));
    }
    Ok(())
}

#[test]
fn test() {
    write_version_into(Path::new("."), Path::new("/tmp/git-build-version")).expect("write version");
}
