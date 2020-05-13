//! # Inlucde NPM
//!
//! Include Static `npm build` ouput in your rust binary
//! When you compile in debug mode the File contents will just be read from disk and not embedded. This can manually be overriden with the `embed` feautere.
//!
//! The argument path is the ouput directory of the npm build. The parent directory is the directory with package.json
//!
//! ```ignore
//! const ASSETS: inpm::Dir<'static> = inpm::include_package!("./client/dist");
//!
//!
//! let content_of_my_file = ASSETS.get("some_dir/my_file.txt").unwrap().contents();
//!
//! ```
//!
//! ## Warp feature
//!
//! `features=["warp"]`
//!
//! ```ignore
//! const ASSETS: inpm::Dir<'static> = inpm::include_package!("./client/dist");
//!
//!
//! let my_file_filter = inpm::warp::embedded(ASSETS);
//!
//! // Allso works with single page applications
//!
//! let my_spa_filter = inpm::warp::spa(ASSETS, "index.html");
//!
//!
//! ```

use proc_macro_hack::proc_macro_hack;

mod dir;
mod file;
pub use crate::dir::Dir;
pub use crate::file::File;

#[proc_macro_hack]
pub use inpm_impl::include_package;

#[cfg(feature = "warp")]
pub mod warp {
    use crate::File;
    use warpd::{
        reject::Rejection,
        reply::{self, Reply},
        Filter,
    };

    fn with<T: Clone + Send + Sync>(
        source: T,
    ) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
        warpd::any().map(move || source.clone())
    }

    pub fn embedded(
        dir: crate::Dir<'static>,
    ) -> impl Filter<Extract = (warpd::reply::WithHeader<File<'static>>,), Error = Rejection> + Clone
    {
        warpd::get()
            .and(warpd::path::tail())
            .and(with(dir))
            .and_then(
                |tail: warpd::path::Tail, dir: crate::Dir<'static>| async move {
                    dir.get(tail.as_str())
                        .map(|file| {
                            warpd::reply::with_header(
                                file,
                                warpd::http::header::CONTENT_TYPE,
                                mime_guess::from_path(file.path())
                                    .first_or_octet_stream()
                                    .as_ref(),
                            )
                        })
                        .ok_or(warpd::reject::not_found())
                },
            )
    }

    // Put this last in the filter chain
    pub fn spa(
        dir: crate::Dir<'static>,
        entry: &'static str,
    ) -> impl Filter<Extract = (warpd::reply::WithHeader<File<'static>>,), Error = Rejection> + Clone
    {
        warpd::get()
            .and(warpd::path::tail())
            .and(with(dir))
            .and(with(entry))
            .and_then(
                |tail: warpd::path::Tail, dir: crate::Dir<'static>, entry: &'static str| async move {
                    dir.get(tail.as_str())
                        .or(dir.get(entry))
                        .map(|file| {
                            warpd::reply::with_header(
                                file,
                                warpd::http::header::CONTENT_TYPE,
                                mime_guess::from_path(file.path())
                                    .first_or_octet_stream()
                                    .as_ref(),
                            )
                        })
                        .ok_or(warpd::reject::not_found())
                },
            )
    }

    impl Reply for File<'_> {
        fn into_response(self) -> reply::Response {
            self.contents().into_owned().into_response()
        }
    }
}
