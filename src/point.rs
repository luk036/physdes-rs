// #![no_std]

use super::Vector2;
use crate::generic::{Contain, MinDist, Overlap};
#[cfg(any(test, feature = "std"))]
#[cfg(test)]
use core::hash;
use core::ops::{Add, Neg, Sub};
use num_traits::Num;

/// The code defines a generic Point struct with x and y coordinates.
///
/// Properties:
///
/// * `xcoord`: The `xcoord` property represents the x-coordinate of a point in a two-dimensional space.
/// It is a generic type `T`, which means it can be any type that implements the necessary traits for
/// the `Point` struct.
/// * `ycoord`: The `ycoord` property represents the y-coordinate of a point in a two-dimensional space.
/// It is a generic type `T`, which means it can be any type that implements the necessary traits for
/// the `Point` struct.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Point<T1, T2> {
    /// x portion of the Point object
    pub xcoord: T1,
    /// y portion of the Point object
    pub ycoord: T2,
}

impl<T1, T2> Point<T1, T2> {
    /// The `new` function creates a new `Point` with the given x and y coordinates.
    ///
    /// Arguments:
    ///
    /// * `xcoord`: The `xcoord` parameter represents the x-coordinate of the point. It is of type `T`,
    /// which means it can be any type that implements the necessary traits for mathematical operations.
    /// * `ycoord`: The `ycoord` parameter represents the y-coordinate of the point. It is used to
    /// specify the vertical position of the point in a two-dimensional coordinate system.
    ///
    /// Returns:
    ///
    /// The `new` function returns a new instance of the `Point` struct with the specified `xcoord` and
    /// `ycoord` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// assert_eq!(Point::new(3, 4).xcoord, 3);
    /// assert_eq!(Point::new(3, 4).ycoord, 4);
    /// ```
    #[inline]
    pub const fn new(xcoord: T1, ycoord: T2) -> Self {
        Point { xcoord, ycoord }
    }

    // pub fn flip(&self) -> Point<T2, T1> {
    //     Point {
    //         xcoord: self.ycoord,
    //         ycoord: self.xcoord,
    //     }
    // }
}

// impl<T, U> Overlap<U> for Point<T1, T2>
// where
//     U: Overlap<T1, T2>,
// {
//     fn overlaps(&self, other: &U) -> bool {
//         other.overlaps(&self.xcoord) && other.overlaps(&self.ycoord)
//     }
// }

// impl<T, U> Contain<U> for Point<T1, T2>
// where
//     T: Contain<U>,
// {
//     fn contains(&self, other: &U) -> bool {
//         self.xcoord.contains(other) && self.ycoord.contains(other)
//     }
// }

impl<T1, T2, U1, U2> Overlap<Point<U1, U2>> for Point<T1, T2>
where
    T1: Overlap<U1>,
    T2: Overlap<U2>,
{
    fn overlaps(&self, other: &Point<U1, U2>) -> bool {
        self.xcoord.overlaps(&other.xcoord) && self.ycoord.overlaps(&other.ycoord)
    }
}

impl<T1, T2, U1, U2> Contain<Point<U1, U2>> for Point<T1, T2>
where
    T1: Contain<U1>,
    T2: Contain<U2>,
{
    fn contains(&self, other: &Point<U1, U2>) -> bool {
        self.xcoord.contains(&other.xcoord) && self.ycoord.contains(&other.ycoord)
    }
}

impl<T1, T2, U1, U2> MinDist<Point<U1, U2>> for Point<T1, T2>
where
    T1: MinDist<U1>,
    T2: MinDist<U2>,
{
    fn min_dist(&self, other: &Point<U1, U2>) -> u32 {
        self.xcoord.min_dist(&other.xcoord) + self.ycoord.min_dist(&other.ycoord)
    }
}

macro_rules! forward_xf_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T1: Clone + Num, T2: Clone + Num> $imp<&'b Vector2<T1, T2>>
            for &'a Point<T1, T2>
        {
            type Output = Point<T1, T2>;

            #[inline]
            fn $method(self, other: &Vector2<T1, T2>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

macro_rules! forward_xf_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<Vector2<T1, T2>> for &'a Point<T1, T2> {
            type Output = Point<T1, T2>;

            #[inline]
            fn $method(self, other: Vector2<T1, T2>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<&'a Vector2<T1, T2>> for Point<T1, T2> {
            type Output = Point<T1, T2>;

            #[inline]
            fn $method(self, other: &Vector2<T1, T2>) -> Self::Output {
                self.$method(other.clone())
            }
        }
    };
}

macro_rules! forward_all_binop {
    (impl $imp:ident, $method:ident) => {
        forward_xf_xf_binop!(impl $imp, $method);
        forward_xf_val_binop!(impl $imp, $method);
        forward_val_xf_binop!(impl $imp, $method);
    };
}

// arithmetic
forward_all_binop!(impl Add, add);

// (a, b) + (c, d) == (a + c), (b + d)
impl<T1: Clone + Num, T2: Clone + Num> Add<Vector2<T1, T2>> for Point<T1, T2> {
    type Output = Self;

    /// Translate a new Point
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Point::new(3, 4) + Vector2::new(5, 3), Point::new(8, 7));
    /// assert_eq!(Point::new(3, 4) + Vector2::new(-5, -3), Point::new(-2, 1));
    /// assert_eq!(Point::new(3, 4) + Vector2::new(5, -3), Point::new(8, 1));
    /// assert_eq!(Point::new(3, 4) + Vector2::new(-5, 3), Point::new(-2, 7));
    /// assert_eq!(Point::new(3, 4) + Vector2::new(0, 0), Point::new(3, 4));
    /// assert_eq!(Point::new(3, 4) + Vector2::new(0, 5), Point::new(3, 9));
    /// ```
    #[inline]
    fn add(self, other: Vector2<T1, T2>) -> Self::Output {
        Self::Output::new(self.xcoord + other.x_, self.ycoord + other.y_)
    }
}

forward_all_binop!(impl Sub, sub);

// (a, b) - (c, d) == (a - c), (b - d)
impl<T1: Clone + Num, T2: Clone + Num> Sub<Vector2<T1, T2>> for Point<T1, T2> {
    type Output = Self;

    /// Translate a new Point
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::vector2::Vector2;
    /// assert_eq!(Point::new(3, 4) - Vector2::new(5, 3), Point::new(-2, 1));
    /// assert_eq!(Point::new(3, 4) - Vector2::new(-5, -3), Point::new(8, 7));
    /// assert_eq!(Point::new(3, 4) - Vector2::new(5, -3), Point::new(-2, 7));
    /// assert_eq!(Point::new(3, 4) - Vector2::new(-5, 3), Point::new(8, 1));
    /// assert_eq!(Point::new(3, 4) - Vector2::new(0, 0), Point::new(3, 4));
    /// assert_eq!(Point::new(3, 4) - Vector2::new(0, 5), Point::new(3, -1));
    /// assert_eq!(Point::new(3, 4) - Vector2::new(5, 0), Point::new(-2, 4));
    /// ```
    #[inline]
    fn sub(self, other: Vector2<T1, T2>) -> Self::Output {
        Self::Output::new(self.xcoord - other.x_, self.ycoord - other.y_)
    }
}

macro_rules! forward_xf_xf_binop2 {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T1: Clone + Num, T2: Clone + Num> $imp<&'b Point<T1, T2>>
            for &'a Point<T1, T2>
        {
            type Output = Vector2<T1, T2>;

            #[inline]
            fn $method(self, other: &Point<T1, T2>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

macro_rules! forward_xf_val_binop2 {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<Point<T1, T2>> for &'a Point<T1, T2> {
            type Output = Vector2<T1, T2>;

            #[inline]
            fn $method(self, other: Point<T1, T2>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop2 {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<&'a Point<T1, T2>> for Point<T1, T2> {
            type Output = Vector2<T1, T2>;

            #[inline]
            fn $method(self, other: &Point<T1, T2>) -> Self::Output {
                self.$method(other.clone())
            }
        }
    };
}

macro_rules! forward_all_binop2 {
    (impl $imp:ident, $method:ident) => {
        forward_xf_xf_binop2!(impl $imp, $method);
        forward_xf_val_binop2!(impl $imp, $method);
        forward_val_xf_binop2!(impl $imp, $method);
    };
}

// arithmetic
forward_all_binop2!(impl Sub, sub);

impl<T1: Clone + Num, T2: Clone + Num> Sub for Point<T1, T2> {
    type Output = Vector2<T1, T2>;

    /// Displacement of two Points
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is of the same type as `self` and represents the other object
    /// that you want to subtract from `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Point::new(3, 4) - Point::new(5, 3), Vector2::new(-2, 1));
    /// assert_eq!(Point::new(3, 4) - Point::new(-5, -3), Vector2::new(8, 7));
    /// assert_eq!(Point::new(3, 4) - Point::new(5, -3), Vector2::new(-2, 7));
    /// assert_eq!(Point::new(3, 4) - Point::new(-5, 3), Vector2::new(8, 1));
    /// assert_eq!(Point::new(3, 4) - Point::new(0, 0), Vector2::new(3, 4));
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.xcoord - other.xcoord, self.ycoord - other.ycoord)
    }
}

// Op Assign

mod opassign {
    use core::ops::{AddAssign, SubAssign};

    use num_traits::NumAssign;

    use crate::Point;
    use crate::Vector2;

    impl<T1: Clone + NumAssign, T2: Clone + NumAssign> AddAssign<Vector2<T1, T2>> for Point<T1, T2> {
        fn add_assign(&mut self, other: Vector2<T1, T2>) {
            self.xcoord += other.x_;
            self.ycoord += other.y_;
        }
    }

    impl<T1: Clone + NumAssign, T2: Clone + NumAssign> SubAssign<Vector2<T1, T2>> for Point<T1, T2> {
        fn sub_assign(&mut self, other: Vector2<T1, T2>) {
            self.xcoord -= other.x_;
            self.ycoord -= other.y_;
        }
    }

    macro_rules! forward_op_assign {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T1: Clone + NumAssign, T2: Clone + NumAssign> $imp<&'a Vector2<T1, T2>>
                for Point<T1, T2>
            {
                #[inline]
                fn $method(&mut self, other: &Vector2<T1, T2>) {
                    self.$method(other.clone())
                }
            }
        };
    }

    forward_op_assign!(impl AddAssign, add_assign);
    forward_op_assign!(impl SubAssign, sub_assign);
}

