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
    extern crate interval;
    use interval::ops::*;
    use interval::Interval;
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

        let x = Interval::<i32>::new(12, 23);
        // let y = Interval::<i32>::new(42, 53);
        println!("{:?}", x);
    }

    #[quickcheck]
    fn check_point(ax: u16, bx: u16) -> bool {
        let a = Point::<i32, i32>::new(ax as i32, 23);
        let b = Vector2::<i32, i32>::new(bx as i32, 45);
        a == (a - b) + b
    }
}
