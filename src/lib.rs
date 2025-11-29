//! # physdes-rs
//!
//! A library for Physical Design in Rust with geometric operations and algorithms.
//!
//! ## Overview
//!
//! ```svgbob
//!    Point (x, y)
//!         *
//!        /|\
//!       / | \
//!      /  |  \
//!     /   |   \
//!    *----*----*
//! Interval [lb, ub]
//!
//!  Vector2 (x, y)
//!      -->
//!     (dx, dy)
//! ```
//!
//! ## Main Components
//!
//! The library provides several geometric structures:
//!
//! - `Point<T1, T2>`: A 2D point with x and y coordinates
//! - `Vector2<T1, T2>`: A 2D vector with x and y components
//! - `Interval<T>`: A range with lower and upper bounds
//! - `Polygon<T>`: An arbitrary polygon
//! - `RPolygon<T>`: A rectilinear polygon
//!
//! # Examples
//!
//! ```
//! use physdes::{Point, Vector2};
//! use physdes::interval::Interval;
//! use physdes::polygon::Polygon as Poly;
//!
//! // Create a point
//! let p = Point::new(3, 4);
//! assert_eq!(p.xcoord, 3);
//! assert_eq!(p.ycoord, 4);
//!
//! // Create a vector
//! let v = Vector2::new(1, 2);
//! assert_eq!(v.x_, 1);
//! assert_eq!(v.y_, 2);
//!
//! // Create an interval
//! let interval = Interval::new(1, 5);
//! assert_eq!(interval.lb(), 1);
//! assert_eq!(interval.ub(), 5);
//!
//! // Create a polygon from points
//! let points = vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(0, 1)];
//! let polygon = Poly::new(&points);
//! assert_eq!(polygon.origin, Point::new(0, 0));
//! ```
//!
//! pub mod halton_int;
pub mod generic;
pub mod interval;
pub mod merge_obj;
pub mod point;
pub mod polygon;
pub mod rpolygon;
pub mod vector2;

pub use crate::point::Point;
pub use crate::polygon::Polygon;
pub use crate::rpolygon::RPolygon;
pub use crate::vector2::Vector2;

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    pub fn it_works() {
        let a = Point::<i32, i32>::new(12, 23);
        let b = Vector2::<i32, i32>::new(34, 45);
        println!("{:?}", a + b);
        println!("{:?}", a - b);

        let mut a = Point::<i32, i32>::new(42, 53);
        a += b;
        a -= b;
        println!("{:?}", -a);

        let c = Point::<i32, i32>::new(12, 23);
        let mm = Point::<Point<i32, i32>, Point<i32, i32>>::new(a, c);
        println!("{:?}", mm);

        let x = interval::Interval::<i32>::new(12, 23);
        // let y = interval::Interval::<i32>::new(42, 53);
        println!("{:?}", x);
    }

    #[quickcheck]
    fn check_point(ax: u16, bx: u16) -> bool {
        let a = Point::<i32, i32>::new(ax as i32, 23);
        let b = Vector2::<i32, i32>::new(bx as i32, 45);
        a == (a - b) + b
    }

    // Additional quickcheck tests to verify build configuration
    #[quickcheck]
    fn check_point_arithmetic_properties(x: i16, y: i16, dx: i16, dy: i16) -> bool {
        let p = Point::<i32, i32>::new(x as i32, y as i32);
        let v = Vector2::<i32, i32>::new(dx as i32, dy as i32);

        // Test associative property: (p + v) - v == p
        let result = (p + v) - v;
        p == result
    }

    #[quickcheck]
    fn check_interval_properties(a: i32, b: i32) -> bool {
        let lower = a.min(b);
        let upper = a.max(b);
        let interval = interval::Interval::<i32>::new(lower, upper);

        // Test that the interval has correct bounds
        interval.lb() <= interval.ub()
    }

    #[test]
    fn test_const_functions() {
        // Test that the const functions we added actually work in const contexts
        const _P1: Point<i32, i32> = Point::new(1, 2);
        const _I1: interval::Interval<i32> = interval::Interval::new(1, 5);
        const _LB: i32 = _I1.lb();
        const _UB: i32 = _I1.ub();
        const _M1: merge_obj::MergeObj<i32, i32> = merge_obj::MergeObj::new(1, 2);
    }
}
