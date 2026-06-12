use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use num_traits::{Num, Signed, Zero};

/// A 2D vector with x and y components.
///
/// ```svgbob
///        y
///        ^
///        |
///        |
///        *-----> x
///       /|
///      / | y_
///     /  |
///    /   |
///   *----+-----> x
///  (0,0) x_
/// ```
///
/// # Examples
///
/// ```
/// use physdes::vector2::Vector2;
///
/// let v = Vector2::new(3, 4);
/// assert_eq!(v.x_, 3);
/// assert_eq!(v.y_, 4);
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
// #[repr(C)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector2<T1, T2> {
    /// x portion of the Vector2 object
    pub x_: T1,
    /// y portion of the Vector2 object
    pub y_: T2,
}

impl<T1, T2> Vector2<T1, T2> {
    /// Creates a new Vector2 with the given x and y values.
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
    /// Computes the dot product of two vectors.
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

    /// Computes the cross product (2D scalar) of two vectors.
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

    /// Multiplies the vector by a scalar factor.
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
    pub fn scale(&self, factor: T1) -> Self {
        Self::new(self.x_.clone() * factor.clone(), self.y_.clone() * factor)
    }

    /// Divides the vector by a scalar factor.
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
    pub fn unscale(&self, factor: T1) -> Self {
        Self::new(self.x_.clone() / factor.clone(), self.y_.clone() / factor)
    }
}

impl<T1: Clone + Signed> Vector2<T1, T1> {
    /// Computes the L1 norm (Manhattan distance from origin): `|x_| + |y_|`.
    ///
    /// [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
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
    /// Computes the Chebyshev (infinity) norm: `max(x_, y_)`.
    ///
    /// Assumes non-negative coordinate values (does not take absolute values internally).
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

// (a, b) + (c, d) == (a + c), (b + d)
impl<T1: Clone + Num, T2: Clone + Num> Add<Vector2<T1, T2>> for Vector2<T1, T2> {
    type Output = Self;

    /// Adds two vectors component-wise.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2) + Vector2::new(3, 4), Vector2::new(4, 6));
    /// ```
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.x_ + other.x_, self.y_ + other.y_)
    }
}

// (a, b) - (c, d) == (a - c), (b - d)
impl<T1: Clone + Num, T2: Clone + Num> Sub<Vector2<T1, T2>> for Vector2<T1, T2> {
    type Output = Self;

    /// Subtracts two vectors component-wise.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// assert_eq!(Vector2::new(1, 2) - Vector2::new(3, 4), Vector2::new(-2, -2));
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
        /// Adds another vector to this one component-wise.
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
        /// Subtracts another vector from this one component-wise.
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
        /// Multiplies each component of the vector by a scalar.
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
        /// Divides each component of the vector by a scalar.
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

    /// Negates both components of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use physdes::vector2::Vector2;
    ///
    /// let v = Vector2::new(1, 2);
    /// assert_eq!(-v, Vector2::new(-1, -2));
    /// ```
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x_, -self.y_)
    }
}

impl<T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg
    for &Vector2<T1, T2>
{
    type Output = Vector2<T1, T2>;

    /// Negates both components of a borrowed vector.
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
impl<T1: Clone + Num + Add, T2: Clone + Num + Add> Zero for Vector2<T1, T2> {
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
mod test {
    #![allow(non_upper_case_globals)]

    use super::Vector2;
    use core::f64;
    use num_traits::Zero;
    use std::hash;

    fn hash<T: hash::Hash>(item: &T) -> u64 {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        let mut hasher = <RandomState as BuildHasher>::Hasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }

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
        fn test(vec: Vector2<f64, f64>, x_val: f64, y_val: f64) {
            assert_eq!(vec, Vector2::new(x_val, y_val));
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
        let vec_a = Vector2::new(0i32, 0i32);
        let vec_b = Vector2::new(1i32, 0i32);
        let vec_c = Vector2::new(0i32, 1i32);
        assert!(hash(&vec_a) != hash(&vec_b));
        assert!(hash(&vec_b) != hash(&vec_c));
        assert!(hash(&vec_c) != hash(&vec_a));
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
        let mut vec_a = _0_1v;
        vec_a += _1_0v;
        assert_eq!(vec_a, _1_1v);
    }

    #[test]
    fn test_sub_assign() {
        let mut vec_a = _1_1v;
        vec_a -= _1_1v;
        assert_eq!(vec_a, _0_0v);
    }

    #[test]
    fn test_mul_assign() {
        let mut vec_a = _05_05v;
        vec_a *= 2.0;
        assert_eq!(vec_a, _1_1v);
    }

    #[test]
    fn test_div_assign() {
        let mut vec_a = _1_1v;
        vec_a /= 2.0;
        assert_eq!(vec_a, _05_05v);
    }

    #[test]
    fn test_rem() {
        assert_eq!(_4_2v % 3.0, Vector2::new(1.0, 2.0));
    }

    #[test]
    fn test_sub_more() {
        assert_eq!(_1_1v - _0_1v, _1_0v);
        assert_eq!(_0_1v - _1_0v, Vector2::new(-1.0, 1.0));
    }

    #[test]
    fn test_add_more() {
        assert_eq!(_1_0v + _0_1v, _1_1v);
    }

    #[test]
    fn test_mul_more() {
        assert_eq!(_1_1v * 2.0, Vector2::new(2.0, 2.0));
    }

    #[test]
    fn test_dot_more_cases() {
        assert_eq!(_0_0v.dot(&_1_1v), 0.0);
        assert_eq!(_1_1v.dot(&_0_0v), 0.0);
        assert_eq!(_neg1_1v.dot(&_1_1v), 0.0);
    }

    #[test]
    fn test_cross_more_cases() {
        assert_eq!(_0_0v.cross(&_1_1v), 0.0);
        assert_eq!(_1_1v.cross(&_0_0v), 0.0);
        assert_eq!(_neg1_1v.cross(&_1_1v), -2.0);
    }

    #[test]
    fn test_l1_norm_more_cases() {
        assert_eq!(_0_0v.l1_norm(), 0.0);
        assert_eq!(_neg1_1v.l1_norm(), 2.0);
    }

    #[test]
    fn test_norm_inf_more_cases() {
        assert_eq!(_0_0v.norm_inf(), 0.0);
        assert_eq!(_neg1_1v.norm_inf(), 1.0);
    }

    #[test]
    fn test_scalar_arithmetic_more_cases() {
        assert_eq!(_0_0v * 2.0, _0_0v);
        assert_eq!(_1_1v * 0.0, _0_0v);
        assert_eq!(_1_1v * 1.0, _1_1v);
        assert_eq!(_1_1v * -1.0, -_1_1v);
    }

    #[test]
    fn test_consts_vv() {
        // check our constants are what Vector2::new creates
        fn test(
            vec: Vector2<Vector2<f64, f64>, Vector2<f64, f64>>,
            w_val: f64,
            x_val: f64,
            y_val: f64,
            z_val: f64,
        ) {
            assert_eq!(
                vec,
                Vector2::new(Vector2::new(w_val, x_val), Vector2::new(y_val, z_val))
            );
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

    #[test]
    #[allow(clippy::op_ref, clippy::clone_on_copy)]
    fn test_mul_ref_scalar() {
        let v = Vector2::new(3, 4);
        let s = 2i32;

        assert_eq!(v.clone() * &s, Vector2::new(6, 8));
        assert_eq!(&v * s, Vector2::new(6, 8));
        assert_eq!(&v * &s, Vector2::new(6, 8));
    }

    #[test]
    #[allow(clippy::op_ref, clippy::clone_on_copy)]
    fn test_scalar_mul_vec_ref() {
        let v = Vector2::new(3, 4);
        let s = 2i32;

        assert_eq!(s * &v, Vector2::new(6, 8));
        assert_eq!(&s * v.clone(), Vector2::new(6, 8));
        assert_eq!(&s * &v, Vector2::new(6, 8));
    }
}

#[test]
fn test_neg_ref() {
    let v = &Vector2::new(1, 2);
    let neg_v = -v;
    assert_eq!(neg_v, Vector2::new(-1, -2));
}

#[test]
fn test_ref_add_assign_ref() {
    let mut v1 = Vector2::new(1.0, 2.0);
    let v2 = &Vector2::new(3.0, 4.0);
    v1 += v2;
    assert_eq!(v1, Vector2::new(4.0, 6.0));
}

#[test]
fn test_ref_sub_assign_ref() {
    let mut v1 = Vector2::new(5.0, 7.0);
    let v2 = &Vector2::new(2.0, 3.0);
    v1 -= v2;
    assert_eq!(v1, Vector2::new(3.0, 4.0));
}

#[test]
fn test_ref_mul_assign_ref() {
    let mut v = Vector2::new(2.0, 3.0);
    let scalar = &2.0;
    v *= scalar;
    assert_eq!(v, Vector2::new(4.0, 6.0));
}

#[test]
fn test_ref_div_assign_ref() {
    let mut v = Vector2::new(6.0, 8.0);
    let scalar = &2.0;
    v /= scalar;
    assert_eq!(v, Vector2::new(3.0, 4.0));
}

#[test]
fn test_mul_integer_types() {
    let v = Vector2::new(2i32, 3i32);
    assert_eq!(v * 2i32, Vector2::new(4, 6));

    let v2 = Vector2::new(2u8, 3u8);
    assert_eq!(v2 * 2u8, Vector2::new(4, 6));
}

#[test]
fn test_neg_edge_cases() {
    let v_zero = Vector2::new(0.0, 0.0);
    assert_eq!(-v_zero, Vector2::new(-0.0, -0.0));

    let v_neg = Vector2::new(-1.0, -2.0);
    assert_eq!(-v_neg, Vector2::new(1.0, 2.0));
}

#[test]
fn test_scalar_mul_i32_ref() {
    let v = Vector2::new(2i32, 3i32);
    let result = v * 2i32;
    assert_eq!(result, Vector2::new(4, 6));
}
