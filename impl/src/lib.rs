extern crate proc_macro;

use proc_macro2::Span;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, Error, LitStr, Result};

use crate::dir::Dir;
use std::path::PathBuf;

mod command;
mod dir;
mod file;

fn do_it(path: PathBuf) -> Result<Dir> {
    let parent = path.parent().expect("No Parent Direcotrio");

    if !parent.exists() {
        return Err(Error::new(
            Span::call_site(),
            format!("Folder {} doesnot exist", parent.display()),
        ));
    }

    let parent = parent.canonicalize().unwrap();

    if cfg!(any(not(debug_assertions), feature = "embed")) || !path.exists() {
        if !parent.join("package.json").exists() {
            return Err(Error::new(
                Span::call_site(),
                format!("Folder {} doenot contain package.json", parent.display()),
            ));
        }
        command::run_command(
            vec!["npm", "install", "--no-package-lock", "--no-audit"],
            &parent,
        )?;
        command::run_command(vec!["npm", "run", "build"], &parent)?;
    }

    if !path.exists() {
        return Err(Error::new(
            Span::call_site(),
            format!("Folder {} doesnot exist after build", path.display()),
        ));
    }

    let path = path.canonicalize().unwrap();

    Dir::from_disk(&path, &path)
}

#[proc_macro_hack]
pub fn include_package(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = PathBuf::from(parse_macro_input!(input as LitStr).value());

    do_it(path)
        .map(|dir| {
            quote! {
                #dir
            }
        })
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}
