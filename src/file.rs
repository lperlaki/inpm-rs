use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};
use std::path::Path;
use std::str;

/// A file with its contents stored in a `&'static [u8]`.
#[derive(Copy, Clone, PartialEq)]
pub struct File<'a> {
    pub path: &'a str,
    #[cfg(any(not(debug_assertions), feature = "embed"))]
    pub embedded: &'a [u8],
    #[cfg(all(debug_assertions, not(feature = "embed")))]
    pub abs_path: &'a str,
}

impl<'a> File<'a> {
    pub fn path(&self) -> &Path {
        Path::new(self.path)
    }

    #[cfg(any(not(debug_assertions), feature = "embed"))]
    pub fn contents(&self) -> Cow<'a, [u8]> {
        self.embedded.into()
    }

    #[cfg(all(debug_assertions, not(feature = "embed")))]
    pub fn contents(&self) -> Cow<'a, [u8]> {
        std::fs::read(self.abs_path).unwrap().into()
    }

    pub fn contents_utf8(&self) -> Option<Cow<'a, str>> {
        Some(match self.contents() {
            Cow::Borrowed(slice) => Cow::Borrowed(str::from_utf8(slice).ok()?),
            Cow::Owned(vec) => Cow::Owned(String::from_utf8(vec).ok()?),
        })
    }
}

impl<'a> Debug for File<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("contents", &format!("<{} bytes>", self.contents().len()))
            .finish()
    }
}
