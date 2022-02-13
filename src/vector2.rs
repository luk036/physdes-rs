// #![no_std]

#[cfg(any(test, feature = "std"))]
// #[cfg_attr(test, macro_use)]
// extern crate std;

// use core::fmt;
#[cfg(test)]
use core::hash;
// use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

// use core::str::FromStr;
#[cfg(feature = "std")]
use std::error::Error;

use num_traits::{Num, Signed, Zero};

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
// #[repr(C)]
pub struct Vector2<T> {
    /// Real portion of the vector2 object
    pub x_: T,
    /// Imaginary portion of the vector2 object
    pub y_: T,
}

impl<T> Vector2<T> {
    /// Create a new Vector2
    #[inline]
    pub const fn new(x_: T, y_: T) -> Self {
        Vector2 { x_, y_ }
    }
}

impl<T: Clone + Num> Vector2<T> {
    /// Returns the dot product
    #[inline]
    pub fn dot(&self, other: &Self) -> T {
        self.x_.clone() * other.x_.clone() + self.y_.clone() * other.y_.clone()
    }

    /// Returns the dot product
    #[inline]
    pub fn cross(&self, other: &Self) -> T {
        self.x_.clone() * other.y_.clone() - self.y_.clone() * other.x_.clone()
    }

    #[inline]
    pub fn norm_sqr(&self) -> T {
        self.dot(self)
    }

    /// Multiplies `self` by the scalar `t`.
    #[inline]
    pub fn scale(&self, t: T) -> Self {
        Self::new(self.x_.clone() * t.clone(), self.y_.clone() * t)
    }

    /// Divides `self` by the scalar `t`.
    #[inline]
    pub fn unscale(&self, t: T) -> Self {
        Self::new(self.x_.clone() / t.clone(), self.y_.clone() / t)
    }
}

impl<T: Clone + Signed> Vector2<T> {
    /// Returns the L1 norm `|x_| + |y_|` -- the [Manhattan distance] from the origin.
    ///
    /// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    pub fn l1_norm(&self) -> T {
        self.x_.abs() + self.y_.abs()
    }
}

impl<T: Clone + PartialOrd> Vector2<T> {
    /// Returns the L1 norm `|x_| + |y_|` -- the [Manhattan distance] from the origin.
    ///
    /// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    pub fn norm_inf(&self) -> T {
        if self.x_ > self.y_ {
            self.x_.clone()
        } else {
            self.y_.clone()
        }
    }
}

macro_rules! forward_xf_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T: Clone + Num> $imp<&'b Vector2<T>> for &'a Vector2<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: &Vector2<T>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

macro_rules! forward_xf_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<Vector2<T>> for &'a Vector2<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: Vector2<T>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<&'a Vector2<T>> for Vector2<T> {
            type Output = Vector2<T>;

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
impl<T: Clone + Num> Add<Vector2<T>> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.x_ + other.x_, self.y_ + other.y_)
    }
}

forward_all_binop!(impl Sub, sub);

// (a, b) - (c, d) == (a - c), (b - d)
impl<T: Clone + Num> Sub<Vector2<T>> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.x_ - other.x_, self.y_ - other.y_)
    }
}

// Op Assign

mod opassign {
    use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

    use num_traits::NumAssign;

    use crate::Vector2;

    impl<T: Clone + NumAssign> AddAssign for Vector2<T> {
        fn add_assign(&mut self, other: Self) {
            self.x_ += other.x_;
            self.y_ += other.y_;
        }
    }

    impl<T: Clone + NumAssign> SubAssign for Vector2<T> {
        fn sub_assign(&mut self, other: Self) {
            self.x_ -= other.x_;
            self.y_ -= other.y_;
        }
    }

    impl<T: Clone + NumAssign> MulAssign<T> for Vector2<T> {
        fn mul_assign(&mut self, other: T) {
            self.x_ *= other.clone();
            self.y_ *= other;
        }
    }

    impl<T: Clone + NumAssign> DivAssign<T> for Vector2<T> {
        fn div_assign(&mut self, other: T) {
            self.x_ /= other.clone();
            self.y_ /= other;
        }
    }

    macro_rules! forward_op_assign1 {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T: Clone + NumAssign> $imp<&'a Vector2<T>> for Vector2<T> {
                #[inline]
                fn $method(&mut self, other: &Self) {
                    self.$method(other.clone())
                }
            }
        };
    }

    macro_rules! forward_op_assign2 {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T: Clone + NumAssign> $imp<&'a T> for Vector2<T> {
                #[inline]
                fn $method(&mut self, other: &T) {
                    self.$method(other.clone())
                }
            }
        };
    }

    forward_op_assign1!(impl AddAssign, add_assign);
    forward_op_assign1!(impl SubAssign, sub_assign);
    forward_op_assign2!(impl MulAssign, mul_assign);
    forward_op_assign2!(impl DivAssign, div_assign);
}

impl<T: Clone + Num + Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x_, -self.y_)
    }
}

impl<'a, T: Clone + Num + Neg<Output = T>> Neg for &'a Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

macro_rules! scalar_arithmetic {
    (@forward $imp:ident::$method:ident for $($scalar:ident),*) => (
        impl<'a, T: Clone + Num> $imp<&'a T> for Vector2<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: &T) -> Self::Output {
                self.$method(other.clone())
            }
        }
        impl<'a, T: Clone + Num> $imp<T> for &'a Vector2<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: T) -> Self::Output {
                self.clone().$method(other)
            }
        }
        impl<'a, 'b, T: Clone + Num> $imp<&'a T> for &'b Vector2<T> {
            type Output = Vector2<T>;

            #[inline]
            fn $method(self, other: &T) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
        $(
            impl<'a> $imp<&'a Vector2<$scalar>> for $scalar {
                type Output = Vector2<$scalar>;

                #[inline]
                fn $method(self, other: &Vector2<$scalar>) -> Vector2<$scalar> {
                    self.$method(other.clone())
                }
            }
            impl<'a> $imp<Vector2<$scalar>> for &'a $scalar {
                type Output = Vector2<$scalar>;

                #[inline]
                fn $method(self, other: Vector2<$scalar>) -> Vector2<$scalar> {
                    self.clone().$method(other)
                }
            }
            impl<'a, 'b> $imp<&'a Vector2<$scalar>> for &'b $scalar {
                type Output = Vector2<$scalar>;

                #[inline]
                fn $method(self, other: &Vector2<$scalar>) -> Vector2<$scalar> {
                    self.clone().$method(other.clone())
                }
            }
        )*
    );
    ($($scalar:ident),*) => (
        scalar_arithmetic!(@forward Mul::mul for $($scalar),*);
        // scalar_arithmetic!(@forward Div::div for $($scalar),*);
        // scalar_arithmetic!(@forward Rem::rem for $($scalar),*);

        $(
            impl Mul<Vector2<$scalar>> for $scalar {
                type Output = Vector2<$scalar>;

                #[inline]
                fn mul(self, other: Vector2<$scalar>) -> Self::Output {
                    Self::Output::new(self * other.x_, self * other.y_)
                }
            }

        )*
    );
}

impl<T: Clone + Num> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn mul(self, other: T) -> Self::Output {
        Self::Output::new(self.x_ * other.clone(), self.y_ * other)
    }
}

impl<T: Clone + Num> Div<T> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn div(self, other: T) -> Self::Output {
        Self::Output::new(self.x_ / other.clone(), self.y_ / other)
    }
}

impl<T: Clone + Num> Rem<T> for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn rem(self, other: T) -> Self::Output {
        Self::Output::new(self.x_ % other.clone(), self.y_ % other)
    }
}

scalar_arithmetic!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

// constants
impl<T: Clone + Num> Zero for Vector2<T> {
    #[inline]
    fn zero() -> Self {
        Self::new(Zero::zero(), Zero::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.x_.is_zero() && self.y_.is_zero()
    }

    #[inline]
    fn set_zero(&mut self) {
        self.x_.set_zero();
        self.y_.set_zero();
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

    use super::{hash, Vector2};
    use core::f64;
    use num_traits::Zero;

    pub const _0_0v: Vector2<f64> = Vector2 { x_: 0.0, y_: 0.0 };
    pub const _1_0v: Vector2<f64> = Vector2 { x_: 1.0, y_: 0.0 };
    pub const _1_1v: Vector2<f64> = Vector2 { x_: 1.0, y_: 1.0 };
    pub const _0_1v: Vector2<f64> = Vector2 { x_: 0.0, y_: 1.0 };
    pub const _neg1_1v: Vector2<f64> = Vector2 { x_: -1.0, y_: 1.0 };
    pub const _05_05v: Vector2<f64> = Vector2 { x_: 0.5, y_: 0.5 };
    pub const all_consts: [Vector2<f64>; 5] = [_0_0v, _1_0v, _1_1v, _neg1_1v, _05_05v];
    pub const _4_2v: Vector2<f64> = Vector2 { x_: 4.0, y_: 2.0 };

    #[test]
    fn test_consts() {
        // check our constants are what Vector2::new creates
        fn test(c: Vector2<f64>, r: f64, i: f64) {
            assert_eq!(c, Vector2::new(r, i));
        }
        test(_0_0v, 0.0, 0.0);
        test(_1_0v, 1.0, 0.0);
        test(_1_1v, 1.0, 1.0);
        test(_neg1_1v, -1.0, 1.0);
        test(_05_05v, 0.5, 0.5);
        assert_eq!(_0_0v, Zero::zero());
    }

    #[test]
    fn test_scale_unscale() {
        assert_eq!(_05_05v.scale(2.0), _1_1v);
        assert_eq!(_1_1v.unscale(2.0), _05_05v);
        for &c in all_consts.iter() {
            assert_eq!(c.scale(2.0).unscale(2.0), c);
        }
    }

    #[test]
    fn test_hash() {
        let a = Vector2::new(0i32, 0i32);
        let b = Vector2::new(1i32, 0i32);
        let c = Vector2::new(0i32, 1i32);
        assert!(hash(&a) != hash(&b));
        assert!(hash(&b) != hash(&c));
        assert!(hash(&c) != hash(&a));
    }
}
