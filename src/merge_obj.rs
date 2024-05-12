// use super::Vector2;
use super::Point;
// use crate::generic::{Contain, Displacement, MinDist, Overlap};
use crate::generic::MinDist;
#[cfg(any(test, feature = "std"))]
#[cfg(test)]
// use core::hash;
// use core::ops::{Add, Neg, Sub};
// use num_traits::Num;
// use std::cmp::Ordering;
use std::cmp;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct MergeObj<T1, T2> {
    impl_: Point<T1, T2>,
}

impl<T1, T2> MergeObj<T1, T2> {
    pub fn new(xcoord: T1, ycoord: T2) -> MergeObj<T1, T2> {
        MergeObj {
            impl_: Point::new(xcoord, ycoord),
        }
    }

    pub fn construct(xcoord: i32, ycoord: i32) -> MergeObj<i32, i32> {
        let impl_ = Point::new(xcoord + ycoord, xcoord - ycoord);
        MergeObj { impl_ }
    }
}

impl<T1: MinDist<T1>, T2: MinDist<T2>> MergeObj<T1, T2> {
    pub fn min_dist_with(&self, other: &MergeObj<T1, T2>) -> u32 {
        cmp::max(
            self.impl_.xcoord.min_dist_with(&other.impl_.xcoord),
            self.impl_.ycoord.min_dist_with(&other.impl_.ycoord),
        )
    }

    // fn enlarge_with(&self, alpha: i32) -> MergeObj<T1, T2> {
    //     let xcoord = enlarge(self.impl_.xcoord, alpha);
    //     let ycoord = enlarge(self.impl_.ycoord, alpha);
    //     MergeObj::new(xcoord, ycoord)
    // }
    //
    // fn intersection_with(&self, other: &MergeObj<T1, T2>) -> MergeObj<T1, T2> {
    //     let point = self.impl_.intersection_with(&other.impl_);
    //     MergeObj::new(point.xcoord, point.ycoord)
    // }
    //
    // fn merge_with(&self, other: &MergeObj<T1, T2>) -> MergeObj<T1, T2> {
    //     let alpha = self.min_dist_with(other);
    //     let half = alpha / 2;
    //     let trr1 = enlarge(&self.impl_, half);
    //     let trr2 = enlarge(&other.impl_, alpha - half);
    //     let impl_ = intersection(&trr1, &trr2);
    //     MergeObj::new(impl_.xcoord, impl_.ycoord)
    // }
}


#[cfg(test)]
mod test {
    // #![allow(non_upper_case_globals)]

    use super::*;
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

    // #[test]
    // fn test_merge() {
    //     let s1 = MergeObj::new(200 + 600, 200 - 600);
    //     let s2 = MergeObj::new(500 + 900, 500 - 900);
    //     let m1 = s1.merge_with(&s2);
    //     println!("{:?}", m1);
    //     assert_eq!(m1, MergeObj::new(Interval::new(1100, 1100), Interval::new(-700, -100)));
    // }
    //
    // #[test]
    // fn test_merge_2() {
    //     let mut a = MergeObj::new(4 + 5, 4 - 5);
    //     let b = MergeObj::new(7 + 9, 7 - 9);
    //     let v = Vector2(2, 3);
    //     a += &v;
    //     a -= &v;
    //     assert_eq!(a, MergeObj::new(4 + 5, 4 - 5));
    //     let r1 = a.enlarge_with(3);
    //     assert_eq!(r1, MergeObj::new(Interval::new(6, 12), Interval::new(-4, 2)));
    //     let r2 = b.enlarge_with(4);
    //     assert_eq!(r2, MergeObj::new(Interval::new(12, 20), Interval::new(-6, 2)));
    //     let r3 = r1.intersection_with(&r2);
    //     assert_eq!(r3, MergeObj::new(Interval::new(12, 12), Interval::new(-4, 2)));
    // }
}
