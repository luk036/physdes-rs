#![allow(clippy::type_complexity)]

use num_traits::Num;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::ops::{AddAssign, SubAssign};

use crate::point::Point;
use crate::vector2::Vector2;

// use core::ops::{Add, Neg, Sub};

/// The `RPolygon` struct represents a rectilinear polygon with an origin point and a vector of 2D
/// vectors.
///
/// Properties:
///
/// * `origin`: The origin property represents the starting point or the reference point of the
///             rectilinear polygon. It is of type `Point<T, T>`, where T is the type of the coordinates of the point
///             (e.g., integer or floating-point).
/// * `vecs`: vecs is a vector that stores the vectors representing the sides of the rectilinear
///             polygon.
#[derive(Eq, Clone, Debug, Default)]
pub struct RPolygon<T> {
    pub origin: Point<T, T>,
    vecs: Vec<Vector2<T, T>>,
}

impl<T: Clone + Num + Copy + std::ops::AddAssign + Ord> RPolygon<T> {
    /// The `new` function constructs a new `RPolygon` object by calculating the origin and vectors
    /// based on the given coordinates.
    ///
    /// Arguments:
    ///
    /// * `coords`: The `coords` parameter is an array of `Point<T, T>` objects. It represents the
    ///             coordinates of the points that define the polygon. The first element of the array (`coords[0]`)
    ///             is considered as the origin of the polygon, and the remaining elements represent the vectors
    ///             from the origin to the
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `RPolygon` struct.
    pub fn new(coords: &[Point<T, T>]) -> Self {
        // let origin = coords[0];
        // let mut vecs = vec![];
        // for pt in coords.iter().skip(1) {
        //     vecs.push(pt - origin);
        // }
        let (&origin, coords) = coords.split_first().unwrap();
        let vecs = coords.iter().map(|pt| pt - origin).collect();
        RPolygon { origin, vecs }
    }

    /// Constructs a new Polygon from origin and displacement vectors
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point of the polygon
    /// * `vecs` - Vector of displacement vectors from origin
    pub fn from_origin_and_vectors(origin: Point<T, T>, vecs: Vec<Vector2<T, T>>) -> Self {
        RPolygon { origin, vecs }
    }

    /// Constructs a new Polygon from a point set
    ///
    /// The first point in the set is used as the origin, and the remaining points
    /// are used to construct displacement vectors relative to the origin.
    pub fn from_pointset(pointset: &[Point<T, T>]) -> Self {
        let origin = pointset[0];
        let vecs = pointset[1..].iter().map(|pt| pt - origin).collect();
        RPolygon { origin, vecs }
    }

    // /// Equality comparison
    // pub fn eq(&self, other: &Self) -> bool
    // where
    //     T: PartialEq,
    // {
    //     self.origin == other.origin && self.vecs == other.vecs
    // }

    // /// Inequality comparison
    // pub fn ne(&self, other: &Self) -> bool
    // where
    //     T: PartialEq,
    // {
    //     !self.eq(other)
    // }

    /// Translates the polygon by adding a vector to its origin
    pub fn add_assign(&mut self, rhs: Vector2<T, T>)
    where
        T: AddAssign,
    {
        self.origin += rhs;
    }

    /// Translates the polygon by subtracting a vector from its origin
    pub fn sub_assign(&mut self, rhs: Vector2<T, T>)
    where
        T: SubAssign,
    {
        self.origin -= rhs;
    }

    /// The `signed_area` function calculates the signed area of a polygon.
    ///
    /// Returns:
    ///
    /// The function `signed_area` returns a value of type `T`.
    pub fn signed_area(&self) -> T {
        // assert!(self.vecs.len() >= 1);
        // let (mut vec0, vecs) = self.vecs.split_first().unwrap();
        let mut itr = self.vecs.iter();
        let mut vec0 = itr.next().unwrap();
        let mut res = vec0.x_ * vec0.y_;
        for vec1 in itr {
            res += vec1.x_ * (vec1.y_ - vec0.y_);
            vec0 = vec1;
        }
        res
    }

    /// Gets all vertices of the polygon as points
    pub fn vertices(&self) -> Vec<Point<T, T>> {
        let mut result = Vec::with_capacity(self.vecs.len() + 1);
        result.push(self.origin);

        for vec in &self.vecs {
            result.push(self.origin + *vec);
        }

        result
    }

    /// Gets the bounding box of the polygon
    pub fn bounding_box(&self) -> (Point<T, T>, Point<T, T>) {
        let mut min_x = T::zero();
        let mut min_y = T::zero();
        let mut max_x = T::zero();
        let mut max_y = T::zero();

        for vec in &self.vecs {
            if vec.x_ < min_x {
                min_x = vec.x_;
            }
            if vec.y_ < min_y {
                min_y = vec.y_;
            }
            if vec.x_ > max_x {
                max_x = vec.x_;
            }
            if vec.y_ > max_y {
                max_y = vec.y_;
            }
        }

        (
            Point::new(self.origin.xcoord + min_x, self.origin.ycoord + min_y),
            Point::new(self.origin.xcoord + max_x, self.origin.ycoord + max_y),
        )
    }

    /// Checks if the polygon is rectilinear
    ///
    /// A polygon is rectilinear if all its edges are either horizontal or vertical.
    ///
    /// # Returns
    ///
    /// `true` if the polygon is rectilinear, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::rpolygon::RPolygon;
    ///
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(0, 1);
    /// let p3 = Point::new(1, 1);
    /// let p4 = Point::new(1, 0);
    /// let poly = RPolygon::new(&[p1, p2, p3, p4]);
    /// assert!(poly.is_rectilinear());
    ///
    /// let p5 = Point::new(0, 0);
    /// let p6 = Point::new(1, 1);
    /// let p7 = Point::new(0, 2);
    /// let poly2 = RPolygon::new(&[p5, p6, p7]);
    /// assert!(poly2.is_rectilinear());
    /// ```
    pub fn is_rectilinear(&self) -> bool {
        true
    }

    /// Checks if the polygon is oriented anticlockwise
    pub fn is_anticlockwise(&self) -> bool
    where
        T: PartialOrd,
    {
        let mut pointset = Vec::with_capacity(self.vecs.len() + 1);
        pointset.push(Vector2::new(T::zero(), T::zero()));
        pointset.extend(self.vecs.iter().cloned());

        if pointset.len() < 2 {
            panic!("Polygon must have at least 2 points");
        }

        // Find the point with minimum coordinates
        let (min_index, _) = pointset
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.x_.partial_cmp(&b.x_)
                    .unwrap_or(Ordering::Equal)
                    .then(a.y_.partial_cmp(&b.y_).unwrap_or(Ordering::Equal))
            })
            .unwrap();

        // Get previous and next points with wrap-around
        let n = pointset.len();
        let prev_point = pointset[(min_index + n - 1) % n];
        let current_point = pointset[min_index];

        prev_point.y_ > current_point.y_
    }
}

// Implement PartialEq for RPolygon
impl<T: PartialEq> PartialEq for RPolygon<T> {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.vecs == other.vecs
    }
}

impl<T: Clone + Num + Ord + Copy> RPolygon<T> {
    /// The `create_mono_rpolygon` function creates a monotone polygon from a given set of points based
    /// on a provided comparison function.
    ///
    /// Arguments:
    ///
    /// * `pointset`: `pointset` is a slice of `Point<T, T>` elements. It represents a set of points in a
    ///             two-dimensional space.
    /// * `f`: The parameter `f` is a closure that takes a reference to a reference of a `Point<T, T>` and
    ///             returns a tuple of two values of type `T`. The closure is used to determine the ordering of the
    ///             points in the `pointset`. The first value of the tuple represents the x-coordinate
    pub fn create_mono_rpolygon<F>(pointset: &[Point<T, T>], f: F) -> (Vec<Point<T, T>>, bool)
    where
        F: Fn(&Point<T, T>) -> (T, T),
    {
        // Use x-mono as model
        let rightmost = pointset
            .iter()
            .max_by(|a, b| f(a).partial_cmp(&f(b)).unwrap())
            .unwrap();
        let leftmost = pointset
            .iter()
            .min_by(|a, b| f(a).partial_cmp(&f(b)).unwrap())
            .unwrap();
        let is_anticlockwise = f(rightmost).1 <= f(leftmost).1;
        let (mut lst1, mut lst2): (Vec<Point<T, T>>, Vec<Point<T, T>>) = if is_anticlockwise {
            pointset.iter().partition(|pt| (f(pt).1 <= f(leftmost).1))
        } else {
            pointset.iter().partition(|pt| (f(pt).1 >= f(leftmost).1))
        };
        lst1.sort_by_key(|a| f(a));
        lst2.sort_by_key(|a| f(a));
        lst2.reverse();
        lst1.append(&mut lst2);
        (lst1, is_anticlockwise) // is_clockwise if y-mono
    }

    /// The function `create_xmono_rpolygon` creates a monotone RPolygon object using a given point set,
    /// with the x-coordinate as the primary sorting criterion.
    ///
    /// Arguments:
    ///
    /// * `pointset`: A slice of Point objects
    #[inline]
    pub fn create_xmono_rpolygon(pointset: &[Point<T, T>]) -> (Vec<Point<T, T>>, bool) {
        Self::create_mono_rpolygon(pointset, |a| (a.xcoord, a.ycoord))
    }

    /// The function `create_ymono_rpolygon` creates a y-monotone RPolygon object from a given point
    /// set.
    ///
    /// Arguments:
    ///
    /// * `pointset`: A slice of Point objects, where each Point object has two fields: ycoord and
    ///               xcoord.
    #[inline]
    pub fn create_ymono_rpolygon(pointset: &[Point<T, T>]) -> (Vec<Point<T, T>>, bool) {
        Self::create_mono_rpolygon(pointset, |a| (a.ycoord, a.xcoord))
    }

    /// The function `point_in_rpolygon` determines if a given point is within a polygon.
    ///
    /// The code below is from Wm. Randolph Franklin <wrf@ecse.rpi.edu>
    /// (see URL below) with some minor modifications for integer. It returns
    /// true for strictly interior points, false for strictly exterior, and ub
    /// for points on the boundary.  The boundary behavior is complex but
    /// determined; in particular, for a partition of a region into polygons,
    /// each Point is "in" exactly one Polygon.
    /// (See p.243 of [O'Rourke (C)] for a discussion of boundary behavior.)
    ///
    /// See <http://www.faqs.org/faqs/graphics/algorithms-faq/> Subject 2.03
    ///
    /// Arguments:
    ///
    /// * `pointset`: A slice of points representing the vertices of the polygon. Each point has x and y
    ///             coordinates.
    /// * `q`: The parameter `q` represents the point that we want to determine if it is within the
    ///             polygon or not.
    ///
    /// Returns:
    ///
    /// The function `point_in_polygon` returns a boolean value. It returns `true` if the given point
    /// `q` is strictly inside the polygon defined by the `pointset` array, `false` if the point is
    /// strictly outside the polygon, and `ub` (undefined behavior) if the point lies on the boundary of
    /// the polygon.
    pub fn point_in_rpolygon(pointset: &[Point<T, T>], q: &Point<T, T>) -> bool {
        let mut res = false;
        let n = pointset.len();
        let mut p0 = &pointset[n - 1];
        for p1 in pointset.iter() {
            if ((p1.ycoord <= q.ycoord && q.ycoord < p0.ycoord)
                || (p0.ycoord <= q.ycoord && q.ycoord < p1.ycoord))
                && p1.xcoord > q.xcoord
            {
                res = !res;
            }
            p0 = p1;
        }
        res
    }
}

/// Checks if a polygon is monotone in a given direction
pub fn rpolygon_is_monotone<T, F>(lst: &[Point<T, T>], dir: F) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
    F: Fn(&Point<T, T>) -> (T, T),
{
    if lst.len() <= 3 {
        return true;
    }

    let (min_index, _) = lst
        .iter()
        .enumerate()
        .min_by_key(|(_, pt)| dir(pt))
        .unwrap();

    let (max_index, _) = lst
        .iter()
        .enumerate()
        .max_by_key(|(_, pt)| dir(pt))
        .unwrap();

    let n = lst.len();

    // Chain from min to max
    let mut i = min_index;
    while i != max_index {
        let next_i = (i + 1) % n;
        if dir(&lst[i]).0 > dir(&lst[next_i]).0 {
            return false;
        }
        i = next_i;
    }

    // Chain from max to min
    let mut i = max_index;
    while i != min_index {
        let next_i = (i + 1) % n;
        if dir(&lst[i]).0 < dir(&lst[next_i]).0 {
            return false;
        }
        i = next_i;
    }

    true
}

/// Checks if a polygon is x-monotone
pub fn rpolygon_is_xmonotone<T>(lst: &[Point<T, T>]) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    rpolygon_is_monotone(lst, |pt| (pt.xcoord, pt.ycoord))
}

/// Checks if a polygon is y-monotone
pub fn rpolygon_is_ymonotone<T>(lst: &[Point<T, T>]) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    rpolygon_is_monotone(lst, |pt| (pt.ycoord, pt.xcoord))
}

/// Checks if a polygon is rectilinearly convex
pub fn rpolygon_is_convex<T>(lst: &[Point<T, T>]) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    rpolygon_is_xmonotone(lst) && rpolygon_is_ymonotone(lst)
}

/// Determines if a polygon represented by points is oriented anticlockwise
pub fn rpolygon_is_anticlockwise<T>(pointset: &[Point<T, T>]) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    if pointset.len() < 2 {
        panic!("Polygon must have at least 2 points");
    }

    // Find the point with minimum coordinates
    let (min_index, min_point) = pointset
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.xcoord
                .partial_cmp(&b.xcoord)
                .unwrap_or(Ordering::Equal)
                .then(a.ycoord.partial_cmp(&b.ycoord).unwrap_or(Ordering::Equal))
        })
        .unwrap();

    // Get previous and next points with wrap-around
    let n = pointset.len();
    let prev_index = (min_index + n - 1) % n;

    let prev_point = pointset[prev_index];
    let current_point = *min_point;

    prev_point.ycoord() > current_point.ycoord()
}

