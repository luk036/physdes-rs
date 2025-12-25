use crate::generic::MinDist;
use crate::interval::{Enlarge, Intersect};
use crate::point::Point;
use std::cmp;

/// Represents a merge object that encapsulates a point with coordinates of type T1 and T2.
///
/// The MergeObj struct is used for geometric operations such as distance calculation,
/// enlargement, intersection, and merging with other merge objects.
///
/// # Examples
///
/// ```
/// use physdes::merge_obj::MergeObj;
///
/// let merge_obj = MergeObj::new(3, 4);
/// let internal_point = merge_obj.get_impl();
/// assert_eq!(internal_point.xcoord, 3);
/// assert_eq!(internal_point.ycoord, 4);
/// ```
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

    /// Returns a reference to the internal Point of the MergeObj
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::merge_obj::MergeObj;
    ///
    /// let merge_obj = MergeObj::new(3, 4);
    /// let internal_point = merge_obj.get_impl();
    /// assert_eq!(internal_point.xcoord, 3);
    /// assert_eq!(internal_point.ycoord, 4);
    /// ```
    pub fn get_impl(&self) -> &Point<T1, T2> {
        &self.impl_
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
        let obj1 = MergeObj::<i32, i32>::construct(4, 5);
        let obj2 = MergeObj::<i32, i32>::construct(7, 9);

        assert_ne!(obj1, obj2);
        assert_eq!(obj1.min_dist_with(&obj2), 7);
        // assert_eq!(min_dist(&obj1, &obj2), 7);
    }

    #[test]
    fn test_merge() {
        let obj1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(200, 600), Interval::new(200, 600));
        let obj2: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(500, 900), Interval::new(500, 900));
        let merged = obj1.merge_with(&obj2);
        println!("{:?}", merged);
        assert_eq!(
            merged,
            MergeObj::new(Interval::new(500, 600), Interval::new(500, 600))
        );
    }

    #[test]
    fn test_merge_2() {
        let mut obj1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(4, 5), Interval::new(4, 5));
        let obj2: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(7, 9), Interval::new(7, 9));
        let vec = Vector2::new(Interval::new(2, 3), Interval::new(2, 3));
        obj1.impl_.xcoord.lb += vec.x_.lb;
        obj1.impl_.xcoord.ub += vec.x_.ub;
        obj1.impl_.ycoord.lb += vec.y_.lb;
        obj1.impl_.ycoord.ub += vec.y_.ub;
        obj1.impl_.xcoord.lb -= vec.x_.lb;
        obj1.impl_.xcoord.ub -= vec.x_.ub;
        obj1.impl_.ycoord.lb -= vec.y_.lb;
        obj1.impl_.ycoord.ub -= vec.y_.ub;
        assert_eq!(obj1, MergeObj::new(Interval::new(4, 5), Interval::new(4, 5)));
        let result1 = obj1.enlarge_with(3);
        assert_eq!(result1, MergeObj::new(Interval::new(1, 8), Interval::new(1, 8)));
        let result2 = obj2.enlarge_with(4);
        assert_eq!(
            result2,
            MergeObj::new(Interval::new(3, 13), Interval::new(3, 13))
        );
        let result3 = result1.intersect_with(&result2);
        assert_eq!(result3, MergeObj::new(Interval::new(3, 8), Interval::new(3, 8)));
    }

    #[test]
    fn test_min_dist_with_more_cases() {
        let obj1 = MergeObj::<i32, i32>::construct(0, 0);
        let obj2 = MergeObj::<i32, i32>::construct(3, 4);
        assert_eq!(obj1.min_dist_with(&obj2), 7);

        let obj3 = MergeObj::<i32, i32>::construct(-3, -4);
        assert_eq!(obj1.min_dist_with(&obj3), 7);
    }

    #[test]
    fn test_enlarge_with_more_cases() {
        let obj1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(200, 600), Interval::new(200, 600));
        let enlarged = obj1.enlarge_with(100);
        assert_eq!(
            enlarged,
            MergeObj::new(Interval::new(100, 700), Interval::new(100, 700))
        );
    }

    #[test]
    fn test_intersect_with_more_cases() {
        let obj1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(200, 600), Interval::new(200, 600));
        let obj2: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(500, 900), Interval::new(500, 900));
        let intersected = obj1.intersect_with(&obj2);
        assert_eq!(
            intersected,
            MergeObj::new(Interval::new(500, 600), Interval::new(500, 600))
        );

        let obj3 = MergeObj::new(Interval::new(700, 900), Interval::new(700, 900));
        let intersected2 = obj1.intersect_with(&obj3);
        assert!(intersected2.impl_.xcoord.is_invalid());
        assert!(intersected2.impl_.ycoord.is_invalid());
    }

    #[test]
    fn test_merge_with_more_cases() {
        let obj1: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(0, 100), Interval::new(0, 100));
        let obj2: MergeObj<Interval<i32>, Interval<i32>> =
            MergeObj::new(Interval::new(100, 200), Interval::new(100, 200));
        let merged = obj1.merge_with(&obj2);
        assert_eq!(
            merged,
            MergeObj::new(Interval::new(100, 100), Interval::new(100, 100))
        );
    }
}
