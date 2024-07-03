# datastar-rs
A Rust helper library for crafting backend responses for the [datastar](https://github.com/delaneyj/datastar) hypermedia library.

## Axum integration
With the `axum` feature turned on, the `FullDatastarResponse` and `StreamingDatastarResponse` types will implement `IntoResponse`, allowing them to be returned from Axum handler functions.