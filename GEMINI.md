# Project: physdes-rs

## Project Overview

`physdes-rs` is a Rust library for physical design in VLSI. It provides data structures and algorithms for working with rectilinear shapes, such as points, vectors, polygons, and intervals. The library is designed to be used in applications that require the manipulation of geometric objects, such as CAD tools for integrated circuit design.

## Building and Running

### Building the library

To build the library, run the following command:

```bash
cargo build
```

### Running tests

To run the test suite, use the following command:

```bash
cargo test --all-features --workspace
```

### Checking formatting

To check the code formatting, run:

```bash
cargo fmt --all --check
```

### Linting

To lint the code with clippy, run:

```bash
cargo clippy --all-targets --all-features --workspace
```

### Building documentation

To build the documentation, run:

```bash
cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

## Development Conventions

### Code Style

The project uses the standard Rust formatting tool, `rustfmt`. Before committing any changes, make sure to run `cargo fmt` to format the code.

### Testing

The project uses `quickcheck` for property-based testing. All new functionality should be accompanied by tests.

### Contribution Guidelines

Contributions are welcome. Before submitting a pull request, please ensure that the code passes all tests, formatting checks, and linter checks. See the `CONTRIBUTING.md` file for more details.
