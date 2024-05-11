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
// #[cfg(feature = "std")]
// use std::error::Error;

use num_traits::{Num, Signed, Zero};

/// The code defines a generic struct called Vector2 with two fields, x_ and y_.
///
/// Properties:
///
/// * `x_`: The `x_` property represents the x-coordinate of the Vector2 object. It is of type `T`,
/// which means it can be any type specified when creating an instance of the Vector2 struct.
/// * `y_`: The `y_` property is the y-coordinate of the `Vector2` object. It represents the vertical
/// position of the vector in a 2D coordinate system.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
// #[repr(C)]
pub struct Vector2<T1, T2> {
    /// x portion of the Vector2 object
    pub x_: T1,
    /// y portion of the Vector2 object
    pub y_: T2,
}

impl<T1, T2> Vector2<T1, T2> {
    /// The function `new` creates a new Vector2 with the given x and y values.
    ///
    /// Arguments:
    ///
    /// * `x_`: The parameter `x_` represents the x-coordinate of the Vector2.
    /// * `y_`: The parameter `y_` represents the y-coordinate of the Vector2. It is of type `T`, which
    /// means it can be any type that is specified when the Vector2 is created.
    ///
    /// Returns:
    ///
    /// The `new` function is returning a new instance of the `Vector2` struct with the provided `x_`
    /// and `y_` values.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2), Vector2 { x_: 1, y_: 2 });
    /// assert_eq!(Vector2::new(3, 4), Vector2 { x_: 3, y_: 4 });
    /// ```
    #[inline]
    pub const fn new(x_: T1, y_: T2) -> Self {
        Vector2 { x_, y_ }
    }
}

impl<T1: Clone + Num> Vector2<T1, T1> {
    /// The `dot` function calculates the dot product of two vectors.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is of the same type as `self`, which means it is an instance of
    /// the same struct or class that the `dot` method is defined in.
    ///
    /// Returns:
    ///
    /// The dot product of two vectors is being returned.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2).dot(&Vector2::new(3, 4)), 11);
    /// assert_eq!(Vector2::new(3, 4).dot(&Vector2::new(1, 2)), 11);
    /// ```
    #[inline]
    pub fn dot(&self, other: &Self) -> T1 {
        self.x_.clone() * other.x_.clone() + self.y_.clone() * other.y_.clone()
    }

    /// The `cross` function calculates the cross product of two vectors.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is of type `Self`, which means it is the same type as the
    /// current object.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2).cross(&Vector2::new(3, 4)), -2);
    /// assert_eq!(Vector2::new(3, 4).cross(&Vector2::new(1, 2)), 2);
    /// ```
    #[inline]
    pub fn cross(&self, other: &Self) -> T1 {
        self.x_.clone() * other.y_.clone() - self.y_.clone() * other.x_.clone()
    }

    // #[inline]
    // pub fn norm_sqr(&self) -> T {
    //     self.dot(self)
    // }

    /// The `scale` function multiplies the vector by a scalar value.
    ///
    /// Arguments:
    ///
    /// * `t`: The parameter `t` is a scalar value that will be used to multiply each component of
    /// `self`.
    ///
    /// Returns:
    ///
    /// The `scale` method returns a new instance of the same type as `self`.
    /// Multiplies `self` by the scalar `t`.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2).scale(3), Vector2::new(3, 6));
    /// assert_eq!(Vector2::new(3, 4).scale(2), Vector2::new(6, 8));
    /// ```
    #[inline]
    pub fn scale(&self, t: T1) -> Self {
        Self::new(self.x_.clone() * t.clone(), self.y_.clone() * t)
    }

    /// The `unscale` function divides the coordinates of a vector by a scalar value.
    ///
    /// Arguments:
    ///
    /// * `t`: The parameter `t` is a scalar value that is used to divide the `self` object. It is of
    /// type `T`, which is a generic type parameter. The division operation is performed on the `x_` and
    /// `y_` fields of the `self` object.
    ///
    /// Returns:
    ///
    /// The `unscale` method returns a new instance of the same type as `self`.
    /// Divides `self` by the scalar `t`.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(3, 6).unscale(3), Vector2::new(1, 2));
    /// assert_eq!(Vector2::new(6, 8).unscale(2), Vector2::new(3, 4));
    /// ```
    #[inline]
    pub fn unscale(&self, t: T1) -> Self {
        Self::new(self.x_.clone() / t.clone(), self.y_.clone() / t)
    }
}

