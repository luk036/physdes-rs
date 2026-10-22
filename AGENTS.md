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

## RPolygon Representation

The canonical point-sequence representation of a rectilinear polygon is the
**even-index representation**, not a full vertex list.

`RPolygon` stores the origin plus the displacement vectors of the even-index
vertices; the omitted vertices are implicit. A closed rectilinear polygon with
`n` vertices stores `n / 2` vertices.

### Canonical encoding

Start from a simple rectilinear polygon with `n` corner vertices (remove
collinear vertices first). Rotate the full vertex list so its first edge is
horizontal. Store every other vertex:

```text
S[i] = full[(start + 2*i) % n],   i = 0 .. n/2 - 1
```

The omitted vertex between two stored vertices is recovered by moving
**x first, then y**:

```text
S[i]   = (x0, y0)
S[i+1] = (x1, y1)
implicit vertex = (x1, y0)
```

Therefore the full boundary is:

```text
S0, (S1.x, S0.y), S1, (S2.x, S1.y), S2, ...
```

Example:

```text
stored:  (0,0) (3,3) (2,1) (1,3)
full:    (0,0) (3,0) (3,3) (2,3) (2,1) (1,1) (1,3) (0,3)
```

Do **not** use the y-first form `(x0, y1)`; that reconstructs a different
polygon. `rpolygon_to_polygon()` / `RPolygon::to_polygon()` implement the
x-first expansion.

### Invariants

- The list has at least **2** stored points.
- Consecutive stored points differ in **both** `x` and `y`. If they share an
  axis, the list is not a canonical even-index representation.
- The stored order follows the boundary order; do not reverse it.
- Start on a vertex whose outgoing edge is horizontal. Starting on a vertical
  edge makes the x-first rule reconstruct a different polygon.
- A rectangle is stored as **2** opposite corners. A 2-point representation
  whose points share `x` or `y` is invalid, not a thin rectangle.
- A monotone/L-shaped (orthogonally convex) polygon is stored as **3** points.
- More stored points describe a larger orthogonally convex polygon.
- The full boundary is reconstructed on demand; it is not the stored form.

### Representation forms

- 2 stored points: rectangle, stored as opposite corners.
- 3 stored points: L-shaped / orthogonally convex polygon.
- More stored points: larger orthogonally convex polygon.
- A 2-point or 3-point representation is valid, not degenerate.

### Deriving test fixtures

Build fixtures from the canonical full vertex list for every cut test. Do not
hand-write stored points or pass a raw full boundary.

```rust
use physdes::Point;

let full = vec![
    Point::new(0, 0), Point::new(3, 0), Point::new(3, 3), Point::new(2, 3),
    Point::new(2, 1), Point::new(1, 1), Point::new(1, 3), Point::new(0, 3),
];

// full[0] -> full[1] is horizontal, so start = 0
let stored: Vec<_> = full.iter().step_by(2).cloned().collect();
```

If the full list starts with a vertical edge, rotate it until the first edge is
horizontal before taking every other vertex.

### Rust-specific notes

- `rpolygon_cut_convex` and `rpolygon_cut_explicit` expect the **even-index**
  representation, not the full vertex list.
- For a clockwise polygon, take the even-index subset of the reversed full
  vertex list; do not reverse the stored list.
- A 2-point or 3-point piece in cut output is normal: it is the even-index
  representation, not a degenerate polygon.
- The cut base cases are 2 stored points (rectangle) and 3 stored points
  (monotone/convex piece).
- If a concavity predicate never fires, check the representation before
  changing the algorithm.
- `rpolygon_cut_implicit` is still a stub; do not assume it decomposes the
  even-index representation.

## Related Projects (Polyglot)

- [physdes-cpp](https://github.com/luk036/physdes-cpp)
- [physdes-py](https://github.com/luk036/physdes-py)