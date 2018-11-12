extern crate git2;
extern crate failure;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;


use git2::{Repository, DescribeOptions};
use std::env;
use std::convert::AsRef;
use std::path::Path;
use failure::Error;

#[proc_macro]
pub fn git_version(input: TokenStream) -> TokenStream {
    let name : syn::Ident = syn::parse(input).expect("parse identifier");
    let vers = repository_version(".").expect("fetch git version");
    quote!(const #name : &'static str = #vers;).into()
}

fn repository_version<P: AsRef<Path>>(topdir: P) -> Result<String, Error> {
    let mut options = DescribeOptions::new();
    options.describe_tags().show_commit_oid_as_fallback(true);
    let repo = Repository::discover(topdir)?;
    let descr = repo.describe(&options)?;
    Ok(descr.format(None)?)
}