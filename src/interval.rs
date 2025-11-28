use crate::generic::{Contain, Displacement, MinDist, Overlap};

use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// The `Interval` struct represents a range of values with a lower bound (`lb`) and an upper bound
/// (`ub`).
///
/// ```svgbob
///  lb        ub
///   |---------|
///   *=========*-----> T
/// ```
///
/// Properties:
///
/// * `lb`: The `lb` property represents the lower bound of the interval. It is of type `T`, which is a
///   generic type that must implement the `PartialOrd` trait. This means that the type `T` must be able
///   to be compared for ordering.
/// * `ub`: The `ub` property represents the upper bound of the interval. It is of type `T`, which is a
///   generic type that must implement the `PartialOrd` trait. The `PartialOrd` trait allows for
///   comparison between values of type `T`.
/// * `_marker`: The `_marker` field is a marker field that is used to indicate that the generic type
///   `T` is used in the struct. It is typically used when you want to associate a type parameter with a
///   struct, but you don't actually need to store any values of that type in the struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T> {
    pub lb: T,
    pub ub: T,
    pub _marker: PhantomData<T>,
}

impl<T> Interval<T> {
    /// The function `new` creates a new instance of a struct with given lower and upper bounds.
    ///
    /// Arguments:
  ///
  /// * `lb`: The `lb` parameter represents the lower bound value. It is of type `T`, which means it
  ///   can be any type that implements the necessary traits for the struct.
  /// * `ub`: The `ub` parameter represents the upper bound value. It is of type `T`, which means it
    ///   can be any type that implements the necessary traits for the struct.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the struct `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::interval::Interval;
    /// use std::marker::PhantomData;
    ///
    /// assert_eq!(Interval::new(1, 2), Interval { lb: 1, ub: 2, _marker: PhantomData });
    /// assert_eq!(Interval::new(2, 1), Interval { lb: 2, ub: 1, _marker: PhantomData });
    /// ```
    #[inline]
    pub fn new(lb: T, ub: T) -> Self {
        Self {
            lb,
            ub,
            _marker: PhantomData,
        }
    }
}

impl<T: Copy> Interval<T> {
    /// The function `lb` returns the value of the field `lb` from the struct.
    ///
    /// Returns:
    ///
    /// The `lb` method is returning the value of the `lb` field of the struct or object that the method
    /// is being called on.
    #[inline]
    pub fn lb(&self) -> T {
        self.lb
    }

    /// This Rust function returns the value of the field `ub`.
    ///
    /// Returns:
    ///
    /// The `ub` field of the struct is being returned.
    #[inline]
    pub fn ub(&self) -> T {
        self.ub
    }
}

impl<T: PartialOrd> Interval<T> {
    /// The function `is_invalid` checks if the lower bound is greater than the upper bound.
    ///
    /// Returns:
    ///
    /// The `is_invalid` function is returning a boolean value based on the comparison `self.lb >
    /// self.ub`. If `self.lb` is greater than `self.ub`, it will return `true`, indicating that the
    /// values are invalid. Otherwise, it will return `false`.
    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.lb > self.ub
    }
}

impl<T: Copy + Sub<Output = T>> Interval<T> {
    /// The `length` function calculates the difference between the upper bound (`ub`) and lower bound
    /// (`lb`) of a value.
    ///
    /// Returns:
    ///
    /// The `length` method is returning the difference between the `ub` (upper bound) and `lb` (lower
    /// bound) values of the struct instance.
    #[inline]
    pub fn length(&self) -> T {
        self.ub - self.lb
    }
}

impl<T> Display for Interval<T>
where
    T: PartialOrd + Copy + Display,
{
    /// The function `fmt` in Rust is used to format a struct by writing its lower bound and upper bound
    /// values in square brackets.
    ///
    /// Arguments:
    ///
    /// * `f`: The `f` parameter in the `fmt` function is a mutable reference to a `Formatter` struct.
    ///   This `Formatter` struct is used for formatting and writing output.
    ///
    /// Returns:
    ///
    /// The `fmt` method is returning a `FmtResult`, which is an alias for `Result<(), Error>`.
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}, {}]", self.lb, self.ub)
    }
}

impl<T: Sub<Output = T>> Sub for Interval<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            lb: self.lb - other.lb,
            ub: self.ub - other.ub,
            _marker: PhantomData,
        }
    }
}

impl<T> Neg for Interval<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Interval<T>;

    /// The `neg` function in Rust returns a new `Interval` with its lower and upper bounds negated.
    #[inline]
    fn neg(self) -> Self::Output {
        Interval {
            lb: -self.ub,
            ub: -self.lb,
            _marker: self._marker,
        }
    }
}

impl<T> AddAssign<T> for Interval<T>
where
    T: Copy + AddAssign<T>,
{
    /// The `add_assign` function in Rust adds a value to both the lower and upper bounds of a data
    /// structure.
    ///
    /// Arguments:
    ///
    /// * `rhs`: The `rhs` parameter in the `add_assign` function represents the right-hand side operand
    ///   that will be added to the `lb` and `ub` fields of the struct or object on which the method is
  ///   called.
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.lb += rhs;
        self.ub += rhs;
    }
}

impl<T> Add<T> for Interval<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Interval<T>;

    /// The `add` function in Rust adds a value to both the lower and upper bounds of an `Interval`
    /// struct.
    ///
    /// Arguments:
    ///
    /// * `rhs`: The `rhs` parameter in the `add` function represents the right-hand side operand that
    ///   will be added to the current `Interval` instance.
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb + rhs,
            ub: self.ub + rhs,
            _marker: self._marker,
        }
    }
}

impl<T> SubAssign<T> for Interval<T>
where
    T: Copy + SubAssign<T>,
{
    /// The `sub_assign` function subtracts a value from both the lower and upper bounds of a variable.
    ///
    /// Arguments:
    ///
    /// * `rhs`: `rhs` is a parameter of type `T` that is passed by value to the `sub_assign` function.
    ///   It is used to subtract its value from both `self.lb` and `self.ub` in the function
  ///   implementation.
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.lb -= rhs;
        self.ub -= rhs;
    }
}

impl<T: Add<Output = T>> Add for Interval<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            lb: self.lb + other.lb,
            ub: self.ub + other.ub,
            _marker: PhantomData,
        }
    }
}

impl<T> Sub<T> for Interval<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Interval<T>;

    /// The function subtracts a value from both the lower and upper bounds of an interval.
    ///
    /// Arguments:
    ///
    /// * `rhs`: The `rhs` parameter in the code snippet represents the right-hand side operand that
    ///   will be subtracted from the interval's lower bound (`lb`) and upper bound (`ub`) values.
    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb - rhs,
            ub: self.ub - rhs,
            _marker: self._marker,
        }
    }
}

impl<T> MulAssign<T> for Interval<T>
where
    T: Copy + MulAssign<T>,
{
    /// The `mul_assign` function in Rust multiplies both the lower and upper bounds of a range by a
    /// given value.
    ///
    /// Arguments:
    ///
    /// * `rhs`: The `rhs` parameter in the `mul_assign` function represents the value that will be
    ///   multiplied with the `lb` and `ub` fields of the struct or object on which the method is called.
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.lb *= rhs;
        self.ub *= rhs;
    }
}

impl<T> Mul<T> for Interval<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Interval<T>;

    /// The `mul` function in Rust defines multiplication for an `Interval` type.
    ///
    /// Arguments:
    ///
    /// * `rhs`: The `rhs` parameter in the `mul` function represents the right-hand side operand that
    ///   will be multiplied with the `Interval` instance on which the method is called.
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb * rhs,
            ub: self.ub * rhs,
            _marker: self._marker,
        }
    }
}

/// The above code is defining a trait named `Enlarge` in Rust. This trait has an associated type
/// `Output` and a method `enlarge_with` that takes a reference to `self` and a parameter `alpha` of
/// type `T`. The method returns an object of type `Output`. This trait can be implemented for types to
/// provide the functionality of enlarging or modifying the object with the provided `alpha` value.
pub trait Enlarge<Alpha> {
    type Output;

    fn enlarge_with(&self, alpha: Alpha) -> Self::Output;
}

impl Enlarge<i32> for i32 {
    type Output = Interval<i32>;

    /// The `enlarge_with` function takes an integer `alpha` and returns an `Interval` struct with lower
    /// bound as `self - alpha` and upper bound as `self + alpha`.
    ///
    /// Arguments:
    ///
    /// * `alpha`: The `alpha` parameter in the `enlarge_with` function represents the amount by which
    ///   the interval should be enlarged. It is an `i32` type, which means it is an integer value.
    ///
    /// Returns:
    ///
    /// An `Interval<i32>` struct is being returned.
    #[inline]
    fn enlarge_with(&self, alpha: i32) -> Interval<i32> {
        Interval {
            lb: *self - alpha,
            ub: *self + alpha,
            _marker: PhantomData,
        }
    }
}

impl<T> Enlarge<T> for Interval<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    type Output = Interval<T>;

    /// The `enlarge_with` function in Rust enlarges an interval by adding a specified value to its
    /// lower bound and subtracting the same value from its upper bound.
    ///
    /// Arguments:
    ///
    /// * `alpha`: The `alpha` parameter in the `enlarge_with` function represents the amount by which
    ///   the lower bound (`lb`) and upper bound (`ub`) of an `Interval` struct are adjusted. The lower
  ///   bound is decreased by `alpha` and the upper bound is increased by `alpha`, effectively enlarg
    ///
    /// Returns:
    ///
    /// The `enlarge_with` method is returning a new `Interval` instance with the lower bound (`lb`)
    /// decreased by `alpha` and the upper bound (`ub`) increased by `alpha`. The `_marker` field is
    /// copied from the original `Interval` instance.
    #[inline]
    fn enlarge_with(&self, alpha: T) -> Self {
        Interval {
            lb: self.lb - alpha,
            ub: self.ub + alpha,
            _marker: self._marker,
        }
    }
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    /// The function `partial_cmp` compares the lower bound of `self` with the upper bound of `other`
    /// and returns the result as an `Option` of `Ordering`.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to another object of the same type as `self`.
    ///
    /// Returns:
    ///
    /// an `Option` containing a `std::cmp::Ordering` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::interval::Interval;
    /// use std::marker::PhantomData;
    /// assert_eq!(Interval::new(1, 2).partial_cmp(&Interval::new(3, 4)), Some(std::cmp::Ordering::Less));
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ub < other.lb {
            Some(std::cmp::Ordering::Less)
        } else if other.ub < self.lb {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

/// The `impl<T: PartialOrd> Overlap<Interval<T>> for Interval<T>` block is implementing the `Overlap`
/// trait for the `Interval<T>` struct.
impl<T: PartialOrd> Overlap<Interval<T>> for Interval<T> {
    /// The `overlaps` function in Rust checks if two intervals overlap with each other.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `overlaps` function represents another interval that you
    ///   want to check for overlap with the interval on which the method is called.
    ///
    /// Returns:
    ///
    /// The `overlaps` function is returning a boolean value, which indicates whether the interval
    /// `self` overlaps with the interval `other`. If there is an overlap between the two intervals, the
    /// function will return `true`, otherwise it will return `false`.
    #[inline]
    fn overlaps(&self, other: &Interval<T>) -> bool {
        self.ub >= other.lb && other.ub >= self.lb
    }
}

impl<T: PartialOrd> Overlap<T> for Interval<T> {
    /// The `overlaps` function in Rust checks if two values overlap within a range.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to an object of type `T`, which is the same type
    ///   as the object that the method `overlaps` is being called on.
    ///
    /// Returns:
    ///
    /// The `overlaps` function is returning a boolean value, which indicates whether the range
    /// represented by `self` overlaps with the range represented by `other`.
    #[inline]
    fn overlaps(&self, other: &T) -> bool {
        self.ub >= *other && *other >= self.lb
    }
}

impl<T: PartialOrd> Overlap<Interval<T>> for T {
    /// The `overlaps` function in Rust checks if two intervals overlap with each other.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to an `Interval<T>` struct, which represents
    ///   another interval. The `Interval<T>` struct likely contains two fields, `lb` and `ub`,
  ///   representing the lower and upper bounds of the interval, respectively. The `overlaps` method is
  ///   used to
    ///
    /// Returns:
    ///
    /// The `overlaps` function is returning a boolean value. It checks if the current interval (`self`)
    /// overlaps with another interval (`other`) by comparing their lower bounds and upper bounds. If
    /// there is any overlap between the two intervals, it returns `true`, otherwise it returns `false`.
    #[inline]
    fn overlaps(&self, other: &Interval<T>) -> bool {
        *self >= other.lb && other.ub >= *self
    }
}

/// The `impl<T: PartialOrd> Contain<Interval<T>> for Interval<T>` block is implementing the `Contain`
/// trait for the `Interval<T>` struct.
impl<T: PartialOrd> Contain<Interval<T>> for Interval<T> {
    /// The `contains` function in Rust checks if one interval contains another interval.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to an `Interval<T>` object that is being
    ///   compared to the current `Interval<T>` object.
    ///
    /// Returns:
    ///
    /// The `contains` method is returning a boolean value, which indicates whether the interval `self`
    /// contains the interval `other`. It checks if the lower bound of `self` is less than or equal to
    /// the lower bound of `other`, and if the upper bound of `other` is less than or equal to the upper
    /// bound of `self`. If both conditions are true, it returns `true
    #[inline]
    fn contains(&self, other: &Interval<T>) -> bool {
        self.lb <= other.lb && other.ub <= self.ub
    }
}

/// The `impl<T: PartialOrd> Contain<T> for Interval<T>` block is implementing the `Contain` trait for
/// the `Interval<T>` struct.
impl<T: PartialOrd> Contain<T> for Interval<T> {
    /// The function checks if a value is within a specified range.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to a value of type `T`, which is the same type
    ///   as the elements stored in the struct or data structure that contains the `contains` method. The
  ///   method checks if the value referenced by `other` falls within the range defined by the lower
  ///   bound (`
    ///
    /// Returns:
    ///
    /// A boolean value is being returned, indicating whether the value `other` is within the range
    /// defined by `self.lb` and `self.ub`.
    #[inline]
    fn contains(&self, other: &T) -> bool {
        self.lb <= *other && *other <= self.ub
    }
}

impl<T: PartialOrd> Contain<Interval<T>> for T {
    /// The function `contains` always returns `false` and takes a reference to another `Interval` as
    /// input.
    ///
    /// Arguments:
    ///
    /// * `_other`: The `_other` parameter is a reference to an `Interval` object of the same type `T`
    ///   as the current object.
    ///
    /// Returns:
    ///
    /// The `contains` function is returning a boolean value `false`.
    #[inline]
    fn contains(&self, _other: &Interval<T>) -> bool {
        false
    }
}

impl<T> Displacement<Interval<T>> for Interval<T>
where
    T: Displacement<T, Output = T>,
{
    type Output = Interval<T>;

    /// The `displace` function in Rust calculates the displacement between two intervals.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `displace` function represents another `Interval` object
    ///   of the same type as `self`. It is used to displace the lower bound and upper bound of the
  ///   current `Interval` object (`self`) by the corresponding lower and upper bounds of the `other`
    #[inline]
    fn displace(&self, other: &Interval<T>) -> Self::Output {
        Self::Output::new(self.lb.displace(&other.lb), self.ub.displace(&other.ub))
    }
}

impl MinDist<Interval<i32>> for Interval<i32> {
    /// The `min_dist_with` function calculates the minimum distance between two intervals of integers.
    ///
    /// Arguments:
    ///
    /// * `other`: The `min_dist_with` function calculates the minimum distance between two intervals.
    ///   The `self` interval is represented by the lower bound `lb` and upper bound `ub` of the current
  ///   instance, while the `other` interval is passed as a reference to an `Interval<i32>`.
    ///
    /// Returns:
    ///
    /// The `min_dist_with` function returns the minimum distance between two intervals. It calculates
    /// the distance based on the upper and lower bounds of the intervals. If the upper bound of the
    /// first interval is less than the lower bound of the second interval, it returns the difference
    /// between the lower bound of the second interval and the upper bound of the first interval. If the
    /// upper bound of the second interval is less
    #[inline]
    fn min_dist_with(&self, other: &Interval<i32>) -> u32 {
        if self.ub < other.lb {
            (other.lb - self.ub) as u32
        } else if other.ub < self.lb {
            (self.lb - other.ub) as u32
        } else {
            0
        }
    }
}

impl MinDist<i32> for Interval<i32> {
    /// This Rust function calculates the minimum distance between a value and a range defined by lower
    /// and upper bounds.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `min_dist_with` function is a reference to an `i32`
    ///   value. This parameter is used to calculate the minimum distance between the current instance
  ///   (self) and the provided `i32` value.
    ///
    /// Returns:
    ///
    /// The `min_dist_with` function returns the minimum distance between the value represented by
    /// `self` and the value referenced by `other`. If the value referenced by `other` is greater than
    /// the upper bound (`ub`) of `self`, it returns the difference between `other` and `ub`. If the
    /// value referenced by `other` is less than the lower bound (`lb`) of `self
    #[inline]
    fn min_dist_with(&self, other: &i32) -> u32 {
        if self.ub < *other {
            (*other - self.ub) as u32
        } else if *other < self.lb {
            (self.lb - *other) as u32
        } else {
            0
        }
    }
}

impl MinDist<Interval<i32>> for i32 {
    /// This Rust function calculates the minimum distance between two intervals of integers.
    ///
    /// Arguments:
    ///
    /// * `other`: The `min_dist_with` function calculates the minimum distance between two intervals.
    ///   The `self` interval is compared with the `other` interval to determine the minimum distance
  ///   between them.
    ///
    /// Returns:
    ///
    /// The `min_dist_with` function returns the minimum distance between two intervals. If the lower
    /// bound of `self` is less than the lower bound of `other`, it returns the difference between the
    /// lower bounds as a `u32`. If the upper bound of `other` is less than the upper bound of `self`,
    /// it returns the difference between the upper bounds as a `u32`. Otherwise
    #[inline]
    fn min_dist_with(&self, other: &Interval<i32>) -> u32 {
        if *self < other.lb {
            (other.lb - *self) as u32
        } else if other.ub < *self {
            (*self - other.ub) as u32
        } else {
            0
        }
    }
}

/// The above code is defining a trait named `Hull` in Rust. This trait has a generic type `T` that must
/// be sized. It also has an associated type `Output`. The trait has a method `hull_with` that takes a
/// reference to another object of type `T` and returns an object of type `Output`. This trait can be
/// implemented for different types to provide the `hull_with` functionality.
pub trait Hull<T: ?Sized> {
    type Output;

    fn hull_with(&self, other: &T) -> Self::Output;
}

impl Hull<i32> for i32 {
    type Output = Interval<i32>;

    /// The function `hull_with` calculates the lower and upper bounds between two values.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `hull_with` function is a reference to an `i32` type.
    ///   This parameter is used to calculate the lower bound (`lb`) and upper bound (`ub`) values for the
  ///   output struct.
    #[inline]
    fn hull_with(&self, other: &i32) -> Self::Output {
        if *self < *other {
            Interval::new(*self, *other)
        } else {
            Interval::new(*other, *self)
        }
    }
}

impl<T> Hull<Interval<T>> for Interval<T>
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

    /// The `hull_with` function calculates the hull (bounding interval) of two intervals by taking the
    /// minimum lower bound and maximum upper bound.
    ///
    /// Arguments:
    ///
    /// * `other`: `other` is a reference to an `Interval<T>` object that is being passed as a parameter
    ///   to the `hull_with` method.
    #[inline]
    fn hull_with(&self, other: &Interval<T>) -> Self::Output {
        Self::Output {
            lb: self.lb.min(other.lb),
            ub: self.ub.max(other.ub),
            _marker: self._marker,
        }
    }
}