impl<T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg for Point<T1, T2> {
    type Output = Self;

    /// Negate a Points
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    ///
    /// assert_eq!(-Point::new(3, 4), Point::new(-3, -4));
    /// assert_eq!(-Point::new(0, 0), Point::new(0, 0));
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.xcoord, -self.ycoord)
    }
}

impl<'a, T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg
    for &'a Point<T1, T2>
{
    type Output = Point<T1, T2>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

#[cfg(test)]
fn hash<T: hash::Hash>(x: &T) -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = <RandomState as BuildHasher>::Hasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {
    #![allow(non_upper_case_globals)]

    use super::*;
    use crate::generic::Overlap;
    use core::i32;

    pub const _0_0p: Point<i32, i32> = Point {
        xcoord: 0,
        ycoord: 0,
    };
    pub const _1_0p: Point<i32, i32> = Point {
        xcoord: 1,
        ycoord: 0,
    };
    pub const _1_1p: Point<i32, i32> = Point {
        xcoord: 1,
        ycoord: 1,
    };
    pub const _0_1p: Point<i32, i32> = Point {
        xcoord: 0,
        ycoord: 1,
    };
    pub const _neg1_1p: Point<i32, i32> = Point {
        xcoord: -1,
        ycoord: 1,
    };
    // pub const all_consts: [Point<i32, i32>; 4] = [_0_0p, _1_0p, _1_1p, _neg1_1p];
    pub const _4_2p: Point<i32, i32> = Point {
        xcoord: 4,
        ycoord: 2,
    };

    #[test]
    fn test_consts() {
        // check our constants are what Point::new creates
        fn test(c: Point<i32, i32>, r: i32, i: i32) {
            assert_eq!(c, Point::new(r, i));
        }
        test(_0_0p, 0, 0);
        test(_1_0p, 1, 0);
        test(_1_1p, 1, 1);
        test(_neg1_1p, -1, 1);
    }

    #[test]
    fn test_hash() {
        let a = Point::new(0i32, 0i32);
        let b = Point::new(1i32, 0i32);
        let c = Point::new(0i32, 1i32);
        assert!(hash(&a) != hash(&b));
        assert!(hash(&b) != hash(&c));
        assert!(hash(&c) != hash(&a));
    }

    #[test]
    fn test_overlap() {
        let a = Point::new(0i32, 0i32);
        let b = Point::new(1i32, 0i32);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_contain() {
        let a = Point::new(0i32, 0i32);
        let b = Point::new(1i32, 0i32);
        assert!(!a.contains(&b));
    }

    #[test]
    fn test_min_dist() {
        let a = Point::new(3i32, 5i32);
        let b = Point::new(6i32, 4i32);
        assert_eq!(a.min_dist(&b), 4);
    }

    #[test]
    fn test_add() {
        let a = Point::new(0i32, 0i32);
        let b = Point::new(1i32, 0i32);
        let v = Vector2::new(5i32, 6i32);
        assert_eq!(a, a + v - v);
        assert_eq!(b, b - v + v);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(0i32, 0i32);
        let b = Point::new(1i32, 0i32);
        let v = Vector2::new(5i32, 6i32);
        assert_eq!(a, a - v + v);
        assert_eq!(b, b + v - v);
    }

    #[test]
    fn test_neg() {
        let a = Point::new(0i32, 0i32);
        let b = Point::new(1i32, 0i32);
        let c = Point::new(0i32, 1i32);
        assert_eq!(a, -(-a));
        assert_eq!(b, -(-b));
        assert_eq!(c, -(-c));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Point::new(1i32, 0i32);
        let b = Point::new(6i32, 6i32);
        let v = Vector2::new(5i32, 6i32);
        a += v;
        assert_eq!(a, b);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Point::new(1i32, 0i32);
        let b = Point::new(-4i32, -6i32);
        let v = Vector2::new(5i32, 6i32);
        a -= v;
        assert_eq!(a, b);
    }

    #[test]
    fn test_neg_assign() {
        let mut a = Point::new(1i32, 0i32);
        let b = Point::new(-1i32, 0i32);
        let c = Point::new(1i32, 0i32);
        a = -a;
        assert_eq!(a, b);
        a = -a;
        assert_eq!(a, c);
    }
}
