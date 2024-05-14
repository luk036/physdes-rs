use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval<T>
where
    T: PartialOrd + Copy,
{
    lb: T,
    ub: T,
}

impl<T> Interval<T>
where
    T: PartialOrd + Copy,
{
    pub fn new(lb: T, ub: T) -> Self {
        Interval { lb, ub }
    }

    pub fn lb(&self) -> T {
        self.lb
    }

    pub fn ub(&self) -> T {
        self.ub
    }

    pub fn length(&self) -> T
    where
        T: Sub<Output = T>,
    {
        self.ub - self.lb
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !(self < other || other < self)
    }

    pub fn contains(&self, obj: T) -> bool {
        self.lb <= obj && obj <= self.ub
    }

    pub fn hull_with(&self, obj: T) -> Self {
        Interval {
            lb: self.lb.min(obj),
            ub: self.ub.max(obj),
        }
    }

    pub fn intersect_with(&self, obj: T) -> Self {
        assert!(self.overlaps(&Interval::new(obj, obj)));
        Interval {
            lb: self.lb.max(obj),
            ub: self.ub.min(obj),
        }
    }

    pub fn min_dist_with(&self, obj: T) -> T
    where
        T: PartialOrd,
    {
        if self < &Interval::new(obj, obj) {
            obj - self.ub
        } else if Interval::new(obj, obj) < *self {
            self.lb - obj
        } else {
            T::default()
        }
    }

    pub fn displace(&self, obj: &Self) -> Self {
        Interval {
            lb: self.lb - obj.lb,
            ub: self.ub - obj.ub,
        }
    }

    pub fn enlarge_with(&self, alpha: T) -> Self {
        Interval {
            lb: self.lb - alpha,
            ub: self.ub + alpha,
        }
    }
}

impl<T> Display for Interval<T>
where
    T: PartialOrd + Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}, {}]", self.lb, self.ub)
    }
}

impl<T> PartialOrd for Interval<T>
where
    T: PartialOrd + Copy,
{
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

impl<T> Ord for Interval<T>
where
    T: PartialOrd + Copy + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> Neg for Interval<T>
where
    T: PartialOrd + Copy + Neg<Output = T>,
{
    type Output = Interval<T>;

    fn neg(self) -> Self::Output {
        Interval {
            lb: -self.ub,
            ub: -self.lb,
        }
    }
}

impl<T> AddAssign<T> for Interval<T>
where
    T: PartialOrd + Copy + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: T) {
        self.lb += rhs;
        self.ub += rhs;
    }
}

impl<T> Add<T> for Interval<T>
where
    T: PartialOrd + Copy + Add<Output = T>,
{
    type Output = Interval<T>;

    fn add(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb + rhs,
            ub: self.ub + rhs,
        }
    }
}

impl<T> SubAssign<T> for Interval<T>
where
    T: PartialOrd + Copy + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.lb -= rhs;
        self.ub -= rhs;
    }
}

impl<T> Sub<T> for Interval<T>
where
    T: PartialOrd + Copy + Sub<Output = T>,
{
    type Output = Interval<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb - rhs,
            ub: self.ub - rhs,
        }
    }
}

impl<T> MulAssign<T> for Interval<T>
where
    T: PartialOrd + Copy + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.lb *= rhs;
        self.ub *= rhs;
    }
}

impl<T> Mul<T> for Interval<T>
where
    T: PartialOrd + Copy + Mul<Output = T>,
{
    type Output = Interval<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Interval {
            lb: self.lb * rhs,
            ub: self.ub * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval() {
        let a = Interval::new(4, 8);
        let b = Interval::new(5, 6);

        assert!(!(a < b));
        assert!(!(b < a));
        assert!(!(a > b));
        assert!(!(b > a));
        assert!(a <= b);
        assert!(b <= a);
        assert!(a >= b);
        assert!(b >= a);

        assert!(!(b == a));
        assert!(b != a);

        assert!(a.contains(4));
        assert!(a.contains(8));
        assert_eq!(a.intersect_with(8), Interval::new(8, 8));
        assert!(a.contains(&b));
        assert_eq!(a.intersect_with(&b), b);
        assert!(!b.contains(&a));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
        assert_eq!(a.min_dist_with(&b), 0);
        assert_eq!(a.length(), 4);
    }
}


