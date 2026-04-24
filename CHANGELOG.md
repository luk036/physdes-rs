# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cross-links to polyglot implementations (physdes-cpp, physdes-py)
- SVG document comments (`svgbobdoc`) for `cargo doc`
- const-fn support for core types
- quickcheck property-based tests

### Changed

- Apply `cargo clippy --fix` and `cargo fmt` fixes
- Improved variable naming (minimum 3 characters)

### Fixed

- Various clippy warnings
