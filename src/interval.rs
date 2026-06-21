use crate::generic::{Contain, Displacement, MinDist, Overlap};

use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A range of values with a lower bound (`lb`) and an upper bound (`ub`).
///
/// ```svgbob
///  lb        ub
///   |---------|
///   *=========*-----> T
/// ```
///
/// An interval is valid when `lb <= ub`. The `is_invalid()` method checks for the
/// invalid case `lb > ub`.
///
/// # Examples
///
/// ```
/// use physdes::interval::Interval;
///
/// let interval = Interval::new(1, 5);
/// assert_eq!(interval.lb, 1);
/// assert_eq!(interval.ub, 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Interval<T> {
    pub lb: T,
    pub ub: T,
    pub _marker: PhantomData<T>,
}

impl<T> Interval<T> {
    /// Creates a new interval with the given lower and upper bounds.
    ///
    /// Note: No validity check is performed. An interval where `lb > ub` is considered
    /// invalid and can be checked with `is_invalid()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::interval::Interval;
    ///
    /// let interval = Interval::new(1, 5);
    /// assert_eq!(interval.lb, 1);
    /// assert_eq!(interval.ub, 5);
    ///
    /// let interval = Interval::new(5, 1);  // Invalid interval but still created
    /// assert_eq!(interval.lb, 5);
    /// assert_eq!(interval.ub, 1);
    /// ```
    #[inline]
    pub const fn new(lb: T, ub: T) -> Self {
        Self {
            lb,
            ub,
            _marker: PhantomData,
        }
    }
}

impl<T: Copy> Interval<T> {
    /// Returns the lower bound.
    #[inline]
    pub const fn lb(&self) -> T {
        self.lb
    }

    /// Returns the upper bound.
    #[inline]
    pub const fn ub(&self) -> T {
        self.ub
    }
}

impl<T: PartialOrd> Interval<T> {
    /// Returns `true` if the lower bound exceeds the upper bound (invalid interval).
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::interval::Interval;
    ///
    /// let valid_interval = Interval::new(1, 5);
    /// assert!(!valid_interval.is_invalid());
    ///
    /// let invalid_interval = Interval::new(5, 1);
    /// assert!(invalid_interval.is_invalid());
    /// ```
    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.lb > self.ub
    }
}

impl<T: Copy + Sub<Output = T>> Interval<T> {
    /// Computes the length of the interval: `ub - lb`.
    ///
    /// $$L = ub - lb$$
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::interval::Interval;
    ///
    /// let interval = Interval::new(2, 8);
    /// assert_eq!(interval.length(), 6);
    ///
    /// let interval = Interval::new(-3, 5);
    /// assert_eq!(interval.length(), 8);
    /// ```
    #[inline]
    pub fn length(&self) -> T {
        self.ub - self.lb
    }
}

impl<T> Display for Interval<T>
where
    T: PartialOrd + Copy + Display,
{
    /// Formats the interval as `[lb, ub]`.
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}, {}]", self.lb, self.ub)
    }
}

/// Interval subtraction (component-wise): $\[a,b\] - \[c,d\] = \[a-c,\; b-d\]$
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

/// Interval negation: $-\[a,b\] = \[-b,\; -a\]$
impl<T> Neg for Interval<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Interval<T>;

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
    /// Shift the interval by a scalar: $\[a,b\] \mathrel{+}= t \implies \[a+t,\; b+t\]$
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

    /// Shift the interval by a scalar: $\[a,b\] + t = \[a+t,\; b+t\]$
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
    /// Shift the interval by a negative scalar: $\[a,b\] \mathrel{-}= t \implies \[a-t,\; b-t\]$
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.lb -= rhs;
        self.ub -= rhs;
    }
}

/// Interval addition (component-wise): $\[a,b\] + \[c,d\] = \[a+c,\; b+d\]$
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

    /// Shift the interval by a negative scalar: $\[a,b\] - t = \[a-t,\; b-t\]$
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
    /// Scale the interval by a scalar: $\[a,b\] \mathrel{*}= t \implies \[a \cdot t,\; b \cdot t\]$
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

    /// Scale the interval by a scalar: $\[a,b\] \cdot t = \[a \cdot t,\; b \cdot t\]$
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb * rhs,
            ub: self.ub * rhs,
            _marker: self._marker,
        }
    }
}

