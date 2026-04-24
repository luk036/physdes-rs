# AGENTS.md

## Quick Commands

```bash
# All checks (in order)
cargo fmt --all -- --check  # formatting
cargo clippy --all-targets --all-features --workspace  # linting
cargo test --all-features --workspace  # tests
cargo doc --no-deps --document-private-items  # docs (fails on warnings)

# Benchmarks
cargo bench

# Fuzzing (requires: cargo install cargo-fuzz)
cargo fuzz run polygon_ops
cargo fuzz run interval_ops
cargo fuzz run point_ops
```

## Important Conventions

- **Library crate name**: `physdes` (not `physdes_rs`) — use `use physdes::...`
- **Features**: `std` (default), `serialize` (serde). Use `--all-features` for dev workflows.
- **MSRV**: 1.70
- **CHANGELOG.md**: Add entries under `[Unreleased]` section, follow Keep a Changelog categories
- **Quickcheck tests**: This repo uses `quickcheck` + `quickcheck_macros` for property-based testing
- **Fuzz workspace**: `fuzz/` is a separate workspace — `cargo test` in root only tests main crate

## Code Style

- No custom `rustfmt.toml` or `.clippy.toml` — uses Rust defaults
- Pre-commit runs: rust-fmt → rust-clippy → rust-test
- codespell ignores: `crate`, `falsy`; excludes `Cargo.lock`, `target/`, `.svg$`
- Use `svgbobdoc` doc comments (visible in `cargo doc`)

## Architecture

- **Entry point**: `src/lib.rs` — re-exports `Point`, `Polygon`, `RPolygon`, `Vector2`
- **Core modules**: `point`, `vector2`, `interval`, `polygon`, `rpolygon`, `merge_obj`, `algorithms`, `vlsi_ops`
- **Generic traits**: `src/generic.rs` — `MinDist`, `Overlap`, `Intersect` traits used by benchmarks
- **Benchmarks**: `benches/geometry_bench.rs` using Criterion
- **Fuzz targets**: `fuzz/fuzz_targets/{polygon_ops,interval_ops,point_ops}.rs`

## Related Projects (Polyglot)

- [physdes-cpp](https://github.com/luk036/physdes-cpp)
- [physdes-py](https://github.com/luk036/physdes-py)