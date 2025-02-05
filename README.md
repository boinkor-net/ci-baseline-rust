# Reusable baseline CI actions and workflows for Rust [![CI](https://github.com/boinkor-net/ci-baseline-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/boinkor-net/ci-baseline-rust/actions/workflows/ci.yml)

The actions in this repo all help drive the CI in this organization's Rust repos. There is one workflow defined here that are [reusable](https://docs.github.com/en/actions/using-workflows/reusing-workflows) in github actions, the rest is all composite actions.

For concrete usage, see [the e2e.yml workflow](.github/workflows/e2e.yml).

All these actions and workflows take common arguments:

* `manifest_dir` - the directory containing Cargo.toml for the codebase under test.
* `rust_toolchain` - the name of the rust toolchain you'd use in `cargo +toolchain_name test`; defaults to `"stable"`. (The only workflow that doesn't accept this input is `ci_baseline_rust_coverage.yml`, due to unstable cli options for testing doctests. It's set to `"nightly"`.)
* `apt_install_packages` - `ubuntu-latest` packages to install before running any cargo builds. These are cached and should restore quickly, but can't depend on pre/postinstall scripts.

# Actions

## `actions/test` - Matrix-able tests

This workflow runs `cargo nextest run` with customizable settings:

* `cargo_test_args` - additional arguments passed to `cargo test`, e.g. for feature selection.

Example github workflow job:

```yml
jobs:
  success_tests:
    strategy:
      fail-fast: false
      matrix:
        rust_toolchain: ["stable", "nightly"]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./actions/test
        with:
          manifest_dir: "tests/success"
          rust_toolchain: ${{ matrix.rust_toolchain }}
```

## `actions/fmt`, `actions/clippy`, `actions/cargo_deny` - Lints for your repo

These actions run the following commands:

* `fmt` - to see if the code formatting is reasonable.
* `clippy` - checks if the code follows extra coding guidelines. Options:
  * `cargo_clippy_args` (default `"-- -D warnings"`) - commandline arguments passed to clippy.
* `cargo_deny` - checks licenses and vulnerable dependencies; this check is skipped if no `deny.toml` exists on the repo root.

Example jobs:

```yml
  success_fmt:
    strategy:
      fail-fast: false
      matrix:
        rust_toolchain: ["stable", "nightly"]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./actions/fmt
        with:
          manifest_dir: "tests/success"
          rust_toolchain: ${{ matrix.rust_toolchain }}

  success_clippy:
    strategy:
      fail-fast: false
      matrix:
        rust_toolchain: ["stable", "nightly"]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./actions/clippy
        with:
          manifest_dir: "tests/success"
          rust_toolchain: ${{ matrix.rust_toolchain }}

  success_deny:
    strategy:
      fail-fast: false
      matrix:
        rust_toolchain: ["stable", "nightly"]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./actions/cargo_deny
        with:
          manifest_dir: "tests/success"
          rust_toolchain: ${{ matrix.rust_toolchain }}
```

# Reusable Workflows
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
    uses: boinkor-net/ci-baseline-rust/.github/workflows/ci_baseline_rust_coverage.yml@main
    secrets:
      CODECOV_TOKEN: ${{ secrets.CODECOV_TOKEN }}
```