/// Trait for enlarging a value by a given margin.
pub trait Enlarge<Alpha> {
    type Output;

    fn enlarge_with(&self, alpha: Alpha) -> Self::Output;
}

impl Enlarge<i32> for i32 {
    type Output = Interval<i32>;

    /// Enlarges a scalar to an interval: $\text{enlarge}(x, \alpha) = \[x-\alpha,\; x+\alpha\]$
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

    /// Enlarges the interval by `alpha` on both sides: $\text{enlarge}(\[a,b\], \alpha) = \[a-\alpha,\; b+\alpha\]$
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
    /// Orders intervals by their position: Less if `ub < other.lb`, Greater if `other.ub < self.lb`, Equal otherwise.
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

impl<T: PartialOrd> Overlap<Interval<T>> for Interval<T> {
    /// Checks if two intervals overlap: $\[a,b\] \cap \[c,d\] \neq \varnothing \iff a \le d \land c \le b$
    #[inline]
    fn overlaps(&self, other: &Interval<T>) -> bool {
        self.ub >= other.lb && other.ub >= self.lb
    }
}

impl<T: PartialOrd> Overlap<T> for Interval<T> {
    /// Checks if the interval contains a scalar value: $x \in \[a,b\] \iff a \le x \le b$
    #[inline]
    fn overlaps(&self, other: &T) -> bool {
        self.ub >= *other && *other >= self.lb
    }
}

impl<T: PartialOrd> Overlap<Interval<T>> for T {
    /// Checks if this value falls within the given interval.
    #[inline]
    fn overlaps(&self, other: &Interval<T>) -> bool {
        *self >= other.lb && other.ub >= *self
    }
}

impl<T: PartialOrd> Contain<Interval<T>> for Interval<T> {
    /// Checks if `self` entirely contains `other`: $\[a,b\] \supseteq \[c,d\] \iff a \le c \land d \le b$
    #[inline]
    fn contains(&self, other: &Interval<T>) -> bool {
        self.lb <= other.lb && other.ub <= self.ub
    }
}

impl<T: PartialOrd> Contain<T> for Interval<T> {
    /// Checks if the interval contains a scalar value: $x \in \[a,b\] \iff a \le x \le b$
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

    /// Computes the component-wise displacement between two intervals.
    #[inline]
    fn displace(&self, other: &Interval<T>) -> Self::Output {
        Self::Output::new(self.lb.displace(&other.lb), self.ub.displace(&other.ub))
    }
}

impl MinDist<Interval<i32>> for Interval<i32> {
    /// Computes the minimum distance between two intervals:
    ///
    /// $$d = \begin{cases} c-b & \text{if } b < c \\ a-d & \text{if } d < a \\ 0 & \text{otherwise} \end{cases}$$
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
    /// Computes the minimum distance between an interval and a scalar value.
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
    /// Computes the minimum distance between a scalar and an interval.
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

/// Trait for computing the convex hull (bounding interval) of two values.
pub trait Hull<T: ?Sized> {
    type Output;

    fn hull_with(&self, other: &T) -> Self::Output;
}

impl Hull<i32> for i32 {
    type Output = Interval<i32>;

    /// Computes the hull of two scalars: $\text{hull}(a,b) = \[\\min(a,b),\; \\max(a,b)\]$
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

    /// Computes the hull (bounding interval) of two intervals:
    ///
    /// $$\text{hull}(\[a,b\],\[c,d\]) = \[\\min(a,c),\; \\max(b,d)\]$$
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

    /// Computes the hull of a scalar value with an interval (delegates to `Interval::hull_with`).
    #[inline]
    fn hull_with(&self, other: &Interval<T>) -> Self::Output {
        other.hull_with(self)
    }
}

/// Trait for computing the intersection of two values.
pub trait Intersect<T: ?Sized> {
    type Output;

    fn intersect_with(&self, other: &T) -> Self::Output;
}

impl Intersect<i32> for i32 {
    type Output = Interval<i32>;

    /// Computes the intersection of two scalars: $\text{intersect}(a,b) = \[\\max(a,b),\; \\min(a,b)\]$
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

    /// Computes the intersection of two intervals:
    ///
    /// $$\text{intersect}(\[a,b\],\[c,d\]) = \[\\max(a,c),\; \\min(b,d)\]$$
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

