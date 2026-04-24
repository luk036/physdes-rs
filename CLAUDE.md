# CLAUDE.md

## Stack
- Rust library for VLSI physical design (geometric algorithms)
- Edition 2021, MSRV 1.70
- Library crate name: `physdes` (use `use physdes::...`)

## Build Commands
```bash
cargo fmt --all -- --check  # formatting check
cargo clippy --all-targets --all-features --workspace  # lint
cargo test --all-features --workspace  # tests
cargo doc --no-deps --document-private-items  # docs (fails on warnings)
cargo bench  # benchmarks
cargo fuzz run [polygon_ops|interval_ops|point_ops]  # fuzzing
```

## Conventions
- Features: `std` (default), `serialize` (serde)
- No custom rustfmt.toml - uses defaults + project rustfmt.toml
- codespell ignores: "crate", "falsy"; excludes: Cargo.lock, target/, .svg$
- CHANGELOG: add under `[Unreleased]`, follow Keep a Changelog

## Architecture
- Entry: `src/lib.rs` - exports `Point`, `Polygon`, `RPolygon`, `Vector2`
- Modules: `point`, `vector2`, `interval`, `polygon`, `rpolygon`, `merge_obj`, `algorithms`, `vlsi_ops`
- Fuzz workspace: `fuzz/` is separate - `cargo test` in root tests main crate only

## Testing
- Uses quickcheck + quickcheck_macros for property-based testing
- Run single test: `cargo test <test_name>`