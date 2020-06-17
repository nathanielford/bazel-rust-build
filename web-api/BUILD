load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "web-api",
    srcs = ["src/main.rs"],
    deps = [
        "//web-api/cargo:rocket",
    ],
)