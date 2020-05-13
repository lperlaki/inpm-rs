use crate::file::File;
use std::path::Path;

/// A directory entry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir<'a> {
    pub path: &'a str,
    pub files: &'a [File<'a>],
}

impl<'a> Dir<'a> {
    /// Get the directory's path.
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    pub fn files(&self) -> impl Iterator<Item = &File<'a>> {
        self.files.iter()
    }

    pub fn contains(&self, path: impl AsRef<Path>) -> bool {
        self.get(path).is_some()
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<File<'a>> {
        let path = path.as_ref();
        self.files()
            .find(|file| file.path() == path)
            .map(|file| *file)
    }
}
