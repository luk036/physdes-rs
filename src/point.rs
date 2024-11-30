// #![no_std]

use super::Vector2;
use crate::generic::{Contain, Displacement, MinDist, Overlap};
use crate::interval::{Hull, Intersect};
// use core::cmp::Ordering;
// #[cfg(any(test, feature = "std"))]
#[cfg(test)]
use core::hash;
use core::ops::{Add, Neg, Sub};
use num_traits::Num;

/// The code defines a generic Point struct with x and y coordinates.
///
/// Properties:
///
/// * `xcoord`: The `xcoord` property represents the x-coordinate of a point in a two-dimensional space.
///             It is a generic type `T`, which means it can be any type that implements the necessary traits for
///             the `Point` struct.
/// * `ycoord`: The `ycoord` property represents the y-coordinate of a point in a two-dimensional space.
///             It is a generic type `T`, which means it can be any type that implements the necessary traits for
///             the `Point` struct.
#[derive(PartialEq, Eq, Copy, PartialOrd, Ord, Clone, Hash, Debug, Default)]
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
    ///             which means it can be any type that implements the necessary traits for mathematical operations.
    /// * `ycoord`: The `ycoord` parameter represents the y-coordinate of the point. It is used to
    ///             specify the vertical position of the point in a two-dimensional coordinate system.
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
}

/// Implements the `Display` trait for the `Point` struct, which allows it to be
/// printed in the format `(x, y)` where `x` and `y` are the coordinates of the point.
///
/// This implementation assumes that the `xcoord` and `ycoord` fields of the `Point`
/// struct implement the `std::fmt::Display` trait, which is enforced by the generic
/// type constraints `T1: std::fmt::Display` and `T2: std::fmt::Display`.
impl<T1: std::fmt::Display, T2: std::fmt::Display> std::fmt::Display for Point<T1, T2> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.xcoord, self.ycoord)
    }
}

/// Flips the coordinates of the `Point` struct, swapping the `xcoord` and `ycoord` fields.
///
/// This is a convenience method that can be used to quickly create a new `Point` with the
/// `xcoord` and `ycoord` fields swapped. It is implemented for `Point` structs where both
/// the `xcoord` and `ycoord` fields implement the `Clone` trait.
///
/// # Examples
///
/// use my_crate::Point;
///
/// let p = Point { xcoord: 1, ycoord: 2 };
/// let flipped = p.flip();
/// assert_eq!(flipped, Point { xcoord: 2, ycoord: 1 });
///
impl<T1: Clone, T2: Clone> Point<T1, T2> {
    #[inline]
    pub fn flip(&self) -> Point<T2, T1> {
        Point {
            xcoord: self.ycoord.clone(),
            ycoord: self.xcoord.clone(),
        }
    }
}

// impl<T1: Ord + Copy, T2: Ord + Copy> PartialOrd for Point<T1, T2> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some((self.xcoord, self.ycoord).cmp(&(other.xcoord, other.ycoord)))
//         // Some(self.xcoord.partial_cmp(&other.xcoord).then(self.ycoord.partial_cmp(&other.ycoord)))
//     }
// }

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

/// Checks if two `Point` instances overlap.
///
/// This implementation checks if the `xcoord` and `ycoord` components of the two `Point` instances
/// overlap, using the `Overlap` trait implementation for their respective component types.
///
/// # Example
///
/// use your_crate::Point;
///
/// let p1 = Point::new(1, 2);
/// let p2 = Point::new(2, 3);
/// assert!(p1.overlaps(&p2));
///
impl<T1, T2, U1, U2> Overlap<Point<U1, U2>> for Point<T1, T2>
where
    T1: Overlap<U1>,
    T2: Overlap<U2>,
{
    /// The `overlaps` function checks if two points overlap in both x and y coordinates.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `overlaps` method is a reference to another `Point`
    ///            struct with generic types `U1` and `U2`.
    ///
    /// Returns:
    ///
    /// The `overlaps` method is returning a boolean value, which indicates whether the x-coordinate of
    /// the current `Point` instance overlaps with the x-coordinate of the `other` `Point` instance, and
    /// whether the y-coordinate of the current `Point` instance overlaps with the y-coordinate of the
    /// `other` `Point` instance. The method returns `true` if both conditions are met, and
    #[inline]
    fn overlaps(&self, other: &Point<U1, U2>) -> bool {
        self.xcoord.overlaps(&other.xcoord) && self.ycoord.overlaps(&other.ycoord)
    }
}

/// Checks if a `Point<T1, T2>` contains a `Point<U1, U2>`.
///
/// This implementation checks if the `xcoord` and `ycoord` fields of the `Point<T1, T2>`
/// contain the corresponding fields of the `Point<U1, U2>`. The `T1` and `T2` types
/// must implement the `Contain` trait for `U1` and `U2` respectively.
impl<T1, T2, U1, U2> Contain<Point<U1, U2>> for Point<T1, T2>
where
    T1: Contain<U1>,
    T2: Contain<U2>,
{
    /// The `contains` function checks if a Point contains another Point by comparing their x and y
    /// coordinates.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to a `Point` struct with generic types `U1` and
    ///             `U2`. It represents another point that you want to check for containment within the current
    ///             `Point` instance.
    ///
    /// Returns:
    ///
    /// The `contains` method is returning a boolean value, which indicates whether the `xcoord` and
    /// `ycoord` of the current `Point` instance contain the `xcoord` and `ycoord` of the `other`
    /// `Point` instance respectively.
    #[inline]
    fn contains(&self, other: &Point<U1, U2>) -> bool {
        self.xcoord.contains(&other.xcoord) && self.ycoord.contains(&other.ycoord)
    }
}

/// The above Rust code is implementing a trait `MinDist` for the `Point` struct. The `MinDist` trait
/// defines a method `min_dist_with` that calculates the minimum distance between two points based on
/// the minimum distance between their individual coordinates (`xcoord` and `ycoord`). The
/// implementation specifies that the minimum distance between two points is calculated by adding the
/// minimum distances between their respective x and y coordinates.
impl<T1, T2, U1, U2> MinDist<Point<U1, U2>> for Point<T1, T2>
where
    T1: MinDist<U1>,
    T2: MinDist<U2>,
{
    /// The function calculates the minimum distance between two points in a two-dimensional space.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to a `Point` struct with generic types `U1` and
    ///             `U2`.
    ///
    /// Returns:
    ///
    /// The `min_dist_with` method is returning the sum of the minimum distances between the
    /// x-coordinate of `self` and the x-coordinate of `other`, and the y-coordinate of `self` and the
    /// y-coordinate of `other`.
    #[inline]
    fn min_dist_with(&self, other: &Point<U1, U2>) -> u32 {
        self.xcoord.min_dist_with(&other.xcoord) + self.ycoord.min_dist_with(&other.ycoord)
    }
}

/// The above Rust code is implementing a `Displacement` trait for the `Point` struct. The
/// `Displacement` trait is generic over two types `T1` and `T2`, and it requires that `T1` and `T2`
/// implement the `Displacement` trait with an associated type `Output`.
impl<T1, T2> Displacement<Point<T1, T2>> for Point<T1, T2>
where
    T1: Displacement<T1, Output = T1>,
    T2: Displacement<T2, Output = T2>,
{
    type Output = Vector2<T1, T2>;

    /// The `displace` function calculates the displacement between two points in Rust.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `displace` method is a reference to another `Point`
    ///             object with the same generic types `T1` and `T2` as the current `Point` object.
    #[inline]
    fn displace(&self, other: &Point<T1, T2>) -> Self::Output {
        Self::Output::new(
            self.xcoord.displace(&other.xcoord),
            self.ycoord.displace(&other.ycoord),
        )
    }
}

// impl<T1, T2> Hull<Point<T1, T2>> for Point<T1, T2>
// where
//     T1: Hull<T1, Output=Interval<T1>>,
//     T2: Hull<T2, Output=Interval<T2>>,
// {
//     type Output = Point<Interval<T1>, Interval<T2>>;
//     fn hull_with(&self, other: &Point<T1, T2>) -> Self::Output {
//         Self::Output::new(
//             self.xcoord.hull_with(&other.xcoord),
//             self.ycoord.hull_with(&other.ycoord),
//         )
//     }
// }

/// The above code is implementing a trait called `Hull` for the `Point` struct in Rust. The `Hull`
/// trait defines a method `hull_with` that calculates the hull (convex hull, for example) of two
/// points. The implementation specifies that the output type of the hull operation on two `Point`
/// instances is a new `Point` with the hull operation applied to the x and y coordinates of the points.
/// The implementation also specifies that the hull operation is applied to the generic types `T1` and
/// `T2` where `T1` and `T
impl<T1, T2> Hull<Point<T1, T2>> for Point<T1, T2>
where
    T1: Hull<T1>,
    T2: Hull<T2>,
{
    type Output = Point<T1::Output, T2::Output>;

    /// The function `hull_with` calculates the hull with another `Point` object by combining their x
    /// and y coordinates.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `hull_with` method is a reference to another `Point`
    ///             struct with the same generic types `T1` and `T2` as the current `Point` struct. It is used to
    ///             combine the coordinates of the current `Point` with the coordinates
    #[inline]
    fn hull_with(&self, other: &Point<T1, T2>) -> Self::Output {
        Self::Output::new(
            self.xcoord.hull_with(&other.xcoord),
            self.ycoord.hull_with(&other.ycoord),
        )
    }
}

/// The above Rust code is implementing an `Intersect` trait for the `Point` struct. The `Intersect`
/// trait is defined for two generic types `T1` and `T2`, and it requires that `T1` and `T2` implement
/// the `Intersect` trait themselves.
impl<T1, T2> Intersect<Point<T1, T2>> for Point<T1, T2>
where
    T1: Intersect<T1>,
    T2: Intersect<T2>,
{
    type Output = Point<T1::Output, T2::Output>;

    /// The `intersect_with` function takes another `Point` as input and returns a new `Point` with
    /// intersected coordinates.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `intersect_with` method is a reference to another
    ///             `Point` struct with the same generic types `T1` and `T2` as the current `Point` struct. It is
    ///             used to compare and intersect the `xcoord` and `ycoord`
    #[inline]
    fn intersect_with(&self, other: &Point<T1, T2>) -> Self::Output {
        Self::Output::new(
            self.xcoord.intersect_with(&other.xcoord),
            self.ycoord.intersect_with(&other.ycoord),
        )
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

/// The above code is a Rust macro definition that creates an implementation for a binary operation on a
/// custom type `Point<T1, T2>` and a reference to `Vector2<T1, T2>`. The macro is used to generate
/// implementations for various traits and methods for the specified types. In this case, it generates
/// an implementation for a specific binary operation method specified by the input parameters ``
/// and ``.
macro_rules! forward_val_xf_binop {
    (impl $imp:ident, $method:ident) => {
        /// The above code is implementing a trait for a specific type in Rust. The trait being
        /// implemented is not explicitly mentioned in the code snippet, but based on the syntax used
        /// (`impl Trait for Type`), it appears to be a custom trait defined elsewhere in the codebase.
        /// The code is implementing the trait for a specific type `Point<T1, T2>`, where `T1` and `T2`
        /// are generic types that must implement the `Clone` and `Num` traits.
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<&'a Vector2<T1, T2>> for Point<T1, T2> {
            type Output = Point<T1, T2>;

            /// The function implements a method that performs a specific operation on two Vector2
            /// instances in Rust.
            ///
            /// Arguments:
            ///
            /// * `other`: The `other` parameter in the code snippet represents a reference to a
            ///             `Vector2` struct with generic types `T1` and `T2`. This parameter is used as the input
            ///             for the method being called on `self`.
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

/// (a, b) - (c, d) == (a - c), (b - d)
/// The above Rust code snippet is implementing the subtraction operation for a Point struct. It defines
/// the implementation of the Sub trait for subtracting a Vector2 from a Point. The code defines the
/// behavior of subtracting a Vector2 from a Point to get a new Point with updated coordinates.
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

/// The above code is implementing the subtraction operation for a custom Point struct in Rust. It
/// defines the behavior of subtracting one Point from another Point, resulting in a Vector2
/// representing the displacement between the two points. The `sub` function takes two Point objects as
/// input and returns a Vector2 object with the x and y coordinates calculated by subtracting the
/// corresponding coordinates of the two points. The code also includes examples demonstrating the usage
/// of the subtraction operation with different scenarios.
impl<T1: Clone + Num, T2: Clone + Num> Sub for Point<T1, T2> {
    type Output = Vector2<T1, T2>;

    /// Displacement of two Points
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is of the same type as `self` and represents the other object
    ///             that you want to subtract from `self`.
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

    /// The above code is implementing the `AddAssign` trait for a custom type `Point<T1, T2>`. This
    /// implementation allows instances of `Point<T1, T2>` to be added to instances of `Vector2<T1, T2>`
    /// using the `+=` operator. Inside the `add_assign` function, the `xcoord` and `ycoord` fields of
    /// the `Point` instance are updated by adding the corresponding `x_` and `y_` fields of the
    /// `Vector2` instance.
    impl<T1: Clone + NumAssign, T2: Clone + NumAssign> AddAssign<Vector2<T1, T2>> for Point<T1, T2> {
        /// The `add_assign` function in Rust adds the x and y coordinates of another Vector2 to the
        /// current Vector2.
        ///
        /// Arguments:
        ///
        /// * `other`: The `other` parameter in the `add_assign` function is of type `Vector2<T1, T2>`.
        ///             It represents another instance of the `Vector2` struct with potentially different generic
        ///             types `T1` and `T2`.
        #[inline]
        fn add_assign(&mut self, other: Vector2<T1, T2>) {
            self.xcoord += other.x_;
            self.ycoord += other.y_;
        }
    }

    /// The above code is implementing the `SubAssign` trait for a custom type `Point<T1, T2>`. This
    /// implementation allows instances of `Point` to be subtracted by instances of `Vector2<T1, T2>`
    /// using the `-=` operator. Inside the implementation, it subtracts the `x_` and `y_` components of
    /// the `other` vector from the `xcoord` and `ycoord` components of the `Point` respectively.
    impl<T1: Clone + NumAssign, T2: Clone + NumAssign> SubAssign<Vector2<T1, T2>> for Point<T1, T2> {
        /// The function `sub_assign` subtracts the `x_` and `y_` components of another `Vector2` from
        /// the `xcoord` and `ycoord` components of the current `Vector2`.
        ///
        /// Arguments:
        ///
        /// * `other`: The `other` parameter in the `sub_assign` function is of type `Vector2<T1, T2>`.
        ///             It represents another instance of the `Vector2` struct with potentially different generic
        ///             types `T1` and `T2`. This parameter is used to subtract the `x_
        #[inline]
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

/// The above code is implementing the `Neg` trait for a custom type `Point<T1, T2>`. This
/// implementation allows for negating instances of the `Point` type. The `Neg` trait requires defining
/// an associated type `Output` and implementing the `neg` method which returns the negated version of
/// the `Point` instance by negating its `xcoord` and `ycoord` values.
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

/// The above code is implementing the `Neg` trait for a reference to a `Point<T1, T2>` struct in Rust.
/// The `Neg` trait is used for the negation operation (unary minus) on a value.
impl<'a, T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg
    for &'a Point<T1, T2>
{
    type Output = Point<T1, T2>;

    /// The function `neg` returns the negation of the cloned value.
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
    use crate::interval::Interval;

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
    fn test_min_dist_with() {
        let a = Point::new(3i32, 5i32);
        let b = Point::new(6i32, 4i32);
        assert_eq!(a.min_dist_with(&b), 4);
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

    #[test]
    fn test_point() {
        let a = Point::new(4, 8);
        let b = Point::new(5, 6);
        assert!(a < b);
        assert!(a <= b);
        assert_ne!(b, a);
    }

    #[test]
    fn test_point2() {
        let a = Point::new(3, 4);
        let r = Point::new(Interval::new(3, 4), Interval::new(5, 6)); // Rectangle
        assert!(!r.contains(&a));
        assert!(r.contains(&Point::new(4, 5)));
        assert!(!r.overlaps(&a));
        assert!(r.overlaps(&Point::new(4, 5)));
        assert!(r.overlaps(&Point::new(4, 6)));
        // assert_eq!(r.intersect_with(&Point::new(4, 5)), Point::new(Interval::new(4, 4), Interval::new(5, 5)));
    }

    #[test]
    fn test_transform() {
        let mut a = Point::new(3, 5);
        let b = Vector2::new(5, 7);
        assert_eq!(a + b, Point::new(8, 12));
        assert_eq!(a - b, Point::new(-2, -2));
        a += b;
        assert_eq!(a, Point::new(8, 12));
        a -= b;
        assert_eq!(a, Point::new(3, 5));
        assert_eq!(a.flip(), Point::new(5, 3));
    }

    #[test]
    fn test_displacement() {
        let a = Point::new(3, 5);
        let b = Point::new(5, 7);
        let c = Point::new(7, 8);
        assert_eq!(a.displace(&b), Vector2::new(-2, -2));
        assert_eq!(a.displace(&c), Vector2::new(-4, -3));
        assert_eq!(b.displace(&c), Vector2::new(-2, -1));
    }

    #[test]
    fn test_enlarge() {
        let _a = Point::new(3, 5);
        // assert_eq!(a.enlarge_with(2), Point::new(Interval::new(1, 5), Interval::new(3, 7)));
    }

    #[test]
    fn test_hull() {
        let a = Point::new(3, 5);
        let b = Point::new(5, 7);
        assert_eq!(
            a.hull_with(&b),
            Point::new(Interval::new(3, 5), Interval::new(5, 7))
        );
    }

    #[test]
    fn test_min_dist_with2() {
        let a = Point::new(3, 5);
        let b = Point::new(5, 7);
        assert_eq!(a.min_dist_with(&b), 4);
    }
}
