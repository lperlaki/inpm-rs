use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};
use std::path::Path;
use std::str;

#[cfg(any(not(debug_assertions), feature = "embed"))]
#[derive(Clone, PartialEq)]
pub struct File {
    path: &'static str,
    embedded: &'static [u8],
}

/// A file with its contents stored in a `&'static [u8]`.
#[cfg(all(debug_assertions, not(feature = "embed")))]
#[derive(Clone, PartialEq)]
pub struct File {
    path: std::path::PathBuf,
    abs_path: std::path::PathBuf,
}

impl File {
    #[cfg(any(not(debug_assertions), feature = "embed"))]
    #[inline]
    #[doc(hidden)]
    pub const fn new(path: &'static str, embedded: &'static [u8]) -> Self {
        Self { path, embedded }
    }

    #[cfg(all(debug_assertions, not(feature = "embed")))]
    #[inline]
    #[doc(hidden)]
    pub const fn new(path: std::path::PathBuf, abs_path: std::path::PathBuf) -> Self {
        Self { path, abs_path }
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    #[cfg(any(not(debug_assertions), feature = "embed"))]
    pub fn contents(&self) -> Cow<'static, [u8]> {
        self.embedded.into()
    }

    #[cfg(all(debug_assertions, not(feature = "embed")))]
    pub fn contents(&self) -> Cow<'static, [u8]> {
        std::fs::read(&self.abs_path).unwrap().into()
    }

    pub fn contents_utf8(&self) -> Option<Cow<'static, str>> {
        Some(match self.contents() {
            Cow::Borrowed(slice) => Cow::Borrowed(str::from_utf8(slice).ok()?),
            Cow::Owned(vec) => Cow::Owned(String::from_utf8(vec).ok()?),
        })
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("contents", &format!("<{} bytes>", self.contents().len()))
            .finish()
    }
}