impl<T1: Clone + Signed> Vector2<T1, T1> {
    /// The `l1_norm` function calculates the Manhattan distance from the origin.
    ///
    /// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    ///
    /// Returns:
    ///
    /// The L1 norm, which is the Manhattan distance from the origin.
    /// Returns the L1 norm `|x_| + |y_|` -- the [Manhattan distance] from the origin.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2).l1_norm(), 3);
    /// assert_eq!(Vector2::new(3, 4).l1_norm(), 7);
    /// ```
    #[inline]
    pub fn l1_norm(&self) -> T1 {
        self.x_.abs() + self.y_.abs()
    }
}

impl<T1: Clone + PartialOrd> Vector2<T1, T1> {
    /// The `norm_inf` function returns the maximum absolute value between `x_` and `y_`.
    ///
    /// Returns:
    ///
    /// The `norm_inf` function returns the maximum value between `|x_|` and `|y_|`.
    /// Returns the infinity norm `max(|x_| + |y_|)`
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2).norm_inf(), 2);
    /// assert_eq!(Vector2::new(3, 4).norm_inf(), 4);
    /// ```
    #[inline]
    pub fn norm_inf(&self) -> T1 {
        if self.x_ > self.y_ {
            self.x_.clone()
        } else {
            self.y_.clone()
        }
    }
}

macro_rules! forward_xf_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T1: Clone + Num, T2: Clone + Num> $imp<&'b Vector2<T1, T2>>
            for &'a Vector2<T1, T2>
        {
            type Output = Vector2<T1, T2>;

            /// The function clones the input arguments and calls the specified method on them.
            ///
            /// Arguments:
            ///
            /// * `other`: A reference to another Vector2 object of the same type as self.
            #[inline]
            fn $method(self, other: &Vector2<T1, T2>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

macro_rules! forward_xf_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<Vector2<T1, T2>> for &'a Vector2<T1, T2> {
            type Output = Vector2<T1, T2>;

            #[inline]
            fn $method(self, other: Vector2<T1, T2>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<&'a Vector2<T1, T2>> for Vector2<T1, T2> {
            type Output = Vector2<T1, T2>;

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
impl<T1: Clone + Num, T2: Clone + Num> Add<Vector2<T1, T2>> for Vector2<T1, T2> {
    type Output = Self;

    /// The function `add` takes two values of the same type and returns their sum.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is of the same type as `self` and represents the other object
    /// that you want to add to `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    /// use std::ops::Add;
    ///
    /// assert_eq!(Vector2::new(1, 2).add(Vector2::new(3, 4)), Vector2::new(4, 6));
    /// assert_eq!(Vector2::new(3, 4).add(Vector2::new(1, 2)), Vector2::new(4, 6));
    /// ```
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.x_ + other.x_, self.y_ + other.y_)
    }
}

forward_all_binop!(impl Sub, sub);

// (a, b) - (c, d) == (a - c), (b - d)
impl<T1: Clone + Num, T2: Clone + Num> Sub<Vector2<T1, T2>> for Vector2<T1, T2> {
    type Output = Self;

    /// The function subtracts the coordinates of two points and returns a new point.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is of the same type as `self` and represents the other value
    /// that you want to subtract from `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    /// use std::ops::Sub;
    ///
    /// assert_eq!(Vector2::new(1, 2).sub(Vector2::new(3, 4)), Vector2::new(-2, -2));
    /// assert_eq!(Vector2::new(3, 4).sub(Vector2::new(1, 2)), Vector2::new(2, 2));
    /// ```
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

    impl<T1: Clone + NumAssign, T2: Clone + NumAssign> AddAssign for Vector2<T1, T2> {
        /// The function `add_assign` adds the values of `other.x_` and `other.y_` to `self.x_` and
        /// `self.y_` respectively.
        ///
        /// Arguments:
        ///
        /// * `other`: The "other" parameter is of type Self, which means it is a reference to another
        /// instance of the same struct or class that the method is defined in. In this case, it
        /// represents another instance of the struct or class that has the same fields or properties as
        /// self.
        ///
        /// # Example
        ///
        /// ```
        /// use physdes::vector2::Vector2;
        /// use std::ops::AddAssign;
        ///
        /// let mut v = Vector2::new(1, 2);
        /// let v2 = Vector2::new(3, 4);
        /// v.add_assign(v2);
        /// assert_eq!(v, Vector2::new(4, 6));
        /// ```
        fn add_assign(&mut self, other: Self) {
            self.x_ += other.x_;
            self.y_ += other.y_;
        }
    }

    impl<T1: Clone + NumAssign, T2: Clone + NumAssign> SubAssign for Vector2<T1, T2> {
        /// The function subtracts the values of another object from the values of the current object.
        ///
        /// Arguments:
        ///
        /// * `other`: The parameter "other" is of type Self, which means it is a reference to another
        /// instance of the same struct or class that the method is defined in. In this case, it is a
        /// reference to another instance of the struct or class that has the same fields as self (x_
        /// and y
        ///
        /// # Example
        ///
        /// ```
        /// use physdes::vector2::Vector2;
        /// use std::ops::SubAssign;
        /// let mut v = Vector2::new(1, 2);
        /// let v2 = Vector2::new(3, 4);
        /// v.sub_assign(v2);
        /// assert_eq!(v, Vector2::new(-2, -2));
        /// ```
        fn sub_assign(&mut self, other: Self) {
            self.x_ -= other.x_;
            self.y_ -= other.y_;
        }
    }

    impl<T1: Clone + NumAssign> MulAssign<T1> for Vector2<T1, T1> {
        /// The function multiplies the values of self.x_ and self.y_ by the value of other.
        ///
        /// Arguments:
        ///
        /// * `other`: The parameter `other` is of type `T`, which means it can be any type that
        /// implements the `Clone` trait.
        ///
        /// # Example
        ///
        /// ```
        /// use physdes::vector2::Vector2;
        /// use std::ops::MulAssign;
        ///
        /// let mut v = Vector2::new(1, 2);
        /// v.mul_assign(3);
        /// assert_eq!(v, Vector2::new(3, 6));
        /// ```
        fn mul_assign(&mut self, other: T1) {
            self.x_ *= other.clone();
            self.y_ *= other;
        }
    }

    impl<T1: Clone + NumAssign> DivAssign<T1> for Vector2<T1, T1> {
        /// The function divides the values of self.x_ and self.y_ by the value of other.
        ///
        /// Arguments:
        ///
        /// * `other`: The parameter `other` is of type `T`, which means it can be any type that
        /// implements the `Clone` trait.
        ///
        /// # Example
        ///
        /// ```
        /// use physdes::vector2::Vector2;
        /// use std::ops::DivAssign;
        ///
        /// let mut v = Vector2::new(3, 6);
        /// v.div_assign(3);
        /// assert_eq!(v, Vector2::new(1, 2));
        /// ```
        fn div_assign(&mut self, other: T1) {
            self.x_ /= other.clone();
            self.y_ /= other;
        }
    }

    macro_rules! forward_op_assign1 {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T1: Clone + NumAssign, T2: Clone + NumAssign> $imp<&'a Vector2<T1, T2>>
                for Vector2<T1, T2>
            {
                #[inline]
                fn $method(&mut self, other: &Self) {
                    self.$method(other.clone())
                }
            }
        };
    }

    macro_rules! forward_op_assign2 {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T1: Clone + NumAssign> $imp<&'a T1> for Vector2<T1, T1> {
                #[inline]
                fn $method(&mut self, other: &T1) {
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

impl<T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg
    for Vector2<T1, T2>
{
    type Output = Self;

    /// The `neg` function returns a new instance of the same type with the negated values of `x_` and
    /// `y_`.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    /// use std::ops::Neg;
    ///
    /// let v = Vector2::new(1, 2);
    /// assert_eq!(-v, Vector2::new(-1, -2));
    /// ```
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x_, -self.y_)
    }
}

impl<'a, T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg
    for &'a Vector2<T1, T2>
{
    type Output = Vector2<T1, T2>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

macro_rules! scalar_arithmetic {
    (@forward $imp:ident::$method:ident for $($scalar:ident),*) => (
        impl<'a, T1: Clone + Num> $imp<&'a T1> for Vector2<T1, T1> {
            type Output = Vector2<T1, T1>;

            #[inline]
            fn $method(self, other: &T1) -> Self::Output {
                self.$method(other.clone())
            }
        }
        impl<'a, T1: Clone + Num> $imp<T1> for &'a Vector2<T1, T1> {
            type Output = Vector2<T1, T1>;

            #[inline]
            fn $method(self, other: T1) -> Self::Output {
                self.clone().$method(other)
            }
        }
        impl<'a, 'b, T1: Clone + Num> $imp<&'a T1> for &'b Vector2<T1, T1> {
            type Output = Vector2<T1, T1>;

            #[inline]
            fn $method(self, other: &T1) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
        $(
            impl<'a> $imp<&'a Vector2<$scalar, $scalar>> for $scalar {
                type Output = Vector2<$scalar, $scalar>;

                #[inline]
                fn $method(self, other: &Vector2<$scalar, $scalar>) -> Vector2<$scalar, $scalar> {
                    self.$method(other.clone())
                }
            }
            impl<'a> $imp<Vector2<$scalar, $scalar>> for &'a $scalar {
                type Output = Vector2<$scalar, $scalar>;

                #[inline]
                fn $method(self, other: Vector2<$scalar, $scalar>) -> Vector2<$scalar, $scalar> {
                    self.clone().$method(other)
                }
            }
            impl<'a, 'b> $imp<&'a Vector2<$scalar, $scalar>> for &'b $scalar {
                type Output = Vector2<$scalar, $scalar>;

                #[inline]
                fn $method(self, other: &Vector2<$scalar, $scalar>) -> Vector2<$scalar, $scalar> {
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
            impl Mul<Vector2<$scalar, $scalar>> for $scalar {
                type Output = Vector2<$scalar, $scalar>;

                #[inline]
                fn mul(self, other: Vector2<$scalar, $scalar>) -> Self::Output {
                    Self::Output::new(self * other.x_, self * other.y_)
                }
            }

        )*
    );
}

impl<T1: Clone + Num> Mul<T1> for Vector2<T1, T1> {
    type Output = Vector2<T1, T1>;

    #[inline]
    fn mul(self, other: T1) -> Self::Output {
        Self::Output::new(self.x_ * other.clone(), self.y_ * other)
    }
}

impl<T1: Clone + Num> Div<T1> for Vector2<T1, T1> {
    type Output = Self;

    #[inline]
    fn div(self, other: T1) -> Self::Output {
        Self::Output::new(self.x_ / other.clone(), self.y_ / other)
    }
}

impl<T1: Clone + Num> Rem<T1> for Vector2<T1, T1> {
    type Output = Vector2<T1, T1>;

    #[inline]
    fn rem(self, other: T1) -> Self::Output {
        Self::Output::new(self.x_ % other.clone(), self.y_ % other)
    }
}

scalar_arithmetic!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

// constants
impl<T1: Clone + Num, T2: Clone + Num> Zero for Vector2<T1, T2> {
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

    pub const _0_0v: Vector2<f64, f64> = Vector2 { x_: 0.0, y_: 0.0 };
    pub const _1_0v: Vector2<f64, f64> = Vector2 { x_: 1.0, y_: 0.0 };
    pub const _1_1v: Vector2<f64, f64> = Vector2 { x_: 1.0, y_: 1.0 };
    pub const _0_1v: Vector2<f64, f64> = Vector2 { x_: 0.0, y_: 1.0 };
    pub const _neg1_1v: Vector2<f64, f64> = Vector2 { x_: -1.0, y_: 1.0 };
    pub const _05_05v: Vector2<f64, f64> = Vector2 { x_: 0.5, y_: 0.5 };
    pub const all_consts: [Vector2<f64, f64>; 5] = [_0_0v, _1_0v, _1_1v, _neg1_1v, _05_05v];
    pub const _4_2v: Vector2<f64, f64> = Vector2 { x_: 4.0, y_: 2.0 };

    pub const _0_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _0_0v,
        y_: _0_0v,
    };

    // vector of vectors
    pub const _0_0_0_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _0_0v,
        y_: _0_0v,
    };
    pub const _1_0_0_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _1_0v,
        y_: _0_0v,
    };
    pub const _1_1_0_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _1_1v,
        y_: _0_0v,
    };
    pub const _0_1_0_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _0_1v,
        y_: _0_0v,
    };
    pub const _neg1_1_0_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _neg1_1v,
        y_: _0_0v,
    };
    pub const _05_05_0_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _05_05v,
        y_: _0_0v,
    };
    pub const _0_0_1_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _0_0v,
        y_: _1_0v,
    };
    pub const _1_0_1_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _1_0v,
        y_: _1_0v,
    };
    pub const _1_1_1_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _1_1v,
        y_: _1_0v,
    };
    pub const _0_1_1_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _0_1v,
        y_: _1_0v,
    };
    pub const _neg1_1_1_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _neg1_1v,
        y_: _1_0v,
    };
    pub const _05_05_1_0vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _05_05v,
        y_: _1_0v,
    };
    pub const _0_0_0_1vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _0_0v,
        y_: _0_1v,
    };
    pub const _1_0_0_1vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _1_0v,
        y_: _0_1v,
    };
    pub const _1_1_0_1vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _1_1v,
        y_: _0_1v,
    };
    pub const _0_1_0_1vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _0_1v,
        y_: _0_1v,
    };
    pub const _neg1_1_0_1vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _neg1_1v,
        y_: _0_1v,
    };
    pub const _05_05_0_1vv: Vector2<Vector2<f64, f64>, Vector2<f64, f64>> = Vector2 {
        x_: _05_05v,
        y_: _0_1v,
    };

    #[test]
    fn test_consts() {
        // check our constants are what Vector2::new creates
        fn test(c: Vector2<f64, f64>, r: f64, i: f64) {
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

    #[test]
    fn test_zero() {
        assert_eq!(_0_0v, Vector2::zero());
    }

    #[test]
    fn test_is_zero() {
        assert!(Vector2::<i32, i32>::zero().is_zero());
        assert!(!_1_1v.is_zero());
    }

    #[test]
    fn test_set_zero() {
        let mut v = _1_1v;
        v.set_zero();
        assert!(v.is_zero());
    }

    #[test]
    fn test_neg() {
        assert_eq!(-(-_1_1v), _1_1v);
    }

    #[test]
    fn test_scalar_arithmetic() {
        assert_eq!(_1_1v * 0.5, _05_05v);
        assert_eq!(_1_1v / 2.0, _05_05v);
        assert_eq!(_4_2v % 2.0, _0_0v);
        assert_eq!(0.5 * _1_1v, _05_05v);
    }

    #[test]
    fn test_scalar_arithmetic_ref() {
        assert_eq!(_1_1v * 0.5, _05_05v);
        assert_eq!(0.5 * _1_1v, _05_05v);
    }

    #[test]
    fn test_dot() {
        assert_eq!(_1_1v.dot(&_1_1v), 2.0);
        assert_eq!(_1_1v.dot(&_neg1_1v), 0.0);
        assert_eq!(_1_1v.dot(&_0_1v), 1.0);
    }

    #[test]
    fn test_cross() {
        assert_eq!(_1_1v.cross(&_1_1v), 0.0);
        assert_eq!(_1_1v.cross(&_neg1_1v), 2.0);
        assert_eq!(_1_1v.cross(&_0_1v), 1.0);
    }

    // #[test]
    // fn test_norm_sqr() {
    //     assert_eq!(_1_1v.norm_sqr(), 2.0);
    //     assert_eq!(_0_1v.norm_sqr(), 1.0);
    //     assert_eq!(_neg1_1v.norm_sqr(), 2.0);
    //     assert_eq!(_05_05v.norm_sqr(), 0.5);
    //     assert_eq!(_1_0v.norm_sqr(), 1.0);
    //     assert_eq!(_0_0v.norm_sqr(), 0.0);
    //     assert_eq!(_4_2v.norm_sqr(), 20.0);
    // }

    #[test]
    fn test_l1_norm() {
        assert_eq!(_1_1v.l1_norm(), 2.0);
        assert_eq!(_0_1v.l1_norm(), 1.0);
        assert_eq!(_neg1_1v.l1_norm(), 2.0);
        assert_eq!(_05_05v.l1_norm(), 1.0);
        assert_eq!(_1_0v.l1_norm(), 1.0);
        assert_eq!(_0_0v.l1_norm(), 0.0);
        assert_eq!(_4_2v.l1_norm(), 6.0);
    }

    #[test]
    fn test_norm_inf() {
        assert_eq!(_1_1v.norm_inf(), 1.0);
        assert_eq!(_0_1v.norm_inf(), 1.0);
        assert_eq!(_neg1_1v.norm_inf(), 1.0);
        assert_eq!(_05_05v.norm_inf(), 0.5);
        assert_eq!(_1_0v.norm_inf(), 1.0);
        assert_eq!(_0_0v.norm_inf(), 0.0);
        assert_eq!(_4_2v.norm_inf(), 4.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = _0_1v;
        a += _1_0v;
        assert_eq!(a, _1_1v);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = _1_1v;
        a -= _1_1v;
        assert_eq!(a, _0_0v);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = _05_05v;
        a *= 2.0;
        assert_eq!(a, _1_1v);
    }

    #[test]
    fn test_div_assign() {
        let mut a = _1_1v;
        a /= 2.0;
        assert_eq!(a, _05_05v);
    }

    #[test]
    fn test_consts_vv() {
        // check our constants are what Vector2::new creates
        fn test(c: Vector2<Vector2<f64, f64>, Vector2<f64, f64>>, w: f64, x: f64, y: f64, z: f64) {
            assert_eq!(c, Vector2::new(Vector2::new(w, x), Vector2::new(y, z)));
        }

        test(_0_0vv, 0.0, 0.0, 0.0, 0.0);
        test(_0_0_0_0vv, 0.0, 0.0, 0.0, 0.0);
        test(_1_0_0_0vv, 1.0, 0.0, 0.0, 0.0);
        test(_1_1_0_0vv, 1.0, 1.0, 0.0, 0.0);
        test(_0_1_0_0vv, 0.0, 1.0, 0.0, 0.0);
        test(_neg1_1_0_0vv, -1.0, 1.0, 0.0, 0.0);
        test(_05_05_0_0vv, 0.5, 0.5, 0.0, 0.0);
        test(_0_0_1_0vv, 0.0, 0.0, 1.0, 0.0);
        test(_1_0_1_0vv, 1.0, 0.0, 1.0, 0.0);
        test(_1_1_1_0vv, 1.0, 1.0, 1.0, 0.0);
        test(_0_1_1_0vv, 0.0, 1.0, 1.0, 0.0);
        test(_neg1_1_1_0vv, -1.0, 1.0, 1.0, 0.0);
        test(_05_05_1_0vv, 0.5, 0.5, 1.0, 0.0);
    }

    // #[test]
    // fn test_scale_unscale_vv() {
    //     assert_eq!(_05_05_0_0vv.scale(2.0), _1_1_0_0vv);
    //     assert_eq!(_1_1_0_0vv.unscale(2.0), _05_05_0_0vv);
    //     for &c in all_consts_vv.iter() {
    //         assert_eq!(c.scale(2.0).unscale(2.0), c);
    //     }
    // }
}
