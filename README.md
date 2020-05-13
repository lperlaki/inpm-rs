# Inlucde NPM

Include Static `npm build` ouput in your rust binary

When you compile in debug mode the File contents will just be read from disk and not embedded. This can manually be overriden with the `embed` feautere.

```rust
const ASSETS: inpm::Dir = inpm::include_package!("./client/dist");


let content_of_my_file = ASSETS.get("some_dir/my_file.txt").contents();

```

## Warp feature

`features=["warp"]`

```rust
const ASSETS: inpm::Dir = inpm::include_package!("./client/dist");


let my_file_filter = inpm::warp::embedded(ASSETS);

// Allso works with single page applications

let my_spa_filter = inpm::warp::spa(ASSETS, "index.html");


```
