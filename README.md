# Reusable baseline CI workflows for Rust [![CI](https://github.com/boinkor-net/ci-baseline-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/boinkor-net/ci-baseline-rust/actions/workflows/ci.yml)

The workflows in this repo all help drive the CI in this organization's Rust repos. There are a few workflows defined here that are [reusable](https://docs.github.com/en/actions/using-workflows/reusing-workflows) in github actions.

All these workflows take common arguments:

* `manifest_dir` - the directory containing Cargo.toml for the codebase under test.
* `rust_toolchain` - the name of the rust toolchain you'd use in `cargo +toolchain_name test`; defaults to `"stable"`. (The only workflow that doesn't accept this input is `ci_baseline_rust_coverage.yml`, due to unstable cli options for testing doctests. It's set to `"nightly"`.)


## `ci_baseline_rust_tests.yml` - Matrix-able tests

This workflow runs `cargo nextest run` with customizable settings:

* `cargo_test_args` - additional arguments passed to `cargo test`, e.g. for feature selection.

Example github workflow job:

```yml
jobs:
  rust_tests:
    uses: "boinkor-net/ci-baseline-rust/.github/workflows/ci_baseline_rust_tests.yml@v1.0.0"
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
    uses: "boinkor-net/ci-baseline-rust/.github/workflows/ci_baseline_rust_lints.yml@v1.0.0"
```

## `ci_baseline_rust_coverage.yml` - Test coverage on codecov.io

This workflow runs tests (unit, integration and doctests) and gathers all their coverage information and uploads it to https://codecov.io. Since gathering doctest coverage requires nightly, this is nightly-only at the moment.

It requires secrets:

* `CODECOV_TOKEN` - the token issued by codecov.io for your repo

And comes with customization options:

* `cargo_test_args` - any additional args to pass to cargo (e.g., for feature selection).

Example job:

```yml
jobs:
  code_coverage:
    uses: boinkor-net/ci-baseline-rust/.github/workflows/ci_baseline_rust_coverage.yml@v1.0.0
    secrets:
      CODECOV_TOKEN: ${{ secrets.CODECOV_TOKEN }}
```
