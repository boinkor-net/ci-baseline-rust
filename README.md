# Reusable baseline CI workflows for Rust

The workflows in this repo all help drive the CI in this organization's Rust repos. There are two workflows defined that are reusable:

## `ci_baseline_rust_tests.yml` - Matrix-able tests

This workflow runs `cargo nextest run` with customizable settings:

* `rust_toolchain` - the name of the rust toolchain you'd use in `cargo +toolchain_name test`; defaults to `"stable"`
* `cargo_test_args` - additional arguments passed to `cargo test`, e.g. for feature selection.

Example github workflow job:

```yml
jobs:
  rust_tests:
    uses: "boinkor-net/ci-baseline-rust/.github/workflows/ci_baseline_rust_tests.yml@main"
    strategy:
      matrix:
        rust_toolchain: [nightly, stable]
        cargo_args:
          - "--no-default-features --features no_std"
          - "--no-default-features --features 'jitter no_std'"
          - "--no-default-features --features std"
          - ""
    with:
      rust_toolchain: ${{matrix.rust_toolchain}}
      cargo_test_args: ${{matrix.cargo_test_args}}
```

## `ci_baseline_rust_lints.yml` - Lints for your repo

This workflow runs some common linter checks on a repo:

* `cargo fmt` - to see if the code formatting is reasonable.
* `cargo clippy` - checks if the code follows extra coding guidelines. Options:
  * `cargo_clippy_args` (default `"-- -D warnings"`) - commandline arguments passed to clippy.
* `cargo deny` - checks licenses and vulnerable dependencies; this check is skipped if no `deny.toml` exists on the repo root.

Example job:

```yml
jobs:
  rust_lints:
    uses: "boinkor-net/ci-baseline-rust/.github/workflows/ci_baseline_rust_lints.yml@main"
```
