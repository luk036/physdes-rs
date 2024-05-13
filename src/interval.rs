use crate::generic::{Contain, Displacement, MinDist, Overlap};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// The `Interval` struct represents a range of values with a lower bound (`lb`) and an upper bound
/// (`ub`).
///
/// Properties:
///
/// * `lb`: The `lb` property represents the lower bound of the interval. It is of type `T`, which is a
/// generic type that must implement the `PartialOrd` trait. This means that the type `T` must be able
/// to be compared for ordering.
/// * `ub`: The `ub` property represents the upper bound of the interval. It is of type `T`, which is a
/// generic type that must implement the `PartialOrd` trait. The `PartialOrd` trait allows for
/// comparison between values of type `T`.
/// * `_marker`: The `_marker` field is a marker field that is used to indicate that the generic type
/// `T` is used in the struct. It is typically used when you want to associate a type parameter with a
/// struct, but you don't actually need to store any values of that type in the struct.
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
    /// can be any type that implements the necessary traits for the struct.
    /// * `ub`: The `ub` parameter represents the upper bound value. It is of type `T`, which means it
    /// can be any type that implements the necessary traits for the struct.
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
    #[inline]
    pub fn lb(&self) -> T {
        self.lb
    }

    #[inline]
    pub fn ub(&self) -> T {
        self.ub
    }
}

impl<T: PartialOrd> Interval<T> {
    #[inline]
    pub fn is_invalid(&self) -> bool {
        self.lb > self.ub
    }
}

impl<T: Copy + Sub<Output = T>> Interval<T> {
    #[inline]
    pub fn length(&self) -> T {
        self.ub - self.lb
    }
}

impl<T> Display for Interval<T>
where
    T: PartialOrd + Copy + Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}, {}]", self.lb, self.ub)
    }
}

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
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.lb -= rhs;
        self.ub -= rhs;
    }
}

impl<T> Sub<T> for Interval<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Interval<T>;

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

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb * rhs,
            ub: self.ub * rhs,
            _marker: self._marker,
        }
    }
}

pub trait Enlarge<T: ?Sized> {
    type Output;

    fn enlarge_with(&self, alpha: T) -> Self::Output;
}

impl Enlarge<i32> for i32 {
    type Output = Interval<i32>;

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
    #[inline]
    fn overlaps(&self, other: &Interval<T>) -> bool {
        self.ub >= other.lb && other.ub >= self.lb
    }
}

impl<T: PartialOrd> Overlap<T> for Interval<T> {
    #[inline]
    fn overlaps(&self, other: &T) -> bool {
        self.ub >= *other && *other >= self.lb
    }
}

impl<T: PartialOrd> Overlap<Interval<T>> for T {
    #[inline]
    fn overlaps(&self, other: &Interval<T>) -> bool {
        *self >= other.lb && other.ub >= *self
    }
}

/// The `impl<T: PartialOrd> Contain<Interval<T>> for Interval<T>` block is implementing the `Contain`
/// trait for the `Interval<T>` struct.
impl<T: PartialOrd> Contain<Interval<T>> for Interval<T> {
    #[inline]
    fn contains(&self, other: &Interval<T>) -> bool {
        self.lb <= other.lb && other.ub <= self.ub
    }
}

/// The `impl<T: PartialOrd> Contain<T> for Interval<T>` block is implementing the `Contain` trait for
/// the `Interval<T>` struct.
impl<T: PartialOrd> Contain<T> for Interval<T> {
    #[inline]
    fn contains(&self, other: &T) -> bool {
        self.lb <= *other && *other <= self.ub
    }
}

impl<T: PartialOrd> Contain<Interval<T>> for T {
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

    #[inline]
    fn displace(&self, other: &Interval<T>) -> Self::Output {
        Self::Output::new(self.lb.displace(&other.lb), self.ub.displace(&other.ub))
    }
}

impl MinDist<Interval<i32>> for Interval<i32> {
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

pub trait Hull<T: ?Sized> {
    type Output;

    fn hull_with(&self, other: &T) -> Self::Output;
}

impl Hull<i32> for i32 {
    type Output = Interval<i32>;

    #[inline]
    fn hull_with(&self, other: &i32) -> Self::Output {
        Self::Output {
            lb: (*self).min(*other),
            ub: (*self).max(*other),
            _marker: PhantomData,
        }
    }
}

impl<T> Hull<Interval<T>> for Interval<T>
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

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

    #[inline]
    fn hull_with(&self, other: &Interval<T>) -> Self::Output {
        other.hull_with(self)
        // Self::Output {
        //     lb: (*self).min(other.lb),
        //     ub: (*self).max(other.ub),
        //     _marker: PhantomData,
        // }
    }
}

pub trait Intersect<T: ?Sized> {
    type Output;

    fn intersect_with(&self, other: &T) -> Self::Output;
}

impl Intersect<i32> for i32 {
    type Output = Interval<i32>;

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

    #[inline]
    fn intersect_with(&self, other: &Interval<T>) -> Self::Output {
        other.intersect_with(self)
        // Self::Output {
        //     lb: (*self).min(other.lb),
        //     ub: (*self).max(other.ub),
        //     _marker: PhantomData,
        // }
    }
}

// pub fn overlap<T: PartialOrd>(lhs: &Interval<T>, rhs: &Interval<T>) -> bool {
//     lhs.overlaps(rhs) || rhs.overlaps(lhs) || lhs == rhs
// }

// pub fn contain<T: PartialOrd>(lhs: &Interval<T>, rhs: &Interval<T>) -> bool {
//     lhs.contains(rhs) && !rhs.contains(lhs)
// }

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

    // @given(integers(), integers(), integers(), integers(), integers())
    // #[test]
    // fn test_interval_hypo(a1: int, a2: int, b1: int, b2: int, v: int) {
    //     a = Interval::new(min(a1, a2), max(a1, a2));
    //     b = Interval::new(min(b1, b2), max(b1, b2));
    //     // c = Interval::new(min(a, b), max(a, b))  // interval of interval
    //     assert!((a - v) + v == a);
    //     assert!((b - v) + v == b);
    //     // assert (c - v) + v == c
    // }

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
        // assert!(a.intersection_with(&8) == Interval::new(8, 8));
        // assert!(a.intersection_with(b) == b);
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
