use std::fmt;
use std::ops::{Add, Sub};

use crate::generic::MinDist;
use crate::interval::Interval;
use crate::point::Point;

/// A Manhattan arc (merging segment) for the DME algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManhattanArc<T> {
    pub impl_p: Point<T, T>,
}

impl<T: Default> Default for ManhattanArc<T> {
    fn default() -> Self {
        ManhattanArc {
            impl_p: Point::new(T::default(), T::default()),
        }
    }
}

impl<T> ManhattanArc<T> {
    #[inline]
    pub const fn new(xcoord: T, ycoord: T) -> Self {
        ManhattanArc {
            impl_p: Point::new(xcoord, ycoord),
        }
    }
}

impl<T: Copy> ManhattanArc<T> {
    #[inline]
    pub fn xcoord(&self) -> T {
        self.impl_p.xcoord
    }
    #[inline]
    pub fn ycoord(&self) -> T {
        self.impl_p.ycoord
    }
}

impl<T: Copy + Sub<Output = T> + Add<Output = T>> ManhattanArc<T> {
    /// Converts a point from normal (Cartesian) space to the rotated (Manhattan) space.
    /// Uses the same transformation as the Python reference: (x, y) -> (x - y, x + y).
    pub fn from_point(pt: Point<T, T>) -> Self {
        ManhattanArc {
            impl_p: Point::new(pt.xcoord - pt.ycoord, pt.xcoord + pt.ycoord),
        }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> ManhattanArc<T> {
    pub fn construct(xcoord: T, ycoord: T) -> Self {
        ManhattanArc {
            impl_p: Point::new(xcoord - ycoord, xcoord + ycoord),
        }
    }
}

impl<T: fmt::Display> fmt::Display for ManhattanArc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}, {}/", self.impl_p.xcoord, self.impl_p.ycoord)
    }
}

// --- i32 implementations ---

impl ManhattanArc<i32> {
    pub fn min_dist_with(&self, other: &Self) -> u32 {
        let dx = self.impl_p.xcoord.min_dist_with(&other.impl_p.xcoord);
        let dy = self.impl_p.ycoord.min_dist_with(&other.impl_p.ycoord);
        dx.max(dy)
    }

    pub fn enlarge_with(&self, alpha: i32) -> ManhattanArc<Interval<i32>> {
        let x = Interval::new(self.impl_p.xcoord - alpha, self.impl_p.xcoord + alpha);
        let y = Interval::new(self.impl_p.ycoord - alpha, self.impl_p.ycoord + alpha);
        ManhattanArc::new(x, y)
    }

    pub fn nearest_point_to(&self, other: &Point<i32, i32>) -> Point<i32, i32> {
        *other
    }

    pub fn merge_with(&self, other: &Self, alpha: i32) -> ManhattanArc<Interval<i32>> {
        let distance = self.min_dist_with(other) as i32;
        let trr1 = self.enlarge_with(alpha);
        let trr2 = other.enlarge_with(distance - alpha);
        let x_intersect = Interval::new(
            trr1.xcoord().lb.max(trr2.xcoord().lb),
            trr1.xcoord().ub.min(trr2.xcoord().ub),
        );
        let y_intersect = Interval::new(
            trr1.ycoord().lb.max(trr2.ycoord().lb),
            trr1.ycoord().ub.min(trr2.ycoord().ub),
        );
        ManhattanArc::new(x_intersect, y_intersect)
    }
}

// --- Interval<i32> implementations ---

impl ManhattanArc<Interval<i32>> {
    /// Returns the upper corner of the merging segment in normal (Cartesian) coordinates.
    /// Upper corner in rotated space is (xcoord.ub, ycoord.ub), then transformed back
    /// to normal space using the inverse of (x-y, x+y): (rx, ry) -> ((rx+ry)/2, (ry-rx)/2).
    pub fn get_upper_corner(&self) -> Point<i32, i32> {
        let rx = self.impl_p.xcoord.ub;
        let ry = self.impl_p.ycoord.ub;
        Point::new((rx + ry) / 2, (ry - rx) / 2)
    }

    pub fn min_dist_with(&self, other: &Self) -> u32 {
        let dx = self.impl_p.xcoord.min_dist_with(&other.impl_p.xcoord);
        let dy = self.impl_p.ycoord.min_dist_with(&other.impl_p.ycoord);
        dx.max(dy)
    }

    fn enlarge_interval(iv: Interval<i32>, alpha: i32) -> Interval<i32> {
        Interval::new(iv.lb - alpha, iv.ub + alpha)
    }

    pub fn enlarge_with(&self, alpha: i32) -> Self {
        ManhattanArc::new(
            Self::enlarge_interval(self.impl_p.xcoord, alpha),
            Self::enlarge_interval(self.impl_p.ycoord, alpha),
        )
    }

    pub fn nearest_point_to(&self, other: &Point<i32, i32>) -> Point<i32, i32> {
        let ms = ManhattanArc::from_point(*other);
        // Clip the query point in rotated space to the segment bounds.
        // This matches the Python Point::nearest_to semantics.
        let rx = ms
            .xcoord()
            .clamp(self.impl_p.xcoord.lb, self.impl_p.xcoord.ub);
        let ry = ms
            .ycoord()
            .clamp(self.impl_p.ycoord.lb, self.impl_p.ycoord.ub);
        // Convert back from rotated space to normal space.
        // Inverse of (x-y, x+y): (rx, ry) -> ((rx+ry)/2, (ry-rx)/2)
        Point::new((rx + ry) / 2, (ry - rx) / 2)
    }

    pub fn merge_with(&self, other: &Self, alpha: i32) -> Self {
        let distance = self.min_dist_with(other) as i32;
        let trr1 = self.enlarge_with(alpha);
        let trr2 = other.enlarge_with(distance - alpha);
        let x_intersect = Interval::new(
            trr1.impl_p.xcoord.lb.max(trr2.impl_p.xcoord.lb),
            trr1.impl_p.xcoord.ub.min(trr2.impl_p.xcoord.ub),
        );
        let y_intersect = Interval::new(
            trr1.impl_p.ycoord.lb.max(trr2.impl_p.ycoord.lb),
            trr1.impl_p.ycoord.ub.min(trr2.impl_p.ycoord.ub),
        );
        ManhattanArc::new(x_intersect, y_intersect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_default() {
        let arc: ManhattanArc<i32> = ManhattanArc::new(5, 10);
        assert_eq!(arc.xcoord(), 5);
        assert_eq!(arc.ycoord(), 10);
    }

    #[test]
    fn test_display() {
        let arc = ManhattanArc::new(3, 7);
        assert_eq!(format!("{}", arc), "/3, 7/");
    }

    #[test]
    fn test_equality() {
        let a = ManhattanArc::new(1, 2);
        let b = ManhattanArc::new(1, 2);
        let c = ManhattanArc::new(3, 4);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_construct() {
        let arc = ManhattanArc::construct(5, 3);
        assert_eq!(arc.xcoord(), 2);
        assert_eq!(arc.ycoord(), 8);
    }

    #[test]
    fn test_from_point() {
        let pt = Point::new(3, 4);
        let arc = ManhattanArc::from_point(pt);
        assert_eq!(arc.xcoord(), -1);
        assert_eq!(arc.ycoord(), 7);
    }

    #[test]
    fn test_min_dist() {
        let a = ManhattanArc::new(0, 0);
        let b = ManhattanArc::new(3, 4);
        assert_eq!(a.min_dist_with(&b), 4);
    }

    #[test]
    fn test_enlarge_with() {
        let a = ManhattanArc::new(5, 10);
        let enlarged = a.enlarge_with(3);
        assert_eq!(enlarged.xcoord(), Interval::new(2, 8));
        assert_eq!(enlarged.ycoord(), Interval::new(7, 13));
    }

    #[test]
    fn test_default() {
        let arc: ManhattanArc<i32> = Default::default();
        assert_eq!(arc.xcoord(), 0);
        assert_eq!(arc.ycoord(), 0);
    }

    #[test]
    fn test_nearest_point_to_i32() {
        let arc = ManhattanArc::new(5, 10);
        let pt = Point::new(3, 4);
        let result = arc.nearest_point_to(&pt);
        assert_eq!(result, pt);
    }

    #[test]
    fn test_merge_with_i32() {
        let arc1 = ManhattanArc::new(0, 10);
        let arc2 = ManhattanArc::new(10, 0);
        let result = arc1.merge_with(&arc2, 2);
        assert_eq!(result.xcoord(), Interval::new(2, 2));
        assert_eq!(result.ycoord(), Interval::new(8, 8));
    }

    #[test]
    fn test_interval_enlarge_with() {
        let arc = ManhattanArc::new(Interval::new(0, 10), Interval::new(5, 15));
        let enlarged = arc.enlarge_with(2);
        assert_eq!(enlarged.xcoord(), Interval::new(-2, 12));
        assert_eq!(enlarged.ycoord(), Interval::new(3, 17));
    }

    #[test]
    fn test_interval_min_dist_with() {
        let arc1 = ManhattanArc::new(Interval::new(0, 5), Interval::new(0, 5));
        let arc2 = ManhattanArc::new(Interval::new(10, 15), Interval::new(10, 15));
        let dist = arc1.min_dist_with(&arc2);
        assert_eq!(dist, 5);
    }

    #[test]
    fn test_interval_nearest_point_to() {
        let arc = ManhattanArc::new(Interval::new(0, 10), Interval::new(0, 10));
        let pt = Point::new(5, 12);
        let result = arc.nearest_point_to(&pt);
        // pt is within or near the arc, should return one of lb/ub/center
        assert!(result.xcoord >= 0 && result.ycoord >= 0);
    }

    #[test]
    fn test_interval_merge_with() {
        let arc1 = ManhattanArc::new(Interval::new(0, 5), Interval::new(0, 5));
        let arc2 = ManhattanArc::new(Interval::new(10, 15), Interval::new(10, 15));
        let result = arc1.merge_with(&arc2, 2);
        assert!(result.xcoord().lb <= result.xcoord().ub);
    }
}
