use crate::file::File;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::path::PathBuf;
use syn::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Dir {
    root: PathBuf,
    #[cfg(any(not(debug_assertions), feature = "embed"))]
    files: Vec<File>,
}

impl Dir {
    pub fn from_disk(root: impl Into<PathBuf>) -> Result<Dir> {
        let root = root.into();

        if !root.exists() {
            return Err(Error::new(
                Span::call_site(),
                format!("The directory doesn't exist"),
            ));
        }
        #[cfg(any(not(debug_assertions), feature = "embed"))]
        let files = walkdir::WalkDir::new(&root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .map(|path| File::from_disk(&root, path))
            .collect::<Result<_>>()?;

        Ok(Dir {
            root,
            #[cfg(any(not(debug_assertions), feature = "embed"))]
            files,
        })
    }
}

impl ToTokens for Dir {
    #[cfg(any(not(debug_assertions), feature = "embed"))]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.root.display().to_string();
        let files = &self.files;

        let tok = quote! {
            $crate::Dir::new(
                 #path,
                &[#(
                    #files
                 ),*],
                )
        };

        tok.to_tokens(tokens);
    }

    #[cfg(all(debug_assertions, not(feature = "embed")))]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.root.display().to_string();

        let tok = quote! {
            $crate::Dir::new(
                #path,
            )
        };

        tok.to_tokens(tokens);
    }
}
