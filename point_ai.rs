use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Point<T1, T2> {
    pub xcoord: T1,
    pub ycoord: T2,
}

impl<T1: std::fmt::Display, T2: std::fmt::Display> std::fmt::Display for Point<T1, T2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.xcoord, self.ycoord)
    }
}

impl<T1, T2> Point<T1, T2> {
    pub fn new(xcoord: T1, ycoord: T2) -> Self {
        Point { xcoord, ycoord }
    }
}

impl<T1: std::cmp::PartialOrd, T2: std::cmp::PartialOrd> Point<T1, T2> {
    pub fn overlaps(&self, other: &Self) -> bool {
        overlap(&self.xcoord, &other.xcoord) && overlap(&self.ycoord, &other.ycoord)
    }

    pub fn contains(&self, other: &Self) -> bool {
        contain(&self.xcoord, &other.xcoord) && contain(&self.ycoord, &other.ycoord)
    }

    pub fn hull_with(&self, other: &Self) -> Self {
        Point {
            xcoord: hull(&self.xcoord, &other.xcoord),
            ycoord: hull(&self.ycoord, &other.ycoord),
        }
    }

    pub fn intersection_with(&self, other: &Self) -> Self {
        Point {
            xcoord: intersection(&self.xcoord, &other.xcoord),
            ycoord: intersection(&self.ycoord, &other.ycoord),
        }
    }

    pub fn min_dist_with(&self, other: &Self) -> f64 {
        min_dist(&self.xcoord, &other.xcoord) + min_dist(&self.ycoord, &other.ycoord)
    }

    pub fn enlarge_with(&self, alpha: f64) -> Self {
        Point {
            xcoord: enlarge(&self.xcoord, alpha),
            ycoord: enlarge(&self.ycoord, alpha),
        }
    }

    pub fn displace(&self, rhs: &Self) -> Vector2<T1, T2>
    where
        T1: std::ops::Sub<Output = T1>,
        T2: std::ops::Sub<Output = T2>,
    {
        Vector2 {
            x: displacement(&self.xcoord, &rhs.xcoord),
            y: displacement(&self.ycoord, &rhs.ycoord),
        }
    }

    pub fn flip(&self) -> Point<T2, T1> {
        Point {
            xcoord: self.ycoord,
            ycoord: self.xcoord,
        }
    }
}

impl<T1: std::ops::Add<Output = T1>, T2: std::ops::Add<Output = T2>> Add<Vector2<T1, T2>> for Point<T1, T2> {
    type Output = Self;

    fn add(self, rhs: Vector2<T1, T2>) -> Self::Output {
        Point {
            xcoord: self.xcoord + rhs.x,
            ycoord: self.ycoord + rhs.y,
        }
    }
}

impl<T1: std::ops::AddAssign, T2: std::ops::AddAssign> AddAssign<Vector2<T1, T2>> for Point<T1, T2> {
    fn add_assign(&mut self, rhs: Vector2<T1, T2>) {
        self.xcoord += rhs.x;
        self.ycoord += rhs.y;
    }
}

impl<T1: std::ops::Sub<Output = T1>, T2: std::ops::Sub<Output = T2>> Sub<Vector2<T1, T2>> for Point<T1, T2> {
    type Output = Self;

    fn sub(self, rhs: Vector2<T1, T2>) -> Self::Output {
        Point {
            xcoord: self.xcoord - rhs.x,
            ycoord: self.ycoord - rhs.y,
        }
    }
}

impl<T1: std::ops::SubAssign, T2: std::ops::SubAssign> SubAssign<Vector2<T1, T2>> for Point<T1, T2> {
    fn sub_assign(&mut self, rhs: Vector2<T1, T2>) {
        self.xcoord -= rhs.x;
        self.ycoord -= rhs.y;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Vector2<T1, T2> {
    pub x: T1,
    pub y: T2,
}

impl<T1: std::fmt::Display, T2: std::fmt::Display> std::fmt::Display for Vector2<T1, T2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T1, T2> Vector2<T1, T2> {
    pub fn new(x: T1, y: T2) -> Self {
        Vector2 { x, y }
    }
}

fn overlap<T: std::cmp::PartialOrd>(a: &T, b: &T) -> bool {
    a <= b
}

fn contain<T: std::cmp::PartialOrd>(a: &T, b: &T) -> bool {
    a >= b
}

fn hull<T: std::cmp::PartialOrd>(a: &T, b: &T) -> T
where
    T: Clone,
{
    if a <= b {
        a.clone()
    } else {
        b.clone()
    }
}

fn intersection<T: std::cmp::PartialOrd>(a: &T, b: &T) -> T
where
    T: Clone,
{
    if a <= b {
        a.clone()
    } else {
        b.clone()
    }
}

fn min_dist<T: std::cmp::PartialOrd + std::ops::Sub<Output = T> + std::ops::Abs<Output = T>>(a: &T, b: &T) -> f64
where
    T: std::convert::Into<f64>,
{
    (a - b).abs().into()
}

fn enlarge<T: std::cmp::PartialOrd + std::ops::Sub<Output = T> + std::ops::Mul<f64, Output = T>>(a: &T, alpha: f64) -> T
where
    T: Clone,
{
    a.clone() - (a.clone() * alpha)
}

fn displacement<T: std::ops::Sub<Output = T>>(a: &T, b: &T) -> T
where
    T: Clone,
{
    b.clone() - a.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let a = Point::new(4, 8);
        let b = Point::new(5, 6);
        assert!(a < b);
        assert!(a <= b);
        assert!(b != a);
    }

    #[test]
    fn test_point2() {
        let a = Point::new(3, 4);
        let r = Point::new(Interval::new(3, 4), Interval::new(5, 6));
        assert_eq!(
            r.intersection_with(&Point::new(4, 5)),
            Point::new(Interval::new(4, 4), Interval::new(5, 5))
        );
    }

    #[test]
    fn test_transform() {
        let a = Point::new(3, 5);
        let b = Vector2::new(5, 7);
        assert_eq!(a + b, Point::new(8, 12));
        assert_eq!(a - b, Point::new(-2, -2));
        assert_eq!(a.flip(), Point::new(5, 3));

        let mut a = a;
        a += b;
        assert_eq!(a, Point::new(8, 12));
        a -= b;
        assert_eq!(a, Point::new(3, 5));
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
        assert_eq!(a.enlarge_with(2), Point::new(Interval::new(1, 5), Interval::new(3, 7)));
    }

    #[test]
    fn test_hull() {
        let a = Point::new(3, 5);
        let b = Point::new(5, 7);
        assert_eq!(a.hull_with(&b), Point::new(Interval::new(3, 5), Interval::new(5, 7)));
    }
}


