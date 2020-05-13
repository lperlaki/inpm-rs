# Inlucde NPM

Include Static `npm build` ouput in your rust binary

When you compile the file contents in debug mode, it will be read from disk and not embedded. This can manually be overriden with the `embed` feature.

```rust
const ASSETS: inpm::Dir = inpm::include_package!("./client/dist");


let content_of_my_file = ASSETS.get("some_dir/my_file.txt").contents();

```

## Warp feature

`features=["warp"]`

```rust
const ASSETS: inpm::Dir = inpm::include_package!("./client/dist");


let my_file_filter = inpm::warp::embedded(ASSETS);

// Also works with single page applications

let my_spa_filter = inpm::warp::spa(ASSETS, "index.html");


```
