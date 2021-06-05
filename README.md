[![Rust](https://github.com/Daan4/hv-conductor-visualizer/actions/workflows/rust.yml/badge.svg)](https://github.com/Daan4/hv-conductor-visualizer/actions/workflows/rust.yml)

Work in progress. No guarantees given as per the license.

Simulate and visualize simple (high-voltage) electrical networks in steady state conditions as a single line diagram.

Current functionality:

* Build any network via CLI or Rust API (supports circuit breaker, disconnector, earthing switch, voltage transformer, transformer)

(Potential) Future functionality/ideas (in random order):

* Simulation
* Visualization (Interactive Web Interface)
* 61850 support in some form
* Parse networks from drawings
* Reading/Writing networks from/to files

Build

    cargo build

Run tests

    cargo test

Run CLI

    cargo run

Generate docs

    rustdoc src/lib.rs
