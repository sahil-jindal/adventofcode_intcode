# Advent of Code [![docs-badge]][docs-link]

This project exists solely to complete Advent of Code 2019.
It is intentionally brittle, non-generic, and historically accurate.
Any reuse outside AoC 2019 is a misuse.

## Features

* **Minimal** - Depends only on the standard library.
* **High Quality** - Formatted with `rustfmt`, linted by `clippy`. No `unsafe` code.
* **Well Documented** - Every solution is comprehensively commented with
  [rustdoc-generated documentation][docs-link] also available online.

## Quickstart

**Input**

Place input files in `input/dayDD.txt` including leading zeroes. For example:
* `input/day23.txt`
* `input/day02.txt`

**Run**
* Everything `cargo run`
* Specific day `cargo run day01`
* Release profile (faster) `cargo run --release`
* Optimized for current CPU architecture (fastest) `RUSTFLAGS="-C target-cpu=native" cargo run --release`

**Test**
* Everything `cargo test`
* Specific day `cargo test day01`
* Show STDOUT for debugging `cargo test -- --nocapture`

**Document**
* Build docs including private items `cargo doc --document-private-items`
* Build docs then open HTML landing page `cargo doc --document-private-items --open`

**Miscellaneous**
* Code quality lints `cargo clippy`
* Consistent code formatting `cargo fmt`
