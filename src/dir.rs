use crate::file::File;
use std::path::Path;

/// An embedded Directory
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dir {
    path: &'static str,
    #[cfg(any(not(debug_assertions), feature = "embed"))]
    files: &'static [File],
}

impl Dir {
    #[cfg(any(not(debug_assertions), feature = "embed"))]
    #[inline]
    #[doc(hidden)]
    pub const fn new(path: &'static str, files: &'static [File]) -> Self {
        Self { path, files }
    }

    #[cfg(all(debug_assertions, not(feature = "embed")))]
    #[inline]
    #[doc(hidden)]
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        Path::new(self.path)
    }

    #[cfg(all(debug_assertions, not(feature = "embed")))]
    pub fn files(&self) -> impl Iterator<Item = File> + '_ {
        let root: std::path::PathBuf = self.path().into();
        walkdir::WalkDir::new(self.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .map(move |abs_path| {
                File::new(
                    abs_path.strip_prefix(&root).unwrap().to_path_buf(),
                    abs_path,
                )
            })
    }
    #[cfg(any(not(debug_assertions), feature = "embed"))]
    pub fn files(&self) -> impl Iterator<Item = File> + '_ {
        self.files.iter().map(|file| file.clone())
    }
    pub fn contains(&self, path: impl AsRef<Path>) -> bool {
        self.get(path).is_some()
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<File> {
        let path = path.as_ref();
        self.files().find(|file| file.path() == path)
    }
}
