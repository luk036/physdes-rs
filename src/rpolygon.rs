#![allow(clippy::type_complexity)]

use num_traits::Num;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::ops::{AddAssign, SubAssign};

use crate::point::Point;
use crate::vector2::Vector2;

/// The `RPolygon` struct represents a rectilinear polygon with an origin point and a vector of 2D
/// vectors.
///
/// ```svgbob
///    *-----*-----*
///    |           |
///    |           *-----*
///    |                 |
///    *-----------------*
///    |
///    *-> origin
/// ```
///
/// Properties:
///
/// * `origin`: The origin property represents the starting point or the reference point of the
///   rectilinear polygon. It is of type `Point<T, T>`, where T is the type of the coordinates of the point
///   (e.g., integer or floating-point).
/// * `vecs`: vecs is a vector that stores the vectors representing the sides of the rectilinear
///   polygon.
///
/// # Examples
///
/// ```
/// use physdes::point::Point;
/// use physdes::rpolygon::RPolygon;
/// use physdes::vector2::Vector2;
///
/// let origin = Point::new(0, 0);
/// let vecs = vec![Vector2::new(1, 0), Vector2::new(1, 1), Vector2::new(0, 1)];
/// let poly = RPolygon::from_origin_and_vectors(origin, vecs);
/// assert_eq!(poly.origin, Point::new(0, 0));
/// // Note: vecs field is private, so we can't access it directly in documentation
/// ```
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
    ///   coordinates of the points that define the polygon. The first element of the array (`coords[0]`)
    ///   is considered as the origin of the polygon, and the remaining elements represent the vectors
    ///   from the origin to the
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
        let vecs = coords.iter().map(|pt| *pt - origin).collect();
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
        Self::new(pointset)
    }

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

impl<T: Clone + Copy + Num + Ord + AddAssign + SubAssign> RPolygon<T> {
    /// Converts this rectilinear polygon to a general polygon.
    ///
    /// Inserts intermediate axis-aligned points for any non-rectilinear
    /// segment transitions.
    pub fn to_polygon(&self) -> crate::polygon::Polygon<T> {
        rpolygon_to_polygon(self)
    }
}