impl<T> Hull<T> for Interval<T>
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

    /// The `hull_with` function calculates the hull (minimum and maximum values) between two values.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to a value of type `T`, which is the same type
    ///   as the values stored in the struct implementing the `hull_with` method. In this method, the
  ///   `other` value is used to update the lower bound (`lb`) and upper bound (`
    #[inline]
    fn hull_with(&self, other: &T) -> Self::Output {
        Self::Output {
            lb: self.lb.min(*other),
            ub: self.ub.max(*other),
            _marker: self._marker,
        }
    }
}

impl<T> Hull<Interval<T>> for T
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

    /// The `hull_with` function in Rust calculates the convex hull of two intervals.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `hull_with` function is a reference to an `Interval<T>`
    ///   object.
    #[inline]
    fn hull_with(&self, other: &Interval<T>) -> Self::Output {
        other.hull_with(self)
    }
}

/// The above code is defining a trait in Rust called `Intersect`. This trait has one associated type
/// `Output` and one method `intersect_with` that takes a reference to another object of type `T` and
/// returns an object of type `Output`. This trait can be implemented for various types to provide
/// custom intersection behavior.
pub trait Intersect<T: ?Sized> {
    type Output;

    fn intersect_with(&self, other: &T) -> Self::Output;
}

impl Intersect<i32> for i32 {
    type Output = Interval<i32>;

