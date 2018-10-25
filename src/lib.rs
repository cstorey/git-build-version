extern crate git2;
#[macro_use]
extern crate quick_error;

use git2::{Repository, DescribeOptions};
use std::env;
use std::convert::AsRef;
use std::fs::{File, create_dir_all};
use std::io::{Write, Read, BufWriter};
use std::path::Path;
use std::path::PathBuf;
use std::fs;

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
        InvalidGitFormat {
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

    try!(create_dir_all(path));

    let path = path.join("version.rs");

    let repo = try!(Repository::discover(topdir));
    let desc = try!(repo.describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true)));


    let content = format!("static VERSION: &'static str = {:?};\n", try!(desc.format(None)));

    let is_fresh = if path.exists() {
        try!(same_content_as(&path, &content))
    } else {
        false
    };

    if !is_fresh {
      let mut file = BufWriter::new(try!(File::create(&path)));

      try!(write!(file, "{}", content));
    }
    Ok(())
}

pub fn print_version<P: AsRef<Path>>(topdir: P) -> Result<(), Error> {
    let repo = try!(Repository::discover(topdir));
    let desc = try!(repo.describe(&DescribeOptions::new().describe_tags().show_commit_oid_as_fallback(true)));
    println!("cargo:rustc-env=GIT_SHA_SHORT={}", try!(desc.format(None)));
    let git_head_path = PathBuf::from(repo.path());
    generate_rerun_if_changed(git_head_path)
}

// Copied and modified from https://github.com/rustyhorde/vergen/blob/master/src/output/envvar.rs#L45
fn generate_rerun_if_changed(git_dir_or_file : PathBuf) -> Result<(), Error> {
    eprintln!("Git repository: {}", git_dir_or_file.display());
    if let Ok(metadata) = fs::metadata(&git_dir_or_file) {
        if metadata.is_dir() {
            // Echo the HEAD path
            let git_head_path = git_dir_or_file.join("HEAD");
            println!("cargo:rerun-if-changed={}", git_head_path.display());

            // Determine where HEAD points and echo that path also.
            let mut f = File::open(&git_head_path)?;
            let mut git_head_contents = String::new();
            let _ = f.read_to_string(&mut git_head_contents)?;
            eprintln!("HEAD contents: {}", git_head_contents);
            let ref_vec: Vec<&str> = git_head_contents.split(": ").collect();

            if ref_vec.len() == 2 {
                let current_head_file = ref_vec[1];
                let git_refs_path = git_dir_or_file.join(current_head_file);
                println!("cargo:rerun-if-changed={}", git_refs_path.display());
            } else {
                eprintln!("You are most likely in a detached HEAD state");
            };
        } else if metadata.is_file() {
            // We are in a worktree, so find out where the actual worktrees/<name>/HEAD file is.
            let mut git_file = File::open(&git_dir_or_file)?;
            let mut git_contents = String::new();
            let _ = git_file.read_to_string(&mut git_contents)?;
            let dir_vec: Vec<&str> = git_contents.split(": ").collect();
            eprintln!(".git contents: {}", git_contents);
            let git_path = dir_vec[1].trim();

            // Echo the HEAD psth
            let git_head_path = PathBuf::from(git_path).join("HEAD");
            println!("cargo:rerun-if-changed={}", git_head_path.display());

            // Find out what the full path to the .git dir is.
            let mut actual_git_dir = PathBuf::from(git_path);
            actual_git_dir.pop();
            actual_git_dir.pop();

            // Determine where HEAD points and echo that path also.
            let mut f = File::open(&git_head_path)?;
            let mut git_head_contents = String::new();
            let _ = f.read_to_string(&mut git_head_contents)?;
            eprintln!("HEAD contents: {}", git_head_contents);
            let ref_vec: Vec<&str> = git_head_contents.split(": ").collect();

            if ref_vec.len() == 2 {
                let current_head_file = ref_vec[1];
                let git_refs_path = actual_git_dir.join(current_head_file);
                println!("cargo:rerun-if-changed={}", git_refs_path.display());
            } else {
                eprintln!("You are most likely in a detached HEAD state");
            };
        } else {
            return Err(Error::MissingEnvVar);
        };
    } else {
        eprintln!("Unable to generate 'cargo:rerun-if-changed'");
    };

    Ok(())
}

#[test]
fn test() {
    print_version(".").expect("write version");
}
