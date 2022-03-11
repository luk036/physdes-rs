// #![no_std]

#[cfg(any(test, feature = "std"))]
// #[cfg_attr(test, macro_use)]
// extern crate std;

// use core::fmt;
#[cfg(test)]
use core::hash;
// use core::iter::{Product, Sum};
use core::ops::{Add, Neg, Sub};

// use core::str::FromStr;
#[cfg(feature = "std")]
use std::error::Error;

// extern crate gcollections;
extern crate interval;

use interval::ops::*;
use interval::Interval;
// use gcollections::ops::*;

use super::Vector2;

use num_traits::{Num, Zero};
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug)]
// #[repr(C)]
pub struct Rect<T> {
    /// Real portion of the vector2 object
    pub x_: Interval<T>,
    /// Imaginary portion of the vector2 object
    pub y_: Interval<T>,
}

impl<T> Rect<T> {
    /// Create a new Rect
    #[inline]
    pub const fn new(x_: Interval<T>, y_: Interval<T>) -> Self {
        Rect { x_, y_ }
    }
}

macro_rules! forward_xf_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T: Clone + Num> $imp<&'b Vector2<T>> for &'a Rect<T> {
            type Output = Rect<T>;

            #[inline]
            fn $method(self, other: &Vector2<T>) -> Self::Output {
                self.clone().$method(&other)
            }
        }
    };
}

macro_rules! forward_xf_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<Vector2<T>> for &'a Rect<T> {
            type Output = Rect<T>;

            #[inline]
            fn $method(self, other: Vector2<T>) -> Self::Output {
                self.clone().$method(&other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<&'a Vector2<T>> for Rect<T> {
            type Output = Rect<T>;

            #[inline]
            fn $method(self, other: &Vector2<T>) -> Self::Output {
                self.$method(other)
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
impl<T: Clone + Num + Width + std::ops::Add<Output = T>> Add<Vector2<T>> for Rect<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Vector2<T>) -> Self::Output {
        Self::Output::new(self.x_ + other.x_, self.y_ + other.y_)
    }
}

forward_all_binop!(impl Sub, sub);

// (a, b) - (c, d) == (a - c), (b - d)
impl<T: Clone + Num + Width + std::ops::Sub<Output = T>> Sub<Vector2<T>> for Rect<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Vector2<T>) -> Self::Output {
        Self::Output::new(self.x_ - other.x_, self.y_ - other.y_)
    }
}

// Op Assign

/*
mod opassign {
    use core::ops::{AddAssign, SubAssign};

    use num_traits::NumAssign;

    use crate::Rect;
    use crate::Vector2;

    impl<T: Clone + NumAssign> AddAssign<Vector2<T>> for Rect<T> {
        fn add_assign(&mut self, other: Vector2<T>) {
            self.x_ += other.x_;
            self.y_ += other.y_;
        }
    }

    impl<T: Clone + NumAssign> SubAssign<Vector2<T>> for Rect<T> {
        fn sub_assign(&mut self, other: Vector2<T>) {
            self.x_ -= other.x_;
            self.y_ -= other.y_;
        }
    }

    macro_rules! forward_op_assign {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T: Clone + NumAssign> $imp<&'a Vector2<T>> for Rect<T> {
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

impl<T: Clone + Num + Neg<Output = T>> Neg for Rect<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x_, -self.y_)
    }
}

impl<'a, T: Clone + Num + Neg<Output = T>> Neg for &'a Rect<T> {
    type Output = Rect<T>;

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
*/

#[cfg(test)]
mod test {
    #![allow(non_upper_case_globals)]

    use super::{Rect, Vector2};
    use core::i32;

    pub const _0_0p: Rect<i32> = Rect { x_: 0, y_: 0 };
    pub const _1_0p: Rect<i32> = Rect { x_: 1, y_: 0 };
    pub const _1_1p: Rect<i32> = Rect { x_: 1, y_: 1 };
    pub const _0_1p: Rect<i32> = Rect { x_: 0, y_: 1 };
    pub const _neg1_1p: Rect<i32> = Rect { x_: -1, y_: 1 };
    pub const all_consts: [Rect<i32>; 4] = [_0_0p, _1_0p, _1_1p, _neg1_1p];
    pub const _4_2p: Rect<i32> = Rect { x_: 4, y_: 2 };

    #[test]
    fn test_consts() {
        // check our constants are what Rect::new creates
        fn test(c: Rect<i32>, r: i32, i: i32) {
            assert_eq!(c, Rect::new(r, i));
        }
        test(_0_0p, 0, 0);
        test(_1_0p, 1, 0);
        test(_1_1p, 1, 1);
        test(_neg1_1p, -1, 1);
    }

    /*
    #[test]
    fn test_hash() {
        let a = Rect::new(0i32, 0i32);
        let b = Rect::new(1i32, 0i32);
        let c = Rect::new(0i32, 1i32);
        assert!(hash(&a) != hash(&b));
        assert!(hash(&b) != hash(&c));
        assert!(hash(&c) != hash(&a));
    }
    */
    #[test]
    fn test_add() {
        let a = Rect::new(0i32, 0i32);
        let b = Rect::new(1i32, 0i32);
        let v = Vector2::new(5i32, 6i32);
        assert_eq!(a, a + v - v);
        assert_eq!(b, b - v + v);
    }
}
