use std::cmp::PartialOrd;
use std::marker::PhantomData;

trait Overlaps<T> {
    fn overlaps(&self, other: &T) -> bool;
}

trait Contains<T> {
    fn contains(&self, other: &T) -> bool;
}

impl<T: PartialOrd> Overlaps<Interval<T>> for Interval<T> {
    fn overlaps(&self, other: &Interval<T>) -> bool {
        !(self < other || other < self)
    }
}

impl<T: PartialOrd> Contains<Interval<T>> for Interval<T> {
    fn contains(&self, other: &Interval<T>) -> bool {
        self.lb <= other.lb && other.ub <= self.ub
    }
}

impl<T: PartialOrd> Contains<T> for Interval<T> {
    fn contains(&self, other: &T) -> bool {
        self.lb <= *other && *other <= self.ub
    }
}

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
    /// use physdes::interval_ai::Interval;
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
    /// use physdes::interval_ai::Interval;
    /// use std::marker::PhantomData;
    /// assert_eq!(Interval::new(1, 2).partial_cmp(&Interval::new(2, 3)), Some(std::cmp::Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.lb.partial_cmp(&other.ub)
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
    /// use physdes::interval_ai::Interval;
    /// assert_eq!(Interval::new(1, 2), Interval::new(1, 2));
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.lb == other.lb && self.ub == other.ub
    }
}

pub fn overlap<T: PartialOrd>(lhs: &Interval<T>, rhs: &Interval<T>) -> bool {
    lhs.overlaps(rhs) || rhs.overlaps(lhs) || lhs == rhs
}

pub fn contain<T: PartialOrd>(lhs: &Interval<T>, rhs: &Interval<T>) -> bool {
    lhs.contains(rhs) && !rhs.contains(lhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval() {
        let a = Interval::new(4, 8);
        let b = Interval::new(5, 6);
        assert!(!overlap(&a, &b));
        assert!(!overlap(&b, &a));
        // assert!(!contain(&a, &b));
        assert!(a.contains(&4));
        assert!(a.contains(&8));
        assert!(a.contains(&b));
        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_ne!(a, b);
        assert_ne!(b, a);
        assert!(overlap(&a, &a));
        assert!(overlap(&b, &b));
        assert!(!contain(&a, &a));
        assert!(!contain(&b, &b));
        // assert!(a.overlaps(&b));
        // assert!(b.overlaps(&a));
    }
}
