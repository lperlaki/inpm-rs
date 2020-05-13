use crate::file::File;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::path::{Path, PathBuf};
use syn::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Dir {
    root_rel_path: PathBuf,
    abs_path: PathBuf,
    files: Vec<File>,
}

impl Dir {
    pub fn from_disk<Q: AsRef<Path>, P: Into<PathBuf>>(root: Q, path: P) -> Result<Dir> {
        let abs_path = path.into();
        let root = root.as_ref();

        let root_rel_path = abs_path.strip_prefix(&root).unwrap().to_path_buf();

        if !abs_path.exists() {
            return Err(Error::new(
                Span::call_site(),
                format!("The directory doesn't exist"),
            ));
        }

        let files = walkdir::WalkDir::new(&abs_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .map(|path| File::from_disk(&root, path))
            .collect::<Result<_>>()?;

        Ok(Dir {
            root_rel_path,
            abs_path,
            files,
        })
    }
}

impl ToTokens for Dir {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let root_rel_path = self.root_rel_path.display().to_string();
        let files = &self.files;

        let tok = quote! {
            $crate::Dir {
                path: #root_rel_path,
                files: &[#(
                    #files
                 ),*],
            }
        };

        tok.to_tokens(tokens);
    }
}
