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

#[derive(Debug)]
pub struct Interval<T: PartialOrd> {
    lb: T,
    ub: T,
    _marker: PhantomData<T>,
}

impl<T: PartialOrd> Interval<T> {
    pub fn new(lb: T, ub: T) -> Self {
        Self {
            lb,
            ub,
            _marker: PhantomData,
        }
    }
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.lb.partial_cmp(&other.ub)?)
    }
}

impl<T: PartialOrd> PartialEq for Interval<T> {
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
        // assert!(a.overlaps(&b));
        // assert!(b.overlaps(&a));
    }
}