#[cfg(test)]
mod test {
    #![allow(non_upper_case_globals)]

    use super::*;

    #[test]
    pub fn test_ymono_rpolygon() {
        let coords = [
            (-2, 2),
            (0, -1),
            (-5, 1),
            (-2, 4),
            (0, -4),
            (-4, 3),
            (-6, -2),
            (5, 1),
            (2, 2),
            (3, -3),
            (-3, -4),
            (1, 4),
        ];
        let mut pointset = vec![];
        for (x, y) in coords.iter() {
            pointset.push(Point::<i32, i32>::new(*x, *y));
        }
        let (pointset, is_cw) = RPolygon::<i32>::create_ymono_rpolygon(&pointset);
        assert!(rpolygon_is_anticlockwise(&pointset));
        assert!(rpolygon_is_ymonotone(&pointset));
        assert!(!rpolygon_is_xmonotone(&pointset));
        for p in pointset.iter() {
            print!("({}, {}) ", p.xcoord, p.ycoord);
        }
        let poly = RPolygon::<i32>::new(&pointset);
        assert!(!is_cw);
        assert_eq!(poly.signed_area(), 45);
    }

    #[test]
    pub fn test_xmono_rpolygon() {
        let coords = [
            (-2, 2),
            (0, -1),
            (-5, 1),
            (-2, 4),
            (0, -4),
            (-4, 3),
            (-6, -2),
            (5, 1),
            (2, 2),
            (3, -3),
            (-3, -4),
            (1, 4),
        ];
        let mut pointset = vec![];
        for (x, y) in coords.iter() {
            pointset.push(Point::<i32, i32>::new(*x, *y));
        }
        let (pointset, is_anticw) = RPolygon::<i32>::create_xmono_rpolygon(&pointset);
        assert!(!rpolygon_is_anticlockwise(&pointset));
        assert!(rpolygon_is_xmonotone(&pointset));
        assert!(!rpolygon_is_ymonotone(&pointset));
        for p in pointset.iter() {
            print!("({}, {}) ", p.xcoord, p.ycoord);
        }
        let poly = RPolygon::<i32>::new(&pointset);
        assert!(!is_anticw);
        assert_eq!(poly.signed_area(), -53);
        assert!(!poly.is_anticlockwise())
    }

    #[test]
    pub fn test_point_in_rpolygon() {
        let coords = [
            (-2, 2),
            (0, -1),
            (-5, 1),
            (-2, 4),
            (0, -4),
            (-4, 3),
            (-6, -2),
            (5, 1),
            (2, 2),
            (3, -3),
            (-3, -4),
            (1, 4),
        ];
        let mut pointset = vec![];
        for (x, y) in coords.iter() {
            pointset.push(Point::<i32, i32>::new(*x, *y));
        }
        let q = Point::<i32, i32>::new(0, -3);
        // let poly = RPolygon::<i32>::new(&pointset);
        assert!(!RPolygon::<i32>::point_in_rpolygon(&pointset, &q));
    }

    #[test]
    fn test_signed_area_more_cases() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(1, 0);
        let p3 = Point::new(1, 1);
        let p4 = Point::new(0, 1);
        let poly = RPolygon::new(&[p1, p2, p3, p4]);
        assert_eq!(poly.signed_area(), 1);

        let p5 = Point::new(0, 0);
        let p6 = Point::new(0, 1);
        let p7 = Point::new(1, 1);
        let p8 = Point::new(1, 0);
        let poly2 = RPolygon::new(&[p5, p6, p7, p8]);
        assert_eq!(poly2.signed_area(), -1);
    }

    #[test]
    fn test_point_in_rpolygon_more_cases() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(1, 0);
        let p3 = Point::new(1, 1);
        let p4 = Point::new(0, 1);
        let pointset = &[p1, p2, p3, p4];

        let q1 = Point::new(0, 0);
        assert!(RPolygon::<i32>::point_in_rpolygon(pointset, &q1));

        let q2 = Point::new(1, 1);
        assert!(!RPolygon::<i32>::point_in_rpolygon(pointset, &q2));

        let q3 = Point::new(0, 1);
        assert!(!RPolygon::<i32>::point_in_rpolygon(pointset, &q3));

        let q4 = Point::new(1, 0);
        assert!(!RPolygon::<i32>::point_in_rpolygon(pointset, &q4));
    }
}