    /// Computes the intersection of an interval and a scalar value.
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

    /// Computes the intersection of a scalar value with an interval.
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
        let interval_a = Interval::new(4, 8);
        let interval_b = Interval::new(5, 6);

        assert!(interval_a <= interval_b);
        assert!(interval_b <= interval_a);
        assert!(interval_a >= interval_b);
        assert!(interval_b >= interval_a);

        assert!(interval_a.overlaps(&interval_b));
        assert!(interval_b.overlaps(&interval_a));
        // assert!(!contain(&interval_a, &interval_b));
        assert!(interval_a.contains(&4));
        assert!(interval_a.contains(&8));
        assert!(interval_a.contains(&interval_b));
        assert_eq!(interval_a, interval_a);
        assert_eq!(interval_b, interval_b);
        assert_ne!(interval_a, interval_b);
        assert_ne!(interval_b, interval_a);
        assert!(interval_a.overlaps(&interval_a));
        assert!(interval_b.overlaps(&interval_b));
        assert!(interval_a.contains(&interval_a));
        assert!(interval_b.contains(&interval_b));
        assert!(interval_a.overlaps(&interval_b));
        assert!(interval_b.overlaps(&interval_a));
    }

    #[test]
    fn test_hull_more_cases() {
        let interval_a = Interval::new(3, 5);
        let interval_b = Interval::new(1, 7);
        assert_eq!(interval_a.hull_with(&interval_b), Interval::new(1, 7));

        let interval_c = Interval::new(-2, 2);
        assert_eq!(interval_a.hull_with(&interval_c), Interval::new(-2, 5));

        let val_d = 4;
        assert_eq!(interval_a.hull_with(&val_d), Interval::new(3, 5));

        let val_e = 8;
        assert_eq!(interval_a.hull_with(&val_e), Interval::new(3, 8));

        let val_f = 0;
        assert_eq!(interval_a.hull_with(&val_f), Interval::new(0, 5));
    }

    #[test]
    fn test_interval1() {
        let interval_a = Interval::new(4, 5);
        let interval_b = Interval::new(6, 8);
        assert!(interval_a < interval_b);
        assert!(!(interval_b == interval_a));
        assert!(interval_b != interval_a);
    }

    #[test]
    fn test_interval2() {
        let interval_a = Interval::new(4, 8);
        let interval_b = Interval::new(5, 6);

        assert!(interval_a.contains(&4));
        assert!(interval_a.contains(&8));
        assert!(interval_a.contains(&interval_b));
        // assert!(interval_a.intersect_with(&8) == Interval::new(8, 8));
        // assert!(interval_a.intersect_with(interval_b) == interval_b);
        assert!(!interval_b.contains(&interval_a));
        assert!(interval_a.overlaps(&interval_b));
        assert!(interval_b.overlaps(&interval_a));
    }

    #[test]
    fn test_interval3() {
        let interval_a = Interval::new(3, 4);
        assert!(interval_a.lb == 3);
        assert!(interval_a.ub == 4);
        // assert!(interval_a.length() == 1);
        assert!(interval_a.contains(&3));
        assert!(interval_a.contains(&4));
        assert!(!interval_a.contains(&5));
        assert!(interval_a.contains(&Interval::new(3, 4)));
        assert!(!interval_a.contains(&Interval::new(3, 5)));
        assert!(!interval_a.contains(&Interval::new(2, 3)));
        assert!(!interval_a.contains(&2));
        assert!(interval_a.contains(&4));
        assert!(!interval_a.contains(&5));
    }

    #[test]
    fn test_arithmetic() {
        let mut interval_a = Interval::new(3, 5);
        // interval_b = Interval::new(5, 7);
        // interval_c = Interval::new(7, 8);
        assert_eq!(interval_a + 1, Interval::new(4, 6));
        assert_eq!(interval_a - 1, Interval::new(2, 4));
        assert_eq!(interval_a * 2, Interval::new(6, 10));
        assert!(-interval_a == Interval::new(-5, -3));
        interval_a += 1;
        assert!(interval_a == Interval::new(4, 6));
        interval_a -= 1;
        assert!(interval_a == Interval::new(3, 5));
        interval_a *= 2;
        assert!(interval_a == Interval::new(6, 10));
    }

