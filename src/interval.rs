use crate::generic::{Contain, Overlap};
use std::cmp::PartialOrd;
use std::marker::PhantomData;

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
#[derive(Debug)]
pub struct Interval<T: PartialOrd> {
    pub lb: T,
    pub ub: T,
    pub _marker: PhantomData<T>,
}

impl<T: PartialOrd> Interval<T> {
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
    pub fn new(lb: T, ub: T) -> Self {
        Self {
            lb,
            ub,
            _marker: PhantomData,
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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ub.partial_cmp(&other.lb)
    }
}

impl<T: PartialOrd> PartialEq for Interval<T> {
    /// The function checks if two objects have equal values for their "lb" and "ub" fields.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter is a reference to another object of the same type as `Self`. In
    /// this case, `Self` refers to the type of the object implementing the `eq` method.
    ///
    /// Returns:
    ///
    /// A boolean value is being returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::interval::Interval;
    /// assert_eq!(Interval::new(1, 2), Interval::new(1, 2));
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.lb == other.lb && self.ub == other.ub
    }
}

/// The `impl<T: PartialOrd> Overlap<Interval<T>> for Interval<T>` block is implementing the `Overlap`
/// trait for the `Interval<T>` struct.
impl<T: PartialOrd> Overlap<Interval<T>> for Interval<T> {
    fn overlaps(&self, other: &Interval<T>) -> bool {
        self.ub >= other.lb && other.ub >= self.lb
    }
}

impl<T: PartialOrd> Overlap<T> for Interval<T> {
    fn overlaps(&self, other: &T) -> bool {
        self.ub >= *other && *other >= self.lb
    }
}

/// The `impl<T: PartialOrd> Contain<Interval<T>> for Interval<T>` block is implementing the `Contain`
/// trait for the `Interval<T>` struct.
impl<T: PartialOrd> Contain<Interval<T>> for Interval<T> {
    fn contains(&self, other: &Interval<T>) -> bool {
        self.lb <= other.lb && other.ub <= self.ub
    }
}

/// The `impl<T: PartialOrd> Contain<T> for Interval<T>` block is implementing the `Contain` trait for
/// the `Interval<T>` struct.
impl<T: PartialOrd> Contain<T> for Interval<T> {
    fn contains(&self, other: &T) -> bool {
        self.lb <= *other && *other <= self.ub
    }
}

// impl<T> MinDist<Interval<T>> for Interval<T>
// {
//     fn min_dist_with(&self, other: &Interval<T>) -> u32 {
//         let diff = self.lb - other.ub;
//     }
// }

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
    fn test_interval2() {
        let a = Interval::new(4, 8);
        let b = Interval::new(5, 6);
    
        assert!(!(a < b));
        assert!(!(b < a));
        // assert!(!(a > b));
        // assert!(!(b > a));
        // assert!(a <= b);
        // assert!(b <= a);
        // assert!(a >= b);
        // assert!(b >= a);
    
        assert!(!(b == a));
        assert!(b != a);
    
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
    
    // #[test]
    // fn test_arithmetic() {
    //     let mut a = Interval::new(3, 5);
    //     // b = Interval::new(5, 7);
    //     // c = Interval::new(7, 8);
    //     assert_eq!(a + 1, Interval::new(4, 6));
    //     assert_eq!(a - 1, Interval::new(2, 4));
    //     assert_eq!(a * 2, Interval::new(6, 10));
    //     assert!(-a == Interval::new(-5, -3));
    //     a += 1;
    //     assert!(a == Interval::new(4, 6));
    //     a -= 1;
    //     assert!(a == Interval::new(3, 5));
    //     a *= 2;
    //     assert!(a == Interval::new(6, 10));
    // }
    
    #[test]
    fn test_overlap() {
        let a = Interval::new(3, 5);
        let b = Interval::new(5, 7);
        let c = Interval::new(7, 8);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&c));
        assert!(!a.overlaps(&c));
        assert!(!c.overlaps(&a));
        // assert!(overlap(a, b));
        // assert!(overlap(b, c));
        // assert!(!overlap(a, c));
        // assert!(!overlap(c, a));
    
        let d = 4;
        assert!(a.overlaps(&d));
        assert!(!a.overlaps(&6));
        // assert!(overlap(a, d));
        // assert!(overlap(d, a));
        // assert!(overlap(d, d));
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
        // assert!(!contain(a, b));
        // assert!(!contain(b, c));
        // assert!(!contain(a, c));
    
        let d = 4;
        assert!(a.contains(&d));
        assert!(!a.contains(&6));
        // assert!(contain(a, d));
        // assert!(!contain(d, a));
        // assert!(contain(d, d));
    }
    
    // #[test]
    // fn test_intersection() {
    //     let a = Interval::new(3, 5);
    //     let b = Interval::new(5, 7);
    //     let c = Interval::new(7, 8);
    //     assert_eq!(a.intersection_with(b), Interval::new(5, 5));
    //     assert_eq!(b.intersection_with(c), Interval::new(7, 7));
    //     with pytest.raises(AssertionError) {
    //         a.intersection_with(c);
    //     }
    //     assert!(intersection(a, b) == Interval::new(5, 5));
    //     assert!(intersection(b, c) == Interval::new(7, 7));
    //     let d = 4;
    //     assert!(a.intersection_with(d) == Interval::new(4, 4));
    //     with pytest.raises(AssertionError) {
    //         assert!(a.intersection_with(6));
    //     }
    //     assert_eq!(intersection(a, d), Interval::new(4, 4));
    //     assert_eq!(intersection(d, a), Interval::new(4, 4));
    //     assert_eq!(intersection(d, d), d);
    // }
    
    // #[test]
    // fn test_hull() {
    //     let a = Interval::new(3, 5);
    //     let b = Interval::new(5, 7);
    //     let c = Interval::new(7, 8);
    //     assert_eq!(a.hull_with(&b), Interval::new(3, 7));
    //     assert_eq!(b.hull_with(&c), Interval::new(5, 8));
    //     assert_eq!(a.hull_with(&c), Interval::new(3, 8));
    
    //     let d = 4;
    //     assert_eq!(a.hull_with(d), Interval::new(3, 5));
    //     assert_eq!(a.hull_with(6), Interval::new(3, 6));
    //     // assert_eq!(hull(a, d), Interval::new(3, 5));
    //     // assert_eq!(hull(a, 6), Interval::new(3, 6));
    //     // assert_eq!(hull(d, a), Interval::new(3, 5));
    //     // assert!(hull(6, a) == Interval::new(3, 6));
    //     // assert!(hull(d, 6) == Interval::new(4, 6));
    // }
    
    // #[test]
    // fn test_min_dist() {
    //     let a = Interval::new(3, 5);
    //     let b = Interval::new(5, 7);
    //     let c = Interval::new(7, 8);
    //     assert_eq!(a.min_dist_with(&b), 0);
    //     assert_eq!(a.min_dist_with(&c), 2);
    //     assert_eq!(b.min_dist_with(&c), 0);
    //     // assert_eq!(min_dist(a, b), 0);
    //     // assert_eq!(min_dist(a, c), 2);
    //     // assert_eq!(min_dist(b, c), 0);
    
    //     let d = 4;
    //     assert!(min_dist(a, d) == 0);
    //     // assert!(min_dist(d, a) == 0);
    //     // assert!(min_dist(a, 6) == 1);
    //     // assert!(min_dist(6, a) == 1);
    // }
    
    // #[test]
    // fn test_displacement() {
    //     let a = Interval::new(3, 5);
    //     let b = Interval::new(5, 7);
    //     let c = Interval::new(7, 8);
    //     assert_eq!(a.displace(&b), Interval::new(-2, -2));
    //     assert_eq!(a.displace(&c), Interval::new(-4, -3));
    //     assert_eq!(b.displace(&c), Interval::new(-2, -1));
    //     // assert_eq!(displacement(a, b), Interval::new(-2, -2));
    //     // assert_eq!(displacement(a, c), Interval::new(-4, -3));
    //     // assert_eq!(displacement(b, c), Interval::new(-2, -1));
    //     let d = 4;
    //     // assert_eq!(displacement(d, d), 0);
    //     // assert_eq!(displacement(d, 6), -2);
    //     // assert_eq!(displacement(6, d), 2);
    // }
    
    // #[test]
    // fn test_enlarge() {
    //     a = Interval::new(3, 5);
    //     // b = Interval::new(5, 7)
    //     // c = Interval::new(7, 8)
    //     assert!(a.enlarge_with(2) == Interval::new(1, 7));
    //     assert!(enlarge(a, 2) == Interval::new(1, 7));    
    //     let d = 4;
    //     assert_eq!(enlarge(d, 6), Interval::new(-2, 10));
    //     assert_eq!(enlarge(6, d), Interval::new(2, 10));
    // }    
}