    /// The `intersect_with` function calculates the intersection of two values by finding the maximum
    /// lower bound and minimum upper bound.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `intersect_with` function is a reference to an `i32`
    ///   type. This parameter is used to find the intersection between the current instance (self) and
  ///   the provided `i32` value.
    #[inline]
    fn intersect_with(&self, other: &i32) -> Self::Output {
        Self::Output {
            lb: (*self).max(*other),
            ub: (*self).min(*other),
            _marker: PhantomData,
        }
    }
}

impl<T> Intersect<Interval<T>> for Interval<T>
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

    /// The `intersect_with` function returns the intersection of two intervals by finding the maximum
    /// lower bound and minimum upper bound between them.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to an `Interval<T>` object, which is used to
    ///   intersect with the current `Interval<T>` object. The `intersect_with` method calculates the
  ///   intersection of the two intervals and returns a new `Interval<T>` object as the output.
    #[inline]
    fn intersect_with(&self, other: &Interval<T>) -> Self::Output {
        Self::Output {
            lb: self.lb.max(other.lb),
            ub: self.ub.min(other.ub),
            _marker: self._marker,
        }
    }
}

impl<T> Intersect<T> for Interval<T>
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

    /// The `intersect_with` function calculates the intersection of two values.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to an object of type `T`, which is the same type
    ///   as the object on which the `intersect_with` method is being called. The method calculates the
  ///   intersection of the object's lower bound (`lb`) and upper bound (`ub`) with the corresponding
  ///   values
    #[inline]
    fn intersect_with(&self, other: &T) -> Self::Output {
        Self::Output {
            lb: self.lb.max(*other),
            ub: self.ub.min(*other),
            _marker: self._marker,
        }
    }
}

