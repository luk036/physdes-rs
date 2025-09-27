use super::Vector2;
use crate::generic::{Contain, Displacement, MinDist, Overlap};
use crate::interval::{Enlarge, Hull, Intersect, Interval};
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use num_traits::Num;

#[cfg(test)]
use core::hash;

/// Generic Point struct with x and y coordinates
///
/// This struct represents a point in 2D space with coordinates of potentially different types.
/// It provides various operations and functionalities for working with points, such as
/// comparison operators, arithmetic operators, flipping, overlap checking, distance calculation, and more.
///
/// Properties:
///
/// * `xcoord`: The x-coordinate of the point
/// * `ycoord`: The y-coordinate of the point
#[derive(PartialEq, Eq, Copy, PartialOrd, Ord, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct Point<T1, T2> {
    /// x portion of the Point object
    pub xcoord: T1,
    /// y portion of the Point object
    pub ycoord: T2,
}

impl<T1, T2> Point<T1, T2> {
    /// Creates a new Point with the given x and y coordinates
    ///
    /// # Arguments
    ///
    /// * `xcoord` - The x-coordinate of the point
    /// * `ycoord` - The y-coordinate of the point
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

    /// Returns a reference to the x-coordinate
    #[inline]
    pub fn xcoord(&self) -> &T1 {
        &self.xcoord
    }

    /// Returns a reference to the y-coordinate
    #[inline]
    pub fn ycoord(&self) -> &T2 {
        &self.ycoord
    }

    /// Returns a mutable reference to the x-coordinate
    #[inline]
    pub fn xcoord_mut(&mut self) -> &mut T1 {
        &mut self.xcoord
    }

    /// Returns a mutable reference to the y-coordinate
    #[inline]
    pub fn ycoord_mut(&mut self) -> &mut T2 {
        &mut self.ycoord
    }

    /// Flips the coordinates according to xcoord-ycoord diagonal line
    ///
    /// Returns a new Point with x and y coordinates swapped
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// let p = Point::new(1, 2);
    /// assert_eq!(p.flip_xy(), Point::new(2, 1));
    /// ```
    #[inline]
    pub fn flip_xy(&self) -> Point<T2, T1>
    where
        T1: Clone,
        T2: Clone,
    {
        Point {
            xcoord: self.ycoord.clone(),
            ycoord: self.xcoord.clone(),
        }
    }

    /// Flips according to ycoord-axis (negates x-coordinate)
    ///
    /// Returns a new Point with x-coordinate negated
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// let p = Point::new(3, 4);
    /// assert_eq!(p.flip_y(), Point::new(-3, 4));
    /// ```
    #[inline]
    pub fn flip_y(&self) -> Point<T1, T2>
    where
        T1: Clone + Neg<Output = T1>,
        T2: Clone,
    {
        Point {
            xcoord: -self.xcoord.clone(),
            ycoord: self.ycoord.clone(),
        }
    }
}

impl<T1: std::fmt::Display, T2: std::fmt::Display> std::fmt::Display for Point<T1, T2> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.xcoord, self.ycoord)
    }
}

impl<T1, T2, U1, U2> Overlap<Point<U1, U2>> for Point<T1, T2>
where
    T1: Overlap<U1>,
    T2: Overlap<U2>,
{
    #[inline]
    fn overlaps(&self, other: &Point<U1, U2>) -> bool {
        self.xcoord.overlaps(&other.xcoord) && self.ycoord.overlaps(&other.ycoord)
    }
}

impl<T1, T2, U1, U2> Contain<Point<U1, U2>> for Point<T1, T2>
where
    T1: Contain<U1>,
    T2: Contain<U2>,
{
    #[inline]
    fn contains(&self, other: &Point<U1, U2>) -> bool {
        self.xcoord.contains(&other.xcoord) && self.ycoord.contains(&other.ycoord)
    }
}

impl<T1, T2, U1, U2> MinDist<Point<U1, U2>> for Point<T1, T2>
where
    T1: MinDist<U1>,
    T2: MinDist<U2>,
{
    #[inline]
    fn min_dist_with(&self, other: &Point<U1, U2>) -> u32 {
        self.xcoord.min_dist_with(&other.xcoord) + self.ycoord.min_dist_with(&other.ycoord)
    }
}

impl<T1, T2> Displacement<Point<T1, T2>> for Point<T1, T2>
where
    T1: Displacement<T1, Output = T1>,
    T2: Displacement<T2, Output = T2>,
{
    type Output = Vector2<T1, T2>;

    #[inline]
    fn displace(&self, other: &Point<T1, T2>) -> Self::Output {
        Self::Output::new(
            self.xcoord.displace(&other.xcoord),
            self.ycoord.displace(&other.ycoord),
        )
    }
}

impl<T1, T2> Hull<Point<T1, T2>> for Point<T1, T2>
where
    T1: Hull<T1>,
    T2: Hull<T2>,
{
    type Output = Point<T1::Output, T2::Output>;

    #[inline]
    fn hull_with(&self, other: &Point<T1, T2>) -> Self::Output {
        Self::Output::new(
            self.xcoord.hull_with(&other.xcoord),
            self.ycoord.hull_with(&other.ycoord),
        )
    }
}

impl<T1, T2> Intersect<Point<T1, T2>> for Point<T1, T2>
where
    T1: Intersect<T1>,
    T2: Intersect<T2>,
{
    type Output = Point<T1::Output, T2::Output>;

    #[inline]
    fn intersect_with(&self, other: &Point<T1, T2>) -> Self::Output {
        Self::Output::new(
            self.xcoord.intersect_with(&other.xcoord),
            self.ycoord.intersect_with(&other.ycoord),
        )
    }
}

impl<T1, T2, Alpha> Enlarge<Alpha> for Point<T1, T2>
where
    T1: Enlarge<Alpha, Output = Interval<T1>> + Copy,
    T2: Enlarge<Alpha, Output = Interval<T2>> + Copy,
    Alpha: Copy,
{
    type Output = Point<Interval<T1>, Interval<T2>>;

    fn enlarge_with(&self, alpha: Alpha) -> Self::Output {
        Self::Output::new(
            self.xcoord.enlarge_with(alpha),
            self.ycoord.enlarge_with(alpha),
        )
    }
}

// Macro implementations for arithmetic operations
macro_rules! forward_xf_xf_binop {
    (impl $imp:ident, $method:ident, $output:ty) => {
        impl<'a, 'b, T1: Clone + Num, T2: Clone + Num> $imp<&'b Vector2<T1, T2>>
            for &'a Point<T1, T2>
        {
            type Output = $output;

            #[inline]
            fn $method(self, other: &Vector2<T1, T2>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    };
}

macro_rules! forward_xf_val_binop {
    (impl $imp:ident, $method:ident, $output:ty) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<Vector2<T1, T2>> for &'a Point<T1, T2> {
            type Output = $output;

            #[inline]
            fn $method(self, other: Vector2<T1, T2>) -> Self::Output {
                self.clone().$method(other)
            }
        }
    };
}

macro_rules! forward_val_xf_binop {
    (impl $imp:ident, $method:ident, $output:ty) => {
        impl<'a, T1: Clone + Num, T2: Clone + Num> $imp<&'a Vector2<T1, T2>> for Point<T1, T2> {
            type Output = $output;

            #[inline]
            fn $method(self, other: &Vector2<T1, T2>) -> Self::Output {
                self.$method(other.clone())
            }
        }
    };
}

macro_rules! forward_all_binop {
    (impl $imp:ident, $method:ident, $output:ty) => {
        forward_xf_xf_binop!(impl $imp, $method, $output);
        forward_xf_val_binop!(impl $imp, $method, $output);
        forward_val_xf_binop!(impl $imp, $method, $output);
    };
}

forward_all_binop!(impl Add, add, Point<T1, T2>);

impl<T1: Clone + Num, T2: Clone + Num> Add<Vector2<T1, T2>> for Point<T1, T2> {
    type Output = Self;

    /// Translate a point by a vector
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

forward_all_binop!(impl Sub, sub, Point<T1, T2>);

impl<T1: Clone + Num, T2: Clone + Num> Sub<Vector2<T1, T2>> for Point<T1, T2> {
    type Output = Self;

    /// Translate a point by a vector (subtraction)
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

// Macro implementations for point-to-point subtraction
impl<T1: Clone + Num, T2: Clone + Num> Sub for Point<T1, T2> {
    type Output = Vector2<T1, T2>;

    /// Calculate displacement vector between two points
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
    /// ```
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.xcoord - other.xcoord, self.ycoord - other.ycoord)
    }
}

// Assignment operations
impl<T1: Clone + Num + AddAssign, T2: Clone + Num + AddAssign> AddAssign<Vector2<T1, T2>>
    for Point<T1, T2>
{
    #[inline]
    fn add_assign(&mut self, other: Vector2<T1, T2>) {
        self.xcoord += other.x_;
        self.ycoord += other.y_;
    }
}

impl<T1: Clone + Num + SubAssign, T2: Clone + Num + SubAssign> SubAssign<Vector2<T1, T2>>
    for Point<T1, T2>
{
    #[inline]
    fn sub_assign(&mut self, other: Vector2<T1, T2>) {
        self.xcoord -= other.x_;
        self.ycoord -= other.y_;
    }
}

impl<'a, T1: Clone + Num + AddAssign, T2: Clone + Num + AddAssign> AddAssign<&'a Vector2<T1, T2>>
    for Point<T1, T2>
{
    #[inline]
    fn add_assign(&mut self, other: &'a Vector2<T1, T2>) {
        self.xcoord += other.x_.clone();
        self.ycoord += other.y_.clone();
    }
}

impl<'a, T1: Clone + Num + SubAssign, T2: Clone + Num + SubAssign> SubAssign<&'a Vector2<T1, T2>>
    for Point<T1, T2>
{
    #[inline]
    fn sub_assign(&mut self, other: &'a Vector2<T1, T2>) {
        self.xcoord -= other.x_.clone();
        self.ycoord -= other.y_.clone();
    }
}

// Negation
impl<T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg for Point<T1, T2> {
    type Output = Self;

    /// Negate a Point
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    ///
    /// assert_eq!(-Point::new(3, 4), Point::new(-3, -4));
    /// assert_eq!(-Point::new(0, 0), Point::new(0, 0));
    /// ```
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.xcoord, -self.ycoord)
    }
}

impl<T1: Clone + Num + Neg<Output = T1>, T2: Clone + Num + Neg<Output = T2>> Neg
    for &Point<T1, T2>
{
    type Output = Point<T1, T2>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

#[cfg(test)]
pub fn hash<T: hash::Hash>(x: &T) -> u64 {
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
    use crate::generic::{Contain, Overlap};
    use crate::interval::Interval;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

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
    pub const _4_2p: Point<i32, i32> = Point {
        xcoord: 4,
        ycoord: 2,
    };

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_construction_and_accessors() {
        let p1 = Point::new(1, 2);
        assert_eq!(p1.xcoord(), &1);
        assert_eq!(p1.ycoord(), &2);
    }

    #[test]
    fn test_comparison() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(1, 2);
        let p3 = Point::new(2, 3);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert!(p1 < p3);
    }

    #[test]
    fn test_arithmetic_operators() {
        let p1 = Point::new(1, 2);
        let v = Vector2::new(1, 1);
        let p4 = p1 + v;
        assert_eq!(p4, Point::new(2, 3));

        let mut p5 = p4;
        p5 -= v;
        assert_eq!(p5, p1);

        let p6 = p4 - v;
        assert_eq!(p6, p1);
    }

    #[test]
    fn test_flip() {
        let p1 = Point::new(1, 2);
        let p_flipped = p1.flip_xy();
        assert_eq!(p_flipped, Point::new(2, 1));

        let p_flipped_y = p1.flip_y();
        assert_eq!(p_flipped_y, Point::new(-1, 2));
    }

    #[test]
    fn test_overlaps_contains_intersects_hull() {
        let p_interval1 = Point::new(Interval::new(0, 2), Interval::new(0, 2));
        let p_interval2 = Point::new(Interval::new(1, 3), Interval::new(1, 3));
        let p_interval3 = Point::new(Interval::new(3, 4), Interval::new(3, 4));

        assert!(p_interval1.overlaps(&p_interval2));
        assert!(!p_interval1.overlaps(&p_interval3));

        assert!(p_interval1.contains(&Point::new(1, 1)));
        assert!(!p_interval1.contains(&Point::new(3, 3)));

        let intersection = p_interval1.intersect_with(&p_interval2);
        assert_eq!(intersection.xcoord, Interval::new(1, 2));
        assert_eq!(intersection.ycoord, Interval::new(1, 2));

        let hull = p_interval1.hull_with(&p_interval2);
        assert_eq!(hull.xcoord, Interval::new(0, 3));
        assert_eq!(hull.ycoord, Interval::new(0, 3));
    }

    #[test]
    fn test_min_distance() {
        let p_interval1 = Point::new(Interval::new(0, 2), Interval::new(0, 2));
        let p_interval2 = Point::new(Interval::new(4, 5), Interval::new(4, 5));

        let dist = p_interval1.min_dist_with(&p_interval2);
        assert_eq!(dist, 4);
    }

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
        assert_eq!(a.flip_xy(), Point::new(5, 3));
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
        let a = Point::new(3, 5);
        let b: Point<Interval<i32>, Interval<i32>> = a.enlarge_with(2);
        assert_eq!(b, Point::new(Interval::new(1, 5), Interval::new(3, 7)));
    }

    #[test]
    fn test_displace_more_cases() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(a.displace(&b), Vector2::new(-3, -4));
        let c = Point::new(-3, -4);
        assert_eq!(a.displace(&c), Vector2::new(3, 4));
    }

    #[test]
    fn test_hull_more_cases() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(
            a.hull_with(&b),
            Point::new(Interval::new(0, 3), Interval::new(0, 4))
        );
        let c = Point::new(-3, -4);
        assert_eq!(
            a.hull_with(&c),
            Point::new(Interval::new(-3, 0), Interval::new(-4, 0))
        );
    }

    #[test]
    fn test_intersect_with_more_cases() {
        let p1 = Point::new(Interval::new(0, 5), Interval::new(0, 5));
        let p2 = Point::new(Interval::new(3, 8), Interval::new(3, 8));
        assert_eq!(
            p1.intersect_with(&p2),
            Point::new(Interval::new(3, 5), Interval::new(3, 5))
        );

        let p3 = Point::new(Interval::new(6, 8), Interval::new(6, 8));
        assert!(p1.intersect_with(&p3).xcoord.is_invalid());
        assert!(p1.intersect_with(&p3).ycoord.is_invalid());
    }

    #[test]
    fn test_overlaps_more_cases() {
        let p1 = Point::new(Interval::new(0, 5), Interval::new(0, 5));
        let p2 = Point::new(Interval::new(5, 8), Interval::new(5, 8));
        assert!(p1.overlaps(&p2));

        let p3 = Point::new(Interval::new(6, 8), Interval::new(6, 8));
        assert!(!p1.overlaps(&p3));
    }

    #[test]
    fn test_contains_more_cases() {
        let p1 = Point::new(Interval::new(0, 10), Interval::new(0, 10));
        let p2 = Point::new(Interval::new(3, 8), Interval::new(3, 8));
        assert!(p1.contains(&p2));

        let p3 = Point::new(Interval::new(3, 12), Interval::new(3, 8));
        assert!(!p1.contains(&p3));
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

    #[test]
    fn test_flip_xy() {
        let p1 = Point::new(1, 2);
        assert_eq!(p1.flip_xy(), Point::new(2, 1));
    }

    #[test]
    fn test_display() {
        let p1 = Point::new(1, 2);
        assert_eq!(format!("{}", p1), "(1, 2)");
    }

    #[test]
    fn test_displace_more() {
        let a = Point::new(3, 5);
        let b = Point::new(-5, 7);
        let c = Point::new(7, -8);
        assert_eq!(a.displace(&b), Vector2::new(8, -2));
        assert_eq!(a.displace(&c), Vector2::new(-4, 13));
        assert_eq!(b.displace(&c), Vector2::new(-12, 15));
    }

    #[test]
    fn test_hull_more() {
        let a = Point::new(3, 5);
        let b = Point::new(5, 7);
        let c = Point::new(-1, 9);
        assert_eq!(
            a.hull_with(&b),
            Point::new(Interval::new(3, 5), Interval::new(5, 7))
        );
        assert_eq!(
            a.hull_with(&c),
            Point::new(Interval::new(-1, 3), Interval::new(5, 9))
        );
    }

    #[test]
    fn test_intersect_with() {
        let p1 = Point::new(Interval::new(0, 5), Interval::new(0, 5));
        let p2 = Point::new(Interval::new(3, 8), Interval::new(3, 8));
        let p3 = Point::new(Interval::new(10, 12), Interval::new(10, 12));

        assert_eq!(
            p1.intersect_with(&p2),
            Point::new(Interval::new(3, 5), Interval::new(3, 5))
        );
        assert_eq!(
            p1.intersect_with(&p3),
            Point::new(Interval::new(10, 5), Interval::new(10, 5))
        );
    }

    #[test]
    fn test_overlaps_more() {
        let p1 = Point::new(Interval::new(0, 5), Interval::new(0, 5));
        let p2 = Point::new(Interval::new(3, 8), Interval::new(3, 8));
        let p3 = Point::new(Interval::new(6, 8), Interval::new(6, 8));

        assert!(p1.overlaps(&p2));
        assert!(!p1.overlaps(&p3));
    }

    #[test]
    fn test_contains_more() {
        let p1 = Point::new(Interval::new(0, 10), Interval::new(0, 10));
        let p2 = Point::new(Interval::new(3, 8), Interval::new(3, 8));
        let p3 = Point::new(Interval::new(6, 12), Interval::new(6, 12));

        assert!(p1.contains(&p2));
        assert!(!p1.contains(&p3));
    }
}