    #[test]
    fn test_overlap() {
        let interval_a = Interval::new(3, 5);
        let interval_b = Interval::new(5, 7);
        let interval_c = Interval::new(7, 8);
        assert!(interval_a.overlaps(&interval_b));
        assert!(interval_b.overlaps(&interval_c));
        assert!(!interval_a.overlaps(&interval_c));
        assert!(!interval_c.overlaps(&interval_a));

        let val_d = 4;
        assert!(interval_a.overlaps(&val_d));
        assert!(!interval_a.overlaps(&6));
        assert!(val_d.overlaps(&interval_a));
        assert!(val_d.overlaps(&val_d));
    }

    #[test]
    fn test_contains() {
        let interval_a = Interval::new(3, 5);
        let interval_b = Interval::new(5, 7);
        let interval_c = Interval::new(7, 8);
        assert!(!interval_a.contains(&interval_b));
        assert!(!interval_b.contains(&interval_c));
        assert!(!interval_a.contains(&interval_c));
        assert!(!interval_c.contains(&interval_a));

        let val_d = 4;
        assert!(interval_a.contains(&val_d));
        assert!(!interval_a.contains(&6));
        assert!(!val_d.contains(&interval_a));
        assert!(val_d.contains(&val_d));
    }

    #[test]
    fn test_intersect() {
        let interval_a = Interval::new(3, 5);
        let interval_b = Interval::new(5, 7);
        let interval_c = Interval::new(7, 8);
        assert_eq!(interval_a.intersect_with(&interval_b), Interval::new(5, 5));
        assert_eq!(interval_b.intersect_with(&interval_c), Interval::new(7, 7));
        assert!(interval_a.intersect_with(&interval_c).is_invalid());
        assert_eq!(interval_a.intersect_with(&interval_b), Interval::new(5, 5));
        assert_eq!(interval_b.intersect_with(&interval_c), Interval::new(7, 7));
        let val_d = 4;
        assert_eq!(interval_a.intersect_with(&val_d), Interval::new(4, 4));
        assert!(interval_a.intersect_with(&6).is_invalid());
        assert_eq!(interval_a.intersect_with(&val_d), Interval::new(4, 4));
        assert_eq!(val_d.intersect_with(&interval_a), Interval::new(4, 4));
        assert_eq!(val_d.intersect_with(&val_d), Interval::new(4, 4));
    }

    #[test]
    fn test_hull() {
        let interval_a = Interval::new(3, 5);
        let interval_b = Interval::new(5, 7);
        let interval_c = Interval::new(7, 8);
        assert_eq!(interval_a.hull_with(&interval_b), Interval::new(3, 7));
        assert_eq!(interval_b.hull_with(&interval_c), Interval::new(5, 8));
        assert_eq!(interval_a.hull_with(&interval_c), Interval::new(3, 8));
        let val_d = 4;
        assert_eq!(interval_a.hull_with(&val_d), Interval::new(3, 5));
        assert_eq!(interval_a.hull_with(&6), Interval::new(3, 6));
        // assert_eq!(hull(interval_a, val_d), Interval::new(3, 5));
        // assert_eq!(hull(interval_a, 6), Interval::new(3, 6));
        // assert_eq!(hull(val_d, interval_a), Interval::new(3, 5));
        // assert!(hull(6, interval_a) == Interval::new(3, 6));
        // assert!(hull(val_d, 6) == Interval::new(4, 6));
    }

    #[test]
    fn test_min_dist() {
        let interval_a = Interval::new(3, 5);
        let interval_b = Interval::new(5, 7);
        let interval_c = Interval::new(7, 8);
        assert_eq!(interval_a.min_dist_with(&interval_b), 0);
        assert_eq!(interval_a.min_dist_with(&interval_c), 2);
        assert_eq!(interval_b.min_dist_with(&interval_c), 0);
        let val_d = 4;
        assert_eq!(interval_a.min_dist_with(&val_d), 0);
        assert_eq!(val_d.min_dist_with(&interval_a), 0);
        assert_eq!(interval_a.min_dist_with(&6), 1);
        assert_eq!(6.min_dist_with(&interval_a), 1);
    }