impl<T> Intersect<Interval<T>> for T
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

    /// The `intersect_with` function in Rust swaps the receiver and argument before calling the
    /// `intersect_with` method on the argument.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `intersect_with` function represents another
    ///   `Interval<T>` that you want to intersect with the current interval.
    #[inline]
    fn intersect_with(&self, other: &Interval<T>) -> Self::Output {
        other.intersect_with(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval() {
        let a = Interval::new(4, 8);
        let b = Interval::new(5, 6);

        assert!(a <= b);
        assert!(b <= a);
        assert!(a >= b);
        assert!(b >= a);

        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
        // assert!(!contain(&a, &b));
        assert!(a.contains(&4));
        assert!(a.contains(&8));
        assert!(a.contains(&b));
        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_ne!(a, b);
        assert_ne!(b, a);
        assert!(a.overlaps(&a));
        assert!(b.overlaps(&b));
        assert!(a.contains(&a));
        assert!(b.contains(&b));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_hull_more_cases() {
        let a = Interval::new(3, 5);
        let b = Interval::new(1, 7);
        assert_eq!(a.hull_with(&b), Interval::new(1, 7));

        let c = Interval::new(-2, 2);
        assert_eq!(a.hull_with(&c), Interval::new(-2, 5));

        let d = 4;
        assert_eq!(a.hull_with(&d), Interval::new(3, 5));

        let e = 8;
        assert_eq!(a.hull_with(&e), Interval::new(3, 8));

        let f = 0;
        assert_eq!(a.hull_with(&f), Interval::new(0, 5));
    }

    #[test]
    fn test_interval1() {
        let a = Interval::new(4, 5);
        let b = Interval::new(6, 8);
        assert!(a < b);
        assert!(!(b == a));
        assert!(b != a);
    }

    #[test]
    fn test_interval2() {
        let a = Interval::new(4, 8);
        let b = Interval::new(5, 6);

        assert!(a.contains(&4));
        assert!(a.contains(&8));
        assert!(a.contains(&b));
        // assert!(a.intersect_with(&8) == Interval::new(8, 8));
        // assert!(a.intersect_with(b) == b);
        assert!(!b.contains(&a));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_interval3() {
        let a = Interval::new(3, 4);
        assert!(a.lb == 3);
        assert!(a.ub == 4);
        // assert!(a.length() == 1);
        assert!(a.contains(&3));
        assert!(a.contains(&4));
        assert!(!a.contains(&5));
        assert!(a.contains(&Interval::new(3, 4)));
        assert!(!a.contains(&Interval::new(3, 5)));
        assert!(!a.contains(&Interval::new(2, 3)));
        assert!(!a.contains(&2));
        assert!(a.contains(&4));
        assert!(!a.contains(&5));
    }

    #[test]
    fn test_arithmetic() {
        let mut a = Interval::new(3, 5);
        // b = Interval::new(5, 7);
        // c = Interval::new(7, 8);
        assert_eq!(a + 1, Interval::new(4, 6));
        assert_eq!(a - 1, Interval::new(2, 4));
        assert_eq!(a * 2, Interval::new(6, 10));
        assert!(-a == Interval::new(-5, -3));
        a += 1;
        assert!(a == Interval::new(4, 6));
        a -= 1;
        assert!(a == Interval::new(3, 5));
        a *= 2;
        assert!(a == Interval::new(6, 10));
    }

    #[test]
    fn test_overlap() {
        let a = Interval::new(3, 5);
        let b = Interval::new(5, 7);
        let c = Interval::new(7, 8);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&c));
        assert!(!a.overlaps(&c));
        assert!(!c.overlaps(&a));

        let d = 4;
        assert!(a.overlaps(&d));
        assert!(!a.overlaps(&6));
        assert!(d.overlaps(&a));
        assert!(d.overlaps(&d));
    }

    #[test]
    fn test_contains() {
        let a = Interval::new(3, 5);
        let b = Interval::new(5, 7);
        let c = Interval::new(7, 8);
        assert!(!a.contains(&b));
        assert!(!b.contains(&c));
        assert!(!a.contains(&c));
        assert!(!c.contains(&a));

        let d = 4;
        assert!(a.contains(&d));
        assert!(!a.contains(&6));
        assert!(!d.contains(&a));
        assert!(d.contains(&d));
    }

    #[test]
    fn test_intersect() {
        let a = Interval::new(3, 5);
        let b = Interval::new(5, 7);
        let c = Interval::new(7, 8);
        assert_eq!(a.intersect_with(&b), Interval::new(5, 5));
        assert_eq!(b.intersect_with(&c), Interval::new(7, 7));
        assert!(a.intersect_with(&c).is_invalid());
        assert_eq!(a.intersect_with(&b), Interval::new(5, 5));
        assert_eq!(b.intersect_with(&c), Interval::new(7, 7));
        let d = 4;
        assert_eq!(a.intersect_with(&d), Interval::new(4, 4));
        assert!(a.intersect_with(&6).is_invalid());
        assert_eq!(a.intersect_with(&d), Interval::new(4, 4));
        assert_eq!(d.intersect_with(&a), Interval::new(4, 4));
        assert_eq!(d.intersect_with(&d), Interval::new(4, 4));
    }

    #[test]
    fn test_hull() {
        let a = Interval::new(3, 5);
        let b = Interval::new(5, 7);
        let c = Interval::new(7, 8);
        assert_eq!(a.hull_with(&b), Interval::new(3, 7));
        assert_eq!(b.hull_with(&c), Interval::new(5, 8));
        assert_eq!(a.hull_with(&c), Interval::new(3, 8));
        let d = 4;
        assert_eq!(a.hull_with(&d), Interval::new(3, 5));
        assert_eq!(a.hull_with(&6), Interval::new(3, 6));
        // assert_eq!(hull(a, d), Interval::new(3, 5));
        // assert_eq!(hull(a, 6), Interval::new(3, 6));
        // assert_eq!(hull(d, a), Interval::new(3, 5));
        // assert!(hull(6, a) == Interval::new(3, 6));
        // assert!(hull(d, 6) == Interval::new(4, 6));
    }

    #[test]
    fn test_min_dist() {
        let a = Interval::new(3, 5);
        let b = Interval::new(5, 7);
        let c = Interval::new(7, 8);
        assert_eq!(a.min_dist_with(&b), 0);
        assert_eq!(a.min_dist_with(&c), 2);
        assert_eq!(b.min_dist_with(&c), 0);
        let d = 4;
        assert_eq!(a.min_dist_with(&d), 0);
        assert_eq!(d.min_dist_with(&a), 0);
        assert_eq!(a.min_dist_with(&6), 1);
        assert_eq!(6.min_dist_with(&a), 1);
    }

    #[test]
    fn test_displacement() {
        let a = Interval::new(3, 5);
        let b = Interval::new(5, 7);
        let c = Interval::new(7, 8);
        assert_eq!(a.displace(&b), Interval::new(-2, -2));
        assert_eq!(a.displace(&c), Interval::new(-4, -3));
        assert_eq!(b.displace(&c), Interval::new(-2, -1));
        let d = 4;
        assert_eq!(d.displace(&d), 0);
        assert_eq!(d.displace(&6), -2);
        assert_eq!(6.displace(&d), 2);
    }

    #[test]
    fn test_enlarge() {
        let a = Interval::new(3, 5);
        assert!(a.enlarge_with(2) == Interval::new(1, 7));
        let d = 4;
        assert_eq!(d.enlarge_with(6), Interval::new(-2, 10));
        assert_eq!(6.enlarge_with(d), Interval::new(2, 10));
    }
}
