# 🧱 physdes-rs

[![Crates.io](https://img.shields.io/crates/v/physdes-rs.svg)](https://crates.io/crates/physdes-rs)
[![Docs.rs](https://docs.rs/physdes-rs/badge.svg)](https://docs.rs/physdes-rs)
[![CI](https://github.com/luk036/physdes-rs/workflows/CI/badge.svg)](https://github.com/luk036/physdes-rs/actions)
[![codecov](https://codecov.io/gh/luk036/physdes-rs/branch/master/graph/badge.svg?token=cvlHj6FLjO)](https://codecov.io/gh/luk036/physdes-rs)

<p align="center">
  <img src="./rectilinear-shapes-for-vlsi-physical-desgin.svg"/>
</p>

**physdes-rs** is a Rust library for VLSI physical design operations, providing efficient geometric data structures and algorithms for electronic design automation (EDA).

## 📚 Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [API Overview](#-api-overview)
- [Use Cases](#-use-cases)
- [Performance](#-performance)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

- **Points & Vectors**: Generic 2D point and vector operations with type-safe arithmetic
- **Intervals**: Efficient range operations and interval algebra
- **Polygons**: Arbitrary polygon support with area calculation and convex hull
- **Rectilinear Polygons**: Optimized algorithms for Manhattan geometry
- **VLSI Operations**: Specialized operations for circuit layout and physical design
- **Generic Types**: Support for multiple numeric types (i32, f64, etc.)
- **No-std Support**: Can be used in embedded environments (with `--no-default-features`)

## 🛠️ Installation

### 📦 Cargo

Add this to your `Cargo.toml`:

```toml
[dependencies]
physdes-rs = "0.1"
```

Or install the binary:

```bash
cargo install physdes-rs
```

## 🚀 Quick Start

### Creating and Manipulating Points

```rust
use physdes::Point;

// Create a point
let p = Point::new(3, 4);

// Access coordinates
println!("Point: ({}, {})", p.xcoord, p.ycoord);

// Arithmetic operations with vectors
use physdes::Vector2;
let v = Vector2::new(1, 2);
let p2 = p + v;

// Comparison operations
let p3 = Point::new(5, 6);
assert!(p2 == p3);
```

### Working with Intervals

```rust
use physdes::interval::Interval;

// Create intervals
let interval_a = Interval::new(1, 5);
let interval_b = Interval::new(3, 7);

// Check overlap
assert!(interval_a.overlaps(&interval_b));

// Compute intersection
let intersection = interval_a.intersect(&interval_b);
assert_eq!(intersection, Some(Interval::new(3, 5)));

// Check containment
assert!(interval_a.contains(&3));
```

### Creating and Analyzing Polygons

```rust
use physdes::Polygon;
use physdes::Point;

// Create a square polygon
let points = vec![
    Point::new(0, 0),
    Point::new(1, 0),
    Point::new(1, 1),
    Point::new(0, 1),
];
let polygon = Polygon::new(&points);

// Calculate area
let area = polygon.area();
assert_eq!(area, 1.0);

// Check if polygon is convex
assert!(polygon.is_convex());

// Get bounding box
let bbox = polygon.bounding_box();
```

### Vector Operations

```rust
use physdes::Vector2;

// Create vectors
let v1 = Vector2::new(3, 4);
let v2 = Vector2::new(1, 2);

// Vector arithmetic
let sum = v1 + v2;
let diff = v1 - v2;
let scaled = v1 * 2;

// Dot product and cross product
let dot = v1.dot(&v2);
let cross = v1.cross(&v2);

// Vector magnitude
let magnitude = v1.norm();
```

## 📖 API Overview

### Core Types

| Type               | Description                          | Module      |
| ------------------ | ------------------------------------ | ----------- |
| `Point<T1, T2>`    | 2D point with x and y coordinates    | `point`     |
| `Vector2<T1, T2>`  | 2D vector with x and y components    | `vector2`   |
| `Interval<T>`      | Range [lb, ub] with interval algebra | `interval`  |
| `Polygon<T>`       | Arbitrary polygon                    | `polygon`   |
| `RPolygon<T>`      | Rectilinear (Manhattan) polygon      | `rpolygon`  |
| `MergeObj<T1, T2>` | Merge object for combining intervals | `merge_obj` |

### Key Operations

#### Point Operations

- Arithmetic: `+`, `-`, `+=`, `-=` with vectors
- Comparison: `==`, `!=`, `<`, `>`
- Distance: `dist_to()`, `dist_to_point()`
- Overlap: `overlaps()`, `contains()`
- Convex hull: `convex_hull()`

#### Vector Operations

- Arithmetic: `+`, `-`, `*`, `/` with scalars
- Dot product: `dot()`
- Cross product: `cross()`
- Norm: `norm()`, `norm_squared()`
- Normalization: `normalize()`

#### Interval Operations

- Overlap: `overlaps()`, `contains()`
- Set operations: `intersect()`, `convex_hull()`
- Properties: `length()`, `is_empty()`

#### Polygon Operations

- Properties: `area()`, `is_convex()`, `orientation()`
- Bounding: `bounding_box()`
- Validation: `is_valid()`, `is_rectilinear()`

## 💡 Use Cases

### 1. Circuit Layout Design

```rust
use physdes::{Point, Polygon, interval::Interval};

// Define circuit component as a rectangle
let component = Polygon::new(&vec![
    Point::new(0, 0),
    Point::new(10, 0),
    Point::new(10, 5),
    Point::new(0, 5),
]);

// Calculate placement constraints
let x_range = Interval::new(0, 100);
let y_range = Interval::new(0, 100);
```

### 2. Floorplanning

```rust
use physdes::RPolygon;

// Create rectilinear polygon for floorplan
let floorplan = RPolygon::from_rectangle(
    Point::new(0, 0),
    Point::new(100, 100),
);

// Check for overlaps with other modules
let module_a = RPolygon::from_rectangle(
    Point::new(10, 10),
    Point::new(30, 40),
);
```

### 3. Wire Routing Analysis

```rust
use physdes::{Point, Vector2};

// Calculate Manhattan distance for routing
let start = Point::new(0, 0);
let end = Point::new(10, 20);
let wire_length = (end.xcoord - start.xcoord).abs() + (end.ycoord - start.ycoord).abs();
```

### 4. Design Rule Checking (DRC)

```rust
use physdes::{Point, interval::Interval};

// Check minimum spacing rules
let component_a = Interval::new(0, 10);
let component_b = Interval::new(15, 25);
let min_spacing = 3;

let spacing = component_b.lb() - component_a.ub();
assert!(spacing >= min_spacing, "Spacing violation!");
```

## ⚡ Performance

The library is designed for performance:

- **Generic over numeric types**: Choose between `i32`, `i64`, `f32`, `f64` based on your needs
- **Efficient algorithms**: Optimized for VLSI design workloads
- **No allocations**: Core operations use stack allocation where possible
- **SIMD-ready**: Structured to enable future SIMD optimizations

### Benchmarks

Run benchmarks with:

```bash
cargo bench
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/luk036/physdes-rs.git
cd physdes-rs

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Run benchmarks
cargo bench
```

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

Built with Rust, designed for VLSI physical design applications.

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## 🔗 Related Projects

### Polyglot Implementations

- [**physdes-cpp**](https://github.com/luk036/physdes-cpp) - C++ version
- [**physdes-py**](https://github.com/luk036/physdes-py) - Python version

### Algorithm Polyglot

- [**algorithm-polyglot**](https://github.com/luk036/algorithm-polyglot) - Meta-repo documenting the polyglot strategy
