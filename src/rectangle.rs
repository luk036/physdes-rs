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

