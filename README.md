# Prototype Building Rust Rocket web application with Bazel


## Build

For web-api
- go into web-api/cargo
- `cargo generate-lockfile` (will rebuild Cargo.lock) (might need to delete .cargo/confgi)
- `cargo vendor --versioned-dirs --locked` (pulls all libs to the local vendor dir)
- `cargo raze`
- go to root of the project
- `bazel build //web-api`