    #[test]
    fn test_displacement() {
        let interval_a = Interval::new(3, 5);
        let interval_b = Interval::new(5, 7);
        let interval_c = Interval::new(7, 8);
        assert_eq!(interval_a.displace(&interval_b), Interval::new(-2, -2));
        assert_eq!(interval_a.displace(&interval_c), Interval::new(-4, -3));
        assert_eq!(interval_b.displace(&interval_c), Interval::new(-2, -1));
        let val_d = 4;
        assert_eq!(val_d.displace(&val_d), 0);
        assert_eq!(val_d.displace(&6), -2);
        assert_eq!(6.displace(&val_d), 2);
    }

    #[test]
    fn test_enlarge() {
        let interval_a = Interval::new(3, 5);
        assert!(interval_a.enlarge_with(2) == Interval::new(1, 7));
        let val_d = 4;
        assert_eq!(val_d.enlarge_with(6), Interval::new(-2, 10));
        assert_eq!(6.enlarge_with(val_d), Interval::new(2, 10));
    }

    #[test]
    fn test_min_dist_with_interval_other_ub_less_self_lb() {
        use crate::generic::MinDist;
        let a = Interval::new(10, 20);
        let b = Interval::new(0, 5);
        assert_eq!(a.min_dist_with(&b), 5);
    }

    #[test]
    fn test_min_dist_with_scalar_other_less_self_lb() {
        use crate::generic::MinDist;
        let a = Interval::new(10, 20);
        assert_eq!(a.min_dist_with(&5), 5);
    }

    #[test]
    fn test_min_dist_with_scalar_self_less_interval_lb() {
        let val = 3;
        let other = Interval::new(10, 20);
        assert_eq!(val.min_dist_with(&other), 7);
    }
}

#[test]
fn test_interval_length() {
    let interval = Interval::new(3, 8);
    assert_eq!(interval.length(), 5);

    let interval2 = Interval::new(-2, 3);
    assert_eq!(interval2.length(), 5);
}

#[test]
fn test_interval_display() {
    let interval = Interval::new(3, 8);
    let display_str = format!("{}", interval);
    assert_eq!(display_str, "[3, 8]");

    let interval2 = Interval::new(-5, 10);
    let display_str2 = format!("{}", interval2);
    assert_eq!(display_str2, "[-5, 10]");
}

#[test]
fn test_interval_sub_interval() {
    let interval_a = Interval::new(5, 10);
    let interval_b = Interval::new(2, 3);
    let result = interval_a - interval_b;
    assert_eq!(result, Interval::new(3, 7));
}

#[test]
fn test_interval_add_interval() {
    let interval_a = Interval::new(1, 3);
    let interval_b = Interval::new(2, 5);
    let result = interval_a + interval_b;
    assert_eq!(result, Interval::new(3, 8));
}

#[test]
fn test_interval_is_invalid() {
    let valid_interval = Interval::new(1, 5);
    assert!(!valid_interval.is_invalid());

    let invalid_interval = Interval::new(5, 1);
    assert!(invalid_interval.is_invalid());
}

#[test]
fn test_interval_sub_assign_interval() {
    let mut interval = Interval::new(10, 20);
    interval -= 3;
    assert_eq!(interval, Interval::new(7, 17));
}

#[test]
fn test_interval_hull_t_for_t() {
    let val = 5;
    let interval = Interval::new(3, 8);
    // T as Hull<Interval<T>>
    let result = val.hull_with(&interval);
    assert_eq!(result.lb, 3);
    assert_eq!(result.ub, 8);
}

#[test]
fn test_interval_contains_interval_t() {
    let interval = Interval::new(3, 8);
    // Contain<Interval<T>> for T - always returns false
    let val = 5;
    assert!(!val.contains(&interval));
}

#[test]
fn test_interval_overlap_interval_t() {
    // Overlap<Interval<T>> for T
    let val = 5;
    let interval = Interval::new(3, 8);
    assert!(val.overlaps(&interval));

    let val2 = 10;
    assert!(!val2.overlaps(&interval));
}

#[test]
fn test_interval_partial_cmp_greater() {
    let interval_a = Interval::new(8, 10);
    let interval_b = Interval::new(3, 5);
    // interval_a is greater than interval_b (no overlap, ub > other.lb)
    let cmp = interval_a.partial_cmp(&interval_b);
    assert_eq!(cmp, Some(std::cmp::Ordering::Greater));
}
