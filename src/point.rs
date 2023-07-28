// #![no_std]

use super::Vector2;
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
pub struct Point<T> {
    /// x portion of the Point object
    pub xcoord: T,
    /// y portion of the Point object
    pub ycoord: T,
}

impl<T> Point<T> {
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
    ///
    /// let a = Point::new(3, 4);
    ///
    /// assert_eq!(a.xcoord, 3);
    /// assert_eq!(a.ycoord, 4);
    #[inline]
    pub const fn new(xcoord: T, ycoord: T) -> Self {
        Point { xcoord, ycoord }
    }
}

macro_rules! forward_xf_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T: Clone + Num> $imp<&'b Vector2<T>> for &'a Point<T> {
            type Output = Point<T>;

            #[inline]
            fn $method(self, other: &Vector2<T>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

macro_rules! forward_xf_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<Vector2<T>> for &'a Point<T> {
            type Output = Point<T>;

            #[inline]
            fn $method(self, other: Vector2<T>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<&'a Vector2<T>> for Point<T> {
            type Output = Point<T>;

            #[inline]
            fn $method(self, other: &Vector2<T>) -> Self::Output {
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
impl<T: Clone + Num> Add<Vector2<T>> for Point<T> {
    type Output = Self;

    /// Translate a new Point
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::vector2::Vector2;
    ///
    /// let a = Point::new(3, 4);
    /// let v = Vector2::new(5, 3);
    /// let a2 = a + v;
    ///
    /// assert_eq!(a2.xcoord, 8);
    /// assert_eq!(a2.ycoord, 7);
    #[inline]
    fn add(self, other: Vector2<T>) -> Self::Output {
        Self::Output::new(self.xcoord + other.x_, self.ycoord + other.y_)
    }
}

forward_all_binop!(impl Sub, sub);

// (a, b) - (c, d) == (a - c), (b - d)
impl<T: Clone + Num> Sub<Vector2<T>> for Point<T> {
    type Output = Self;

    /// Translate a new Point
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::vector2::Vector2;
    ///
    /// let a = Point::new(3, 4);
    /// let v = Vector2::new(5, 3);
    /// let a2 = a - v;
    ///
    /// assert_eq!(a2.xcoord, -2);
    /// assert_eq!(a2.ycoord, 1);
    #[inline]
    fn sub(self, other: Vector2<T>) -> Self::Output {
        Self::Output::new(self.xcoord - other.x_, self.ycoord - other.y_)
    }
}

macro_rules! forward_xf_xf_binop2 {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T: Clone + Num> $imp<&'b Point<T>> for &'a Point<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: &Point<T>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

macro_rules! forward_xf_val_binop2 {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<Point<T>> for &'a Point<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: Point<T>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop2 {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<&'a Point<T>> for Point<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: &Point<T>) -> Self::Output {
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

/// Displacement of two Points
///
/// # Examples
///
/// ```
/// use physdes::point::Point;
/// use physdes::vector2::Vector2;
///
/// let a = Point::new(3, 4);
/// let b = Point::new(5, 3);
/// let v = a - b;
///
/// assert_eq!(v.x_, -2);
/// assert_eq!(v.y_, 1);
impl<T: Clone + Num> Sub for Point<T> {
    type Output = Vector2<T>;

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

    impl<T: Clone + NumAssign> AddAssign<Vector2<T>> for Point<T> {
        fn add_assign(&mut self, other: Vector2<T>) {
            self.xcoord += other.x_;
            self.ycoord += other.y_;
        }
    }

    impl<T: Clone + NumAssign> SubAssign<Vector2<T>> for Point<T> {
        fn sub_assign(&mut self, other: Vector2<T>) {
            self.xcoord -= other.x_;
            self.ycoord -= other.y_;
        }
    }

    macro_rules! forward_op_assign {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T: Clone + NumAssign> $imp<&'a Vector2<T>> for Point<T> {
                #[inline]
                fn $method(&mut self, other: &Vector2<T>) {
                    self.$method(other.clone())
                }
            }
        };
    }

    forward_op_assign!(impl AddAssign, add_assign);
    forward_op_assign!(impl SubAssign, sub_assign);
}

impl<T: Clone + Num + Neg<Output = T>> Neg for Point<T> {
    type Output = Self;

    /// Negate a Points
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    ///
    /// let a = Point::new(3, 4);
    /// let b = -a;
    ///
    /// assert_eq!(b.xcoord, -3);
    /// assert_eq!(b.ycoord, -4);
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.xcoord, -self.ycoord)
    }
}

impl<'a, T: Clone + Num + Neg<Output = T>> Neg for &'a Point<T> {
    type Output = Point<T>;

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

    use super::{hash, Point, Vector2};
    use core::i32;

    pub const _0_0p: Point<i32> = Point {
        xcoord: 0,
        ycoord: 0,
    };
    pub const _1_0p: Point<i32> = Point {
        xcoord: 1,
        ycoord: 0,
    };
    pub const _1_1p: Point<i32> = Point {
        xcoord: 1,
        ycoord: 1,
    };
    pub const _0_1p: Point<i32> = Point {
        xcoord: 0,
        ycoord: 1,
    };
    pub const _neg1_1p: Point<i32> = Point {
        xcoord: -1,
        ycoord: 1,
    };
    // pub const all_consts: [Point<i32>; 4] = [_0_0p, _1_0p, _1_1p, _neg1_1p];
    pub const _4_2p: Point<i32> = Point {
        xcoord: 4,
        ycoord: 2,
    };

    #[test]
    fn test_consts() {
        // check our constants are what Point::new creates
        fn test(c: Point<i32>, r: i32, i: i32) {
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
    fn test_add() {
        let a = Point::new(0i32, 0i32);
        let b = Point::new(1i32, 0i32);
        let v = Vector2::new(5i32, 6i32);
        assert_eq!(a, a + v - v);
        assert_eq!(b, b - v + v);
    }
}
