# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Steiner forest grid module (`steiner_forest_grid`) matching C++/Python
- Cross-language verification tests (11 integration tests)
- Polygon `signed_area_x2` and RPolygon `signed_area` benchmarks
- Cross-links to polyglot implementations (physdes-cpp, physdes-py)
- SVG document comments (`svgbobdoc`) for `cargo doc`
- const-fn support for core types
- quickcheck property-based tests

### Changed

- `GlobalRouter::new` terminal sort: added deterministic tiebreaker
- Sync with sibling C++/Python projects — all three now produce identical numerical results
- Sync with sibling C++ project (physdes-cpp) by opencode + deepseek-v4-flash
- `GlobalRoutingTree` insertion search prunes subtrees via per-node bounding boxes (output-preserving; ~3.8x faster at 2000 terminals)
- `steiner_forest_grid` caches component roots once per iteration instead of calling `find` per edge (output-preserving)

### Fixed

- Various clippy warnings
