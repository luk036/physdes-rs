use crate::generic::MinDist;
use crate::interval::{Enlarge, Intersect};
use crate::point::Point;
use std::cmp;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct MergeObj<T1, T2> {
    impl_: Point<T1, T2>,
}

impl<T1, T2> MergeObj<T1, T2> {
    pub const fn new(xcoord: T1, ycoord: T2) -> MergeObj<T1, T2> {
        MergeObj {
            impl_: Point::new(xcoord, ycoord),
        }
    }

    pub const fn construct(xcoord: i32, ycoord: i32) -> MergeObj<i32, i32> {
        let impl_ = Point::new(xcoord + ycoord, xcoord - ycoord);
        MergeObj { impl_ }
    }
}

impl<T1, T2> MergeObj<T1, T2>
where
    T1: MinDist<T1>,
    T2: MinDist<T2>,
{
    pub fn min_dist_with(&self, other: &MergeObj<T1, T2>) -> u32 {
        cmp::max(
            self.impl_.xcoord.min_dist_with(&other.impl_.xcoord),
            self.impl_.ycoord.min_dist_with(&other.impl_.ycoord),
        )
    }
}

impl<T1, T2> MergeObj<T1, T2>
where
    T1: MinDist<T1> + Enlarge<i32, Output = T1> + Intersect<T1, Output = T1>,
    T2: MinDist<T2> + Enlarge<i32, Output = T2> + Intersect<T2, Output = T2>,
{
    pub fn enlarge_with(&self, alpha: i32) -> MergeObj<T1, T2> {
        let xcoord = self.impl_.xcoord.enlarge_with(alpha);
        let ycoord = self.impl_.ycoord.enlarge_with(alpha);
        MergeObj::new(xcoord, ycoord)
    }

    pub fn intersect_with(&self, other: &MergeObj<T1, T2>) -> MergeObj<T1, T2> {
        let point = self.impl_.intersect_with(&other.impl_);
        MergeObj::new(point.xcoord, point.ycoord)
    }

    pub fn merge_with(&self, other: &MergeObj<T1, T2>) -> MergeObj<T1, T2> {
        let alpha = self.min_dist_with(other);
        let half = alpha / 2;
        let trr1 = self.enlarge_with(half as i32);
        let trr2 = other.enlarge_with((alpha - half) as i32);
        trr1.intersect_with(&trr2)
    }
}

#[cfg(test)]
mod test {
    // #![allow(non_upper_case_globals)]

    use super::*;
    use crate::interval::Interval;
    use crate::vector2::Vector2;

    // use crate::generic::Overlap;
    // use crate::interval::Interval;

    // use core::i32;

    #[test]
    fn test_merge_obj() {
        let r1 = MergeObj::<i32, i32>::construct(4, 5);
        let r2 = MergeObj::<i32, i32>::construct(7, 9);

        assert_ne!(r1, r2);
        assert_eq!(r1.min_dist_with(&r2), 7);
        // assert_eq!(min_dist(&r1, &r2), 7);
    }

    #[test]
    fn test_merge() {
        let s1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(200, 600), Interval::new(200, 600));
        let s2: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(500, 900), Interval::new(500, 900));
        let m1 = s1.merge_with(&s2);
        println!("{:?}", m1);
        assert_eq!(
            m1,
            MergeObj::new(Interval::new(500, 600), Interval::new(500, 600))
        );
    }

    #[test]
    fn test_merge_2() {
        let mut a: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(4, 5), Interval::new(4, 5));
        let b: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(7, 9), Interval::new(7, 9));
        let v = Vector2::new(Interval::new(2, 3), Interval::new(2, 3));
        a.impl_.xcoord.lb += v.x_.lb;
        a.impl_.xcoord.ub += v.x_.ub;
        a.impl_.ycoord.lb += v.y_.lb;
        a.impl_.ycoord.ub += v.y_.ub;
        a.impl_.xcoord.lb -= v.x_.lb;
        a.impl_.xcoord.ub -= v.x_.ub;
        a.impl_.ycoord.lb -= v.y_.lb;
        a.impl_.ycoord.ub -= v.y_.ub;
        assert_eq!(a, MergeObj::new(Interval::new(4, 5), Interval::new(4, 5)));
        let r1 = a.enlarge_with(3);
        assert_eq!(r1, MergeObj::new(Interval::new(1, 8), Interval::new(1, 8)));
        let r2 = b.enlarge_with(4);
        assert_eq!(
            r2,
            MergeObj::new(Interval::new(3, 13), Interval::new(3, 13))
        );
        let r3 = r1.intersect_with(&r2);
        assert_eq!(r3, MergeObj::new(Interval::new(3, 8), Interval::new(3, 8)));
    }

    #[test]
    fn test_min_dist_with_more_cases() {
        let r1 = MergeObj::<i32, i32>::construct(0, 0);
        let r2 = MergeObj::<i32, i32>::construct(3, 4);
        assert_eq!(r1.min_dist_with(&r2), 7);

        let r3 = MergeObj::<i32, i32>::construct(-3, -4);
        assert_eq!(r1.min_dist_with(&r3), 7);
    }

    #[test]
    fn test_enlarge_with_more_cases() {
        let s1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(200, 600), Interval::new(200, 600));
        let enlarged = s1.enlarge_with(100);
        assert_eq!(
            enlarged,
            MergeObj::new(Interval::new(100, 700), Interval::new(100, 700))
        );
    }

    #[test]
    fn test_intersect_with_more_cases() {
        let s1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(200, 600), Interval::new(200, 600));
        let s2: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(500, 900), Interval::new(500, 900));
        let intersected = s1.intersect_with(&s2);
        assert_eq!(
            intersected,
            MergeObj::new(Interval::new(500, 600), Interval::new(500, 600))
        );

        let s3 = MergeObj::new(Interval::new(700, 900), Interval::new(700, 900));
        let intersected2 = s1.intersect_with(&s3);
        assert!(intersected2.impl_.xcoord.is_invalid());
        assert!(intersected2.impl_.ycoord.is_invalid());
    }

    #[test]
    fn test_merge_with_more_cases() {
        let s1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(0, 100), Interval::new(0, 100));
        let s2: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(100, 200), Interval::new(100, 200));
        let merged = s1.merge_with(&s2);
        assert_eq!(
            merged,
            MergeObj::new(Interval::new(100, 100), Interval::new(100, 100))
        );
    }
}
