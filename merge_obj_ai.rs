use std::ops::{Add, Sub};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
struct Point<T1, T2> {
    xcoord: T1,
    ycoord: T2,
}

impl<T1, T2> Point<T1, T2> {
    fn new(xcoord: T1, ycoord: T2) -> Self {
        Point { xcoord, ycoord }
    }

    fn intersection_with(&self, other: &Point<T1, T2>) -> Point<T1, T2> {
        // TODO: implement intersection logic
        Point::new(self.xcoord, self.ycoord)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Interval<T>(T, T);

fn enlarge<T>(value: T, alpha: i32) -> Interval<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    // TODO: implement enlarge logic
    Interval(value, value)
}

fn min_dist<T>(a: T, b: T) -> i32
where
    T: std::cmp::PartialOrd,
{
    // TODO: implement min_dist logic
    0
}

fn intersection<T1, T2>(a: Point<T1, T2>, b: Point<T1, T2>) -> Point<T1, T2>
where
    T1: Copy,
    T2: Copy,
{
    // TODO: implement intersection logic
    Point::new(a.xcoord, a.ycoord)
}

#[derive(Debug, PartialEq, Eq)]
struct MergeObj<T1, T2> {
    impl_: Point<T1, T2>,
}

impl<T1, T2> MergeObj<T1, T2> {
    fn new(xcoord: T1, ycoord: T2) -> Self {
        MergeObj {
            impl_: Point::new(xcoord, ycoord),
        }
    }

    fn construct(xcoord: i32, ycoord: i32) -> MergeObj<i32, i32> {
        let impl_ = Point::new(xcoord + ycoord, xcoord - ycoord);
        MergeObj { impl_ }
    }

    fn min_dist_with(&self, other: &MergeObj<T1, T2>) -> i32 {
        // Note: take max of xcoord and ycoord
        max(
            min_dist(self.impl_.xcoord, other.impl_.xcoord),
            min_dist(self.impl_.ycoord, other.impl_.ycoord),
        )
    }

    fn enlarge_with(&self, alpha: i32) -> MergeObj<Interval<T1>, Interval<T2>> {
        let xcoord = enlarge(self.impl_.xcoord, alpha);
        let ycoord = enlarge(self.impl_.ycoord, alpha);
        MergeObj::new(xcoord, ycoord)
    }

    fn intersection_with(&self, other: &MergeObj<T1, T2>) -> MergeObj<T1, T2> {
        let point = self.impl_.intersection_with(&other.impl_);
        MergeObj::new(point.xcoord, point.ycoord)
    }

    fn merge_with(&self, other: &MergeObj<T1, T2>) -> MergeObj<Interval<T1>, Interval<T2>> {
        let alpha = self.min_dist_with(other);
        let half = alpha / 2;
        let trr1 = self.enlarge_with(half);
        let trr2 = other.enlarge_with(alpha - half);
        let impl_ = intersection(trr1.impl_, trr2.impl_);
        MergeObj::new(impl_.xcoord, impl_.ycoord)
    }
}

impl<T1, T2> fmt::Display for MergeObj<T1, T2>
where
    T1: fmt::Display,
    T2: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}, {}/", self.impl_.xcoord, self.impl_.ycoord)
    }
}

impl<T1, T2> Add<Vector2<T1, T2>> for MergeObj<T1, T2>
where
    T1: std::ops::Add<Output = T1>,
    T2: std::ops::Add<Output = T2> + std::ops::Sub<Output = T2>,
{
    type Output = MergeObj<T1, T2>;

    fn add(mut self, rhs: Vector2<T1, T2>) -> Self::Output {
        self.impl_.xcoord += rhs.x + rhs.y;
        self.impl_.ycoord += rhs.x - rhs.y;
        self
    }
}

impl<T1, T2> Sub<Vector2<T1, T2>> for MergeObj<T1, T2>
where
    T1: std::ops::Sub<Output = T1>,
    T2: std::ops::Sub<Output = T2>,
{
    type Output = MergeObj<T1, T2>;

    fn sub(mut self, rhs: Vector2<T1, T2>) -> Self::Output {
        self.impl_.xcoord -= rhs.x + rhs.y;
        self.impl_.ycoord -= rhs.x - rhs.y;
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Vector2<T1, T2> {
    x: T1,
    y: T2,
}

fn max<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

fn test_MergeObj() {
    let r1 = MergeObj::construct(4, 5);
    let r2 = MergeObj::construct(7, 9);
    // let v = Vector2 { x: 5, y: 6 };

    assert_ne!(r1, r2);
    // assert_eq!((r1 - v) + v, r1);
    // assert!(!overlap(r1, r2));
    assert_eq!(r1.min_dist_with(&r2), 7);
    assert_eq!(min_dist(&r1, &r2), 7);
}

fn test_merge() {
    let s1 = MergeObj::new(200 + 600, 200 - 600);
    let s2 = MergeObj::new(500 + 900, 500 - 900);
    let m1 = s1.merge_with(&s2);
    println!("{}", m1);
    assert_eq!(m1, MergeObj::new(Interval(1100, 1100), Interval(-700, -100)));
}

fn test_merge_2() {
    let a = MergeObj::new(4 + 5, 4 - 5);
    let b = MergeObj::new(7 + 9, 7 - 9);
    let v = Vector2 { x: 2, y: 3 };
    let mut a = a + v;
    a = a - v;
    assert_eq!(a, MergeObj::new(4 + 5, 4 - 5));
    let r1 = a.enlarge_with(3);
    assert_eq!(r1, MergeObj::new(Interval(6, 12), Interval(-4, 2)));
    let r2 = b.enlarge_with(4);
    assert_eq!(r2, MergeObj::new(Interval(12, 20), Interval(-6, 2)));
    let r3 = r1.intersection_with(&r2);
    assert_eq!(r3, MergeObj::new(Interval(12, 12), Interval(-4, 2)));
}