/// Converts a rectilinear polygon to a general polygon.
///
/// Adds intermediate points for axis-aligned segments.
pub fn rpolygon_to_polygon<T: Clone + Copy + Num + Ord + AddAssign + SubAssign>(
    rpoly: &RPolygon<T>,
) -> crate::polygon::Polygon<T> {
    use crate::polygon::Polygon;
    use crate::vector2::Vector2;

    let _vecs = &rpoly.vecs;
    if _vecs.is_empty() {
        return Polygon::from_origin_and_vectors(rpoly.origin, vec![]);
    }

    let mut new_vecs: Vec<Vector2<T, T>> = Vec::new();
    let mut current = Vector2::new(T::zero(), T::zero());

    for &next in _vecs {
        if current.x_ != next.x_ && current.y_ != next.y_ {
            new_vecs.push(Vector2::new(next.x_, current.y_));
        }
        new_vecs.push(next);
        current = next;
    }

    // Handle closing segment back to origin
    let origin_vec = Vector2::new(T::zero(), T::zero());
    if current.x_ != origin_vec.x_ && current.y_ != origin_vec.y_ {
        new_vecs.push(Vector2::new(origin_vec.x_, current.y_));
    }

    Polygon::from_origin_and_vectors(rpoly.origin, new_vecs)
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
    ///   two-dimensional space.
    /// * `f`: The parameter `f` is a closure that takes a reference to a reference of a `Point<T, T>` and
    ///   returns a tuple of two values of type `T`. The closure is used to determine the ordering of the
    ///   points in the `pointset`. The first value of the tuple represents the x-coordinate
    pub fn create_mono_rpolygon<F>(pointset: &[Point<T, T>], func: F) -> (Vec<Point<T, T>>, bool)
    where
        F: Fn(&Point<T, T>) -> (T, T),
    {
        // Use x-mono as model
        let rightmost = pointset
            .iter()
            .max_by(|a, b| func(a).partial_cmp(&func(b)).unwrap())
            .unwrap();
        let leftmost = pointset
            .iter()
            .min_by(|a, b| func(a).partial_cmp(&func(b)).unwrap())
            .unwrap();
        let is_anticlockwise = func(rightmost).1 <= func(leftmost).1;
        let (mut lst1, mut lst2): (Vec<Point<T, T>>, Vec<Point<T, T>>) = if is_anticlockwise {
            pointset
                .iter()
                .partition(|pt| func(pt).1 <= func(leftmost).1)
        } else {
            pointset
                .iter()
                .partition(|pt| func(pt).1 >= func(leftmost).1)
        };
        lst1.sort_by_key(|a| func(a));
        lst2.sort_by_key(|a| func(a));
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
    ///   xcoord.
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
    ///   coordinates.
    /// * `q`: The parameter `q` represents the point that we want to determine if it is within the
    ///   polygon or not.
    ///
    /// Returns:
    ///
    /// The function `point_in_polygon` returns a boolean value. It returns `true` if the given point
    /// `q` is strictly inside the polygon defined by the `pointset` array, `false` if the point is
    /// strictly outside the polygon, and `ub` (undefined behavior) if the point lies on the boundary of
    /// the polygon.
    pub fn point_in_rpolygon(pointset: &[Point<T, T>], query_pt: &Point<T, T>) -> bool {
        let mut result = false;
        let n = pointset.len();
        let mut pt0 = &pointset[n - 1];
        for pt1 in pointset.iter() {
            if ((pt1.ycoord <= query_pt.ycoord && query_pt.ycoord < pt0.ycoord)
                || (pt0.ycoord <= query_pt.ycoord && query_pt.ycoord < pt1.ycoord))
                && pt1.xcoord > query_pt.xcoord
            {
                result = !result;
            }
            pt0 = pt1;
        }
        result
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

// Test helpers (also used by standalone area tests below)
#[cfg(test)]
fn test_pointset() -> Vec<Point<i32, i32>> {
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
    coords.iter().map(|(x, y)| Point::<i32, i32>::new(*x, *y)).collect()
}

#[cfg(test)]
fn make_test_rpolygon() -> (Vec<Point<i32, i32>>, RPolygon<i32>) {
    let pointset = test_pointset();
    let (poly_points, _is_cw) = RPolygon::<i32>::create_xmono_rpolygon(&pointset);
    let rpoly = RPolygon::<i32>::new(&poly_points);
    (poly_points, rpoly)
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
        for (x_coord, y_coord) in coords.iter() {
            pointset.push(Point::<i32, i32>::new(*x_coord, *y_coord));
        }
        let (poly_points, is_cw) = RPolygon::<i32>::create_ymono_rpolygon(&pointset);
        assert!(rpolygon_is_anticlockwise(&poly_points));
        assert!(rpolygon_is_ymonotone(&poly_points));
        assert!(!rpolygon_is_xmonotone(&poly_points));
        for pt in poly_points.iter() {
            print!("({}, {}) ", pt.xcoord, pt.ycoord);
        }
        let poly = RPolygon::<i32>::new(&poly_points);
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
        for (x_coord, y_coord) in coords.iter() {
            pointset.push(Point::<i32, i32>::new(*x_coord, *y_coord));
        }
        let (poly_points, is_anticw) = RPolygon::<i32>::create_xmono_rpolygon(&pointset);
        assert!(!rpolygon_is_anticlockwise(&poly_points));
        assert!(rpolygon_is_xmonotone(&poly_points));
        assert!(!rpolygon_is_ymonotone(&poly_points));
        for pt in poly_points.iter() {
            print!("({}, {}) ", pt.xcoord, pt.ycoord);
        }
        let poly = RPolygon::<i32>::new(&poly_points);
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
        for (x_coord, y_coord) in coords.iter() {
            pointset.push(Point::<i32, i32>::new(*x_coord, *y_coord));
        }
        let query_pt = Point::<i32, i32>::new(0, -3);
        // let poly = RPolygon::<i32>::new(&pointset);
        assert!(!RPolygon::<i32>::point_in_rpolygon(&pointset, &query_pt));
    }

    #[test]
    fn test_signed_area_more_cases() {
        let pt1 = Point::new(0, 0);
        let pt2 = Point::new(1, 0);
        let pt3 = Point::new(1, 1);
        let pt4 = Point::new(0, 1);
        let poly = RPolygon::new(&[pt1, pt2, pt3, pt4]);
        assert_eq!(poly.signed_area(), 1);

        let pt5 = Point::new(0, 0);
        let pt6 = Point::new(0, 1);
        let pt7 = Point::new(1, 1);
        let pt8 = Point::new(1, 0);
        let poly2 = RPolygon::new(&[pt5, pt6, pt7, pt8]);
        assert_eq!(poly2.signed_area(), -1);
    }

    #[test]
    fn test_point_in_rpolygon_more_cases() {
        let pt1 = Point::new(0, 0);
        let pt2 = Point::new(1, 0);
        let pt3 = Point::new(1, 1);
        let pt4 = Point::new(0, 1);
        let pointset = &[pt1, pt2, pt3, pt4];

        let query_pt1 = Point::new(0, 0);
        assert!(RPolygon::<i32>::point_in_rpolygon(pointset, &query_pt1));

        let query_pt2 = Point::new(1, 1);
        assert!(!RPolygon::<i32>::point_in_rpolygon(pointset, &query_pt2));

        let query_pt3 = Point::new(0, 1);
        assert!(!RPolygon::<i32>::point_in_rpolygon(pointset, &query_pt3));

        let query_pt4 = Point::new(1, 0);
        assert!(!RPolygon::<i32>::point_in_rpolygon(pointset, &query_pt4));
    }
}

#[test]
fn test_rpolygon_is_xmonotone() {
    // Test x-monotone polygon
    let coords = [(0, 0), (1, 0), (2, 0), (2, 1), (1, 1), (0, 1)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    assert!(rpolygon_is_xmonotone(&pointset));

    // Test non-x-monotone polygon
    let coords2 = [(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)];
    let pointset2: Vec<Point<i32, i32>> = coords2.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    assert!(!rpolygon_is_xmonotone(&pointset2));
}

#[test]
fn test_rpolygon_is_ymonotone() {
    // Test y-monotone polygon
    let coords = [(0, 0), (0, 1), (0, 2), (1, 2), (1, 1), (1, 0)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    assert!(rpolygon_is_ymonotone(&pointset));

    // Test non-y-monotone polygon
    let coords2 = [(0, 0), (0, 2), (1, 1), (2, 2), (2, 0)];
    let pointset2: Vec<Point<i32, i32>> = coords2.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    assert!(!rpolygon_is_ymonotone(&pointset2));
}

#[test]
fn test_rpolygon_is_convex() {
    // Test rectilinearly convex polygon
    let coords = [(0, 0), (0, 2), (2, 2), (2, 0)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    assert!(rpolygon_is_convex(&pointset));

    // Test non-convex polygon
    let coords2 = [(0, 0), (0, 2), (1, 1), (2, 2), (2, 0)];
    let pointset2: Vec<Point<i32, i32>> = coords2.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    assert!(!rpolygon_is_convex(&pointset2));
}

#[test]
fn test_rpolygon_vertices() {
    let pt1 = Point::new(0, 0);
    let pt2 = Point::new(1, 0);
    let pt3 = Point::new(1, 1);
    let poly = RPolygon::new(&[pt1, pt2, pt3]);
    let vertices = poly.vertices();
    assert_eq!(vertices.len(), 3);
    assert_eq!(vertices[0], pt1);
    assert_eq!(vertices[1], pt2);
    assert_eq!(vertices[2], pt3);
}

#[test]
fn test_rpolygon_bounding_box() {
    let pt1 = Point::new(0, 0);
    let pt2 = Point::new(2, 0);
    let pt3 = Point::new(2, 2);
    let pt4 = Point::new(0, 2);
    let poly = RPolygon::new(&[pt1, pt2, pt3, pt4]);
    let (min, max) = poly.bounding_box();
    assert_eq!(min, Point::new(0, 0));
    assert_eq!(max, Point::new(2, 2));
}

#[test]
fn test_rpolygon_add_assign() {
    let pt1 = Point::new(0, 0);
    let pt2 = Point::new(1, 0);
    let pt3 = Point::new(1, 1);
    let mut poly = RPolygon::new(&[pt1, pt2, pt3]);
    poly.add_assign(Vector2::new(1, 1));
    let vertices = poly.vertices();
    assert_eq!(vertices[0], Point::new(1, 1));
}

#[test]
fn test_rpolygon_sub_assign() {
    let pt1 = Point::new(1, 1);
    let pt2 = Point::new(2, 1);
    let pt3 = Point::new(2, 2);
    let mut poly = RPolygon::new(&[pt1, pt2, pt3]);
    poly.sub_assign(Vector2::new(1, 1));
    let vertices = poly.vertices();
    assert_eq!(vertices[0], Point::new(0, 0));
}

#[test]
fn test_rpolygon_is_rectilinear() {
    let pt1 = Point::new(0, 0);
    let pt2 = Point::new(1, 0);
    let pt3 = Point::new(1, 1);
    let pt4 = Point::new(0, 1);
    let poly = RPolygon::new(&[pt1, pt2, pt3, pt4]);
    assert!(poly.is_rectilinear());
}

#[test]
fn test_rpolygon_from_origin_and_vectors() {
    let origin = Point::new(0, 0);
    let vecs = vec![Vector2::new(1, 0), Vector2::new(1, 1), Vector2::new(0, 1)];
    let poly = RPolygon::from_origin_and_vectors(origin, vecs);
    assert_eq!(poly.origin, Point::new(0, 0));
    let vertices = poly.vertices();
    assert_eq!(vertices.len(), 4);
}

#[test]
fn test_rpolygon_from_pointset() {
    let pointset = [Point::new(0, 0), Point::new(1, 0), Point::new(1, 1)];
    let poly = RPolygon::from_pointset(&pointset);
    assert_eq!(poly.origin, Point::new(0, 0));
}

#[test]
fn test_rpolygon_partial_eq() {
    let pt1 = Point::new(0, 0);
    let pt2 = Point::new(1, 0);
    let pt3 = Point::new(1, 1);
    let poly1 = RPolygon::new(&[pt1, pt2, pt3]);
    let poly2 = RPolygon::new(&[pt1, pt2, pt3]);
    assert_eq!(poly1, poly2);

    let poly3 = RPolygon::new(&[pt1, pt2, Point::new(0, 1)]);
    assert_ne!(poly1, poly3);
}

#[test]
fn test_rpolygon_is_anticlockwise_standalone() {
    let pointset = [Point::new(0, 0), Point::new(1, 0), Point::new(1, 1)];
    assert!(rpolygon_is_anticlockwise(&pointset));
}

#[test]
fn test_rpolygon_default() {
    let poly: RPolygon<i32> = RPolygon::default();
    assert_eq!(poly.origin, Point::new(0, 0));
}

// ============================================================
// Area-based algorithm verification tests
// ============================================================

#[cfg(test)]
use crate::rpolygon_cut::{rpolygon_cut_convex, rpolygon_cut_explicit, rpolygon_cut_implicit, rpolygon_cut_rectangle};
#[cfg(test)]
use crate::rpolygon_hull::{rpolygon_make_convex_hull, rpolygon_make_xmonotone_hull, rpolygon_make_ymonotone_hull};

/// Test that to_polygon preserves signed area for a rectilinear polygon.
#[test]
fn test_to_polygon_area_preservation() {
    let (_points, rpoly) = make_test_rpolygon();
    let original_area = rpoly.signed_area();
    let poly = rpoly.to_polygon();
    let poly_area = poly.signed_area_x2();
    // For rectilinear polygons, the polygon signed_area_x2 should be
    // 2x the rpolygon signed_area (both use the same winding).
    // signed_area_x2 returns the doubled signed area.
    // RPolygon::signed_area returns area (not doubled).
    // We verify: original_area * 2 == poly_area (for same orientation)
    assert_eq!(
        original_area * 2,
        poly_area,
        "to_polygon should preserve area: RPolygon area {} * 2 = {}, Polygon signed_area_x2 = {}",
        original_area,
        original_area * 2,
        poly_area
    );
}

/// Test that to_polygon on simple rectangles preserves area.
#[test]
fn test_to_polygon_rectangle_area() {
    // Unit square (anticlockwise)
    let pts = [
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(0, 1),
    ];
    let rpoly = RPolygon::new(&pts);
    let area = rpoly.signed_area();
    let poly = rpoly.to_polygon();
    assert_eq!(area * 2, poly.signed_area_x2());
}

/// Test that rpolygon_cut_convex preserves total signed area.
#[test]
fn test_rpolygon_cut_convex_area_preservation() {
    let (points, rpoly) = make_test_rpolygon();
    let original_area = rpoly.signed_area();
    let is_anticw = rpolygon_is_anticlockwise(&points);

    let convex_pieces = rpolygon_cut_convex(&points, is_anticw);
    assert!(!convex_pieces.is_empty(), "Should produce at least one piece");

    let total_pieces_area: i32 = convex_pieces
        .iter()
        .map(|piece| RPolygon::new(piece).signed_area())
        .sum();

    assert_eq!(
        original_area, total_pieces_area,
        "Convex cut should preserve total signed area: original={}, total={}",
        original_area, total_pieces_area
    );
}

/// Test that rpolygon_cut_explicit preserves total signed area.
#[test]
fn test_rpolygon_cut_explicit_area_preservation() {
    let (points, _rpoly) = make_test_rpolygon();
    let hull = rpolygon_make_convex_hull(&points, rpolygon_is_anticlockwise(&points));
    let hull_area = RPolygon::new(&hull).signed_area();
    let is_anticw = rpolygon_is_anticlockwise(&hull);

    let pieces = rpolygon_cut_explicit(&hull, is_anticw);
    assert!(!pieces.is_empty(), "Should produce at least one piece");

    let total_pieces_area: i32 = pieces
        .iter()
        .map(|piece| RPolygon::new(piece).signed_area())
        .sum();

    assert_eq!(
        hull_area, total_pieces_area,
        "Explicit cut should preserve hull area: hull={}, total={}",
        hull_area, total_pieces_area
    );
}

/// Test that rpolygon_cut_implicit preserves total signed area.
#[test]
fn test_rpolygon_cut_implicit_area_preservation() {
    let (points, _rpoly) = make_test_rpolygon();
    let hull = rpolygon_make_convex_hull(&points, rpolygon_is_anticlockwise(&points));
    let hull_area = RPolygon::new(&hull).signed_area();
    let is_anticw = rpolygon_is_anticlockwise(&hull);

    let pieces = rpolygon_cut_implicit(&hull, is_anticw);
    assert!(!pieces.is_empty(), "Should produce at least one piece");

    let total_pieces_area: i32 = pieces
        .iter()
        .map(|piece| RPolygon::new(piece).signed_area())
        .sum();

    assert_eq!(
        hull_area, total_pieces_area,
        "Implicit cut should preserve hull area: hull={}, total={}",
        hull_area, total_pieces_area
    );
}

/// Test that rpolygon_cut_rectangle preserves total signed area.
#[test]
fn test_rpolygon_cut_rectangle_area_preservation() {
    let (points, _rpoly) = make_test_rpolygon();
    let hull = rpolygon_make_convex_hull(&points, rpolygon_is_anticlockwise(&points));
    let hull_area = RPolygon::new(&hull).signed_area();
    let is_anticw = rpolygon_is_anticlockwise(&hull);

    let pieces = rpolygon_cut_rectangle(&hull, is_anticw);
    assert!(!pieces.is_empty(), "Should produce at least one piece");

    let total_pieces_area: i32 = pieces
        .iter()
        .map(|piece| RPolygon::new(piece).signed_area())
        .sum();

    assert_eq!(
        hull_area, total_pieces_area,
        "Rectangle cut should preserve hull area: hull={}, total={}",
        hull_area, total_pieces_area
    );
}

/// Verifies hull area is >= original area for multiple shapes.
/// This is a fundamental property: convex hull area >= original polygon area.
#[test]
fn test_convex_hull_area_larger_than_original() {
    // Test with the complex x-monotone rpolygon
    let (points, rpoly) = make_test_rpolygon();
    let orig_area: i32 = rpoly.signed_area();
    let orig_abs = orig_area.unsigned_abs();
    let hull = rpolygon_make_convex_hull(&points, rpolygon_is_anticlockwise(&points));
    let hull_rpoly = RPolygon::new(&hull);
    let hull_area: i32 = hull_rpoly.signed_area();
    let hull_abs = hull_area.unsigned_abs();
    assert!(
        hull_abs >= orig_abs,
        "Convex hull area ({}) should be >= original area ({}) for complex polygon",
        hull_abs,
        orig_abs
    );

    // Test with L-shape
    let l_pts = [
        Point::new(0, 0),
        Point::new(3, 0),
        Point::new(3, 1),
        Point::new(1, 1),
        Point::new(1, 2),
        Point::new(0, 2),
    ];
    let l_area: i32 = RPolygon::new(&l_pts).signed_area();
    let l_abs = l_area.unsigned_abs();
    let l_hull = rpolygon_make_convex_hull(&l_pts, rpolygon_is_anticlockwise(&l_pts));
    let l_hull_area: i32 = RPolygon::new(&l_hull).signed_area();
    let l_hull_abs = l_hull_area.unsigned_abs();
    assert!(
        l_hull_abs >= l_abs,
        "Convex hull area ({}) should be >= L-shape area ({})",
        l_hull_abs,
        l_abs
    );

    // Test with unit square (already convex, hull == original)
    let sq_pts = [
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(0, 1),
    ];
    let sq_area: i32 = RPolygon::new(&sq_pts).signed_area();
    let sq_abs = sq_area.unsigned_abs();
    let sq_hull = rpolygon_make_convex_hull(&sq_pts, rpolygon_is_anticlockwise(&sq_pts));
    let sq_hull_area: i32 = RPolygon::new(&sq_hull).signed_area();
    let sq_hull_abs = sq_hull_area.unsigned_abs();
    assert_eq!(
        sq_hull_abs, sq_abs,
        "Convex hull of a convex square should have same area"
    );
}

/// Square polygon tests for cut operations.
#[test]
fn test_square_cut_operations() {
    let pts = [
        Point::new(0, 0),
        Point::new(2, 0),
        Point::new(2, 2),
        Point::new(0, 2),
    ];
    let area = RPolygon::new(&pts).signed_area();
    let is_anticw = rpolygon_is_anticlockwise(&pts);

    // A square is already convex, so cut should return the square itself
    let convex = rpolygon_cut_convex(&pts, is_anticw);
    let total: i32 = convex.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Convex cut of square should preserve area");

    let explicit = rpolygon_cut_explicit(&pts, is_anticw);
    let total: i32 = explicit.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Explicit cut of square should preserve area");

    let implicit = rpolygon_cut_implicit(&pts, is_anticw);
    let total: i32 = implicit.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Implicit cut of square should preserve area");

    let rect = rpolygon_cut_rectangle(&pts, is_anticw);
    let total: i32 = rect.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Rectangle cut of square should preserve area");
}

/// L-shaped polygon tests for cut operations.
#[test]
fn test_lshape_cut_operations() {
    // L-shape: a rectilinear polygon that is not convex
    let pts = [
        Point::new(0, 0),
        Point::new(3, 0),
        Point::new(3, 1),
        Point::new(1, 1),
        Point::new(1, 2),
        Point::new(0, 2),
    ];
    let area = RPolygon::new(&pts).signed_area();
    let is_anticw = rpolygon_is_anticlockwise(&pts);

    let convex = rpolygon_cut_convex(&pts, is_anticw);
    let total: i32 = convex.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Convex cut of L-shape should preserve area");
    assert!(!convex.is_empty());

    let explicit = rpolygon_cut_explicit(&pts, is_anticw);
    let total: i32 = explicit.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Explicit cut of L-shape should preserve area");

    let implicit = rpolygon_cut_implicit(&pts, is_anticw);
    let total: i32 = implicit.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Implicit cut of L-shape should preserve area");

    let rect = rpolygon_cut_rectangle(&pts, is_anticw);
    let total: i32 = rect.iter().map(|p| RPolygon::new(p).signed_area()).sum();
    assert_eq!(area, total, "Rectangle cut of L-shape should preserve area");
}
