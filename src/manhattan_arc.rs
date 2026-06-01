use std::fmt;
use std::ops::{Add, Neg, Sub};

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

impl<T: Copy + Neg<Output = T>> ManhattanArc<T> {
    pub fn from_point(pt: Point<T, T>) -> Self {
        ManhattanArc {
            impl_p: Point::new(-pt.ycoord, pt.xcoord),
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
        let pt_arc = ManhattanArc::new(
            Interval::new(ms.xcoord(), ms.xcoord()),
            Interval::new(ms.ycoord(), ms.ycoord()),
        );
        let distance = self.min_dist_with(&pt_arc) as i32;
        let trr = self.enlarge_with(distance);

        let lb = Point::new(self.impl_p.xcoord.lb, self.impl_p.ycoord.lb);
        let ub = Point::new(self.impl_p.xcoord.ub, self.impl_p.ycoord.ub);
        let center = Point::new(
            (self.impl_p.xcoord.lb + self.impl_p.xcoord.ub) / 2,
            (self.impl_p.ycoord.lb + self.impl_p.ycoord.ub) / 2,
        );

        let mut m = center;
        if trr.impl_p.xcoord.lb <= lb.xcoord
            && lb.xcoord <= trr.impl_p.xcoord.ub
            && trr.impl_p.ycoord.lb <= lb.ycoord
            && lb.ycoord <= trr.impl_p.ycoord.ub
        {
            m = lb;
        } else if trr.impl_p.xcoord.lb <= ub.xcoord
            && ub.xcoord <= trr.impl_p.xcoord.ub
            && trr.impl_p.ycoord.lb <= ub.ycoord
            && ub.ycoord <= trr.impl_p.ycoord.ub
        {
            m = ub;
        }
        m
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
        assert_eq!(arc.xcoord(), -4);
        assert_eq!(arc.ycoord(), 3);
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
}
