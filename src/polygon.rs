#![allow(clippy::type_complexity)]

use num_traits::Num;
use std::cmp::Ordering;
use std::ops::{AddAssign, SubAssign};

use crate::point::Point;
use crate::vector2::Vector2;

/// Represents an arbitrary polygon with coordinates of type T
///
/// The `Polygon` struct stores the origin point and a vector of edges that define the polygon.
/// It provides various operations and functionalities for working with polygons, such as
/// area calculation, point containment checks, and geometric property verification.
///
/// ```svgbob
///       *-----*-----*
///      /     / \   \
///     /     /   \   \
///    *-----*     *---*
///    |  origin
///    *--> vecs[0]
/// ```
///
/// Properties:
///
/// * `origin`: The origin point of the polygon
/// * `vecs`: Vector of displacement vectors from origin to other vertices
///
/// # Examples
///
/// ```
/// use physdes::point::Point;
/// use physdes::polygon::Polygon;
/// use physdes::vector2::Vector2;
///
/// let origin = Point::new(0, 0);
/// let vecs = vec![Vector2::new(1, 0), Vector2::new(1, 1), Vector2::new(0, 1)];
/// let poly = Polygon::from_origin_and_vectors(origin, vecs);
/// assert_eq!(poly.origin, Point::new(0, 0));
/// assert_eq!(poly.vecs.len(), 3);
/// ```
#[derive(Eq, Clone, Debug, Default)]
pub struct Polygon<T> {
    pub origin: Point<T, T>,
    pub vecs: Vec<Vector2<T, T>>,
}

impl<T: Clone + Num + Ord + Copy + std::ops::AddAssign> Polygon<T> {
    /// Constructs a new Polygon from a set of points
    ///
    /// The first point in the slice is used as the origin, and the remaining points
    /// are used to construct displacement vectors relative to the origin.
    ///
    /// # Arguments
    ///
    /// * `coords` - A slice of points representing the vertices of the polygon
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::polygon::Polygon;
    /// use physdes::vector2::Vector2;
    ///
    /// let p1 = Point::new(1, 1);
    /// let p2 = Point::new(2, 2);
    /// let p3 = Point::new(3, 3);
    /// let p4 = Point::new(4, 4);
    /// let p5 = Point::new(5, 5);
    /// let poly = Polygon::new(&[p1, p2, p3, p4, p5]);
    /// assert_eq!(poly.origin, Point::new(1, 1));
    /// assert_eq!(poly.vecs.len(), 4);
    /// assert_eq!(poly.vecs[0], Vector2::new(1, 1));
    /// ```
    pub fn new(coords: &[Point<T, T>]) -> Self {
        let (&origin, coords) = coords.split_first().unwrap();
        let vecs = coords.iter().map(|pt| *pt - origin).collect();
        Polygon { origin, vecs }
    }

    /// Constructs a new Polygon from origin and displacement vectors
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point of the polygon
    /// * `vecs` - Vector of displacement vectors from origin
    pub fn from_origin_and_vectors(origin: Point<T, T>, vecs: Vec<Vector2<T, T>>) -> Self {
        Polygon { origin, vecs }
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

    /// Calculates the signed area of the polygon multiplied by 2
    ///
    /// This function calculates the signed area by summing the cross products
    /// of adjacent edges. The result is multiplied by 2 to avoid the need for
    /// floating-point arithmetic.
    ///
    /// # Returns
    ///
    /// The signed area of the polygon multiplied by 2
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::polygon::Polygon;
    /// use physdes::vector2::Vector2;
    ///
    /// let p1 = Point::new(1, 1);
    /// let p2 = Point::new(2, 2);
    /// let p3 = Point::new(3, 3);
    /// let p4 = Point::new(4, 4);
    /// let p5 = Point::new(5, 5);
    /// let poly = Polygon::new(&[p1, p2, p3, p4, p5]);
    /// assert_eq!(poly.signed_area_x2(), 0);
    /// ```
    pub fn signed_area_x2(&self) -> T {
        let vecs = &self.vecs;
        let n = vecs.len();
        if n < 2 {
            return T::zero();
        }

        let mut itr = vecs.iter();
        let vec0 = itr.next().unwrap();
        let vec1 = itr.next().unwrap();
        let mut res = vec0.x_ * vec1.y_ - vecs[n - 1].x_ * vecs[n - 2].y_;

        let mut vec0 = vec0;
        let mut vec1 = vec1;

        for vec2 in itr {
            res += vec1.x_ * (vec2.y_ - vec0.y_);
            vec0 = vec1;
            vec1 = vec2;
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
    /// use physdes::polygon::Polygon;
    ///
    /// let p1 = Point::new(0, 0);
    /// let p2 = Point::new(0, 1);
    /// let p3 = Point::new(1, 1);
    /// let p4 = Point::new(1, 0);
    /// let poly = Polygon::new(&[p1, p2, p3, p4]);
    /// assert!(poly.is_rectilinear());
    ///
    /// let p5 = Point::new(0, 0);
    /// let p6 = Point::new(1, 1);
    /// let p7 = Point::new(0, 2);
    /// let poly2 = Polygon::new(&[p5, p6, p7]);
    /// assert!(!poly2.is_rectilinear());
    /// ```
    pub fn is_rectilinear(&self) -> bool {
        if self.vecs.is_empty() {
            return true;
        }

        // Check from origin to vecs[0]
        if self.vecs[0].x_ != T::zero() && self.vecs[0].y_ != T::zero() {
            return false;
        }

        // Check between vecs
        for i in 0..self.vecs.len() - 1 {
            let v1 = self.vecs[i];
            let v2 = self.vecs[i + 1];
            if v1.x_ != v2.x_ && v1.y_ != v2.y_ {
                return false;
            }
        }

        // Check from vecs[-1] to origin
        let last_vec = self.vecs.last().unwrap();
        last_vec.x_ == T::zero() || last_vec.y_ == T::zero()
    }

    /// Checks if the polygon is oriented anticlockwise
    pub fn is_anticlockwise(&self) -> bool
    where
        T: PartialOrd,
    {
        let mut pointset = Vec::with_capacity(self.vecs.len() + 1);
        pointset.push(Vector2::new(T::zero(), T::zero()));
        pointset.extend(self.vecs.iter().cloned());

        if pointset.len() < 3 {
            panic!("Polygon must have at least 3 points");
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
        let next_point = pointset[(min_index + 1) % n];

        // Calculate cross product
        (current_point - prev_point).cross(&(next_point - current_point)) > T::zero()
    }

    /// Checks if the polygon is convex
    pub fn is_convex(&self) -> bool
    where
        T: PartialOrd,
    {
        let n = self.vecs.len() + 1;
        if n < 3 {
            return false;
        }
        if n == 3 {
            return true;
        }

        let is_anticlockwise = self.is_anticlockwise();

        // Create extended pointset for easier edge traversal
        let mut pointset = Vec::with_capacity(n + 2);
        pointset.push(*self.vecs.last().unwrap());
        pointset.push(Vector2::new(T::zero(), T::zero()));
        pointset.extend(self.vecs.iter().cloned());
        pointset.push(Vector2::new(T::zero(), T::zero()));

        if is_anticlockwise {
            for i in 1..pointset.len() - 1 {
                let v1 = pointset[i] - pointset[i - 1];
                let v2 = pointset[i + 1] - pointset[i];
                if v1.cross(&v2) < T::zero() {
                    return false;
                }
            }
        } else {
            for i in 1..pointset.len() - 1 {
                let v1 = pointset[i] - pointset[i - 1];
                let v2 = pointset[i + 1] - pointset[i];
                if v1.cross(&v2) > T::zero() {
                    return false;
                }
            }
        }

        true
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
}

/// Creates a monotone polygon from a set of points using a custom comparison function
///
/// # Arguments
///
/// * `pointset` - A slice of points to create the polygon from
/// * `f` - A closure that defines the ordering of points
pub fn create_mono_polygon<T, F>(pointset: &[Point<T, T>], f: F) -> Vec<Point<T, T>>
where
    T: Clone + Num + Ord + Copy + PartialOrd,
    F: Fn(&Point<T, T>) -> (T, T),
{
    let max_pt = pointset
        .iter()
        .max_by(|a, b| f(a).partial_cmp(&f(b)).unwrap())
        .unwrap();
    let min_pt = pointset
        .iter()
        .min_by(|a, b| f(a).partial_cmp(&f(b)).unwrap())
        .unwrap();
    let d = *max_pt - *min_pt;

    let (mut lst1, mut lst2): (Vec<Point<T, T>>, Vec<Point<T, T>>) = pointset
        .iter()
        .partition(|&a| d.cross(&(*a - *min_pt)) <= T::zero());

    lst1.sort_by_key(|a| f(a));
    lst2.sort_by_key(|a| f(a));
    lst2.reverse();
    lst1.append(&mut lst2);
    lst1
}

/// Creates an x-monotone polygon from a set of points
///
/// Points are ordered primarily by x-coordinate, secondarily by y-coordinate
#[inline]
pub fn create_xmono_polygon<T>(pointset: &[Point<T, T>]) -> Vec<Point<T, T>>
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    create_mono_polygon(pointset, |a| (a.xcoord, a.ycoord))
}

/// Creates a y-monotone polygon from a set of points
///
/// Points are ordered primarily by y-coordinate, secondarily by x-coordinate
#[inline]
pub fn create_ymono_polygon<T>(pointset: &[Point<T, T>]) -> Vec<Point<T, T>>
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    create_mono_polygon(pointset, |a| (a.ycoord, a.xcoord))
}

/// Checks if a polygon is monotone in a given direction
pub fn polygon_is_monotone<T, F>(lst: &[Point<T, T>], dir: F) -> bool
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
        .min_by(|(_, a), (_, b)| dir(a).partial_cmp(&dir(b)).unwrap())
        .unwrap();

    let (max_index, _) = lst
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| dir(a).partial_cmp(&dir(b)).unwrap())
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
pub fn polygon_is_xmonotone<T>(lst: &[Point<T, T>]) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    polygon_is_monotone(lst, |pt| (pt.xcoord, pt.ycoord))
}

/// Checks if a polygon is y-monotone
pub fn polygon_is_ymonotone<T>(lst: &[Point<T, T>]) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    polygon_is_monotone(lst, |pt| (pt.ycoord, pt.xcoord))
}

/// Determines if a point is inside a polygon using the winding number algorithm
pub fn point_in_polygon<T>(pointset: &[Point<T, T>], ptq: &Point<T, T>) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    let n = pointset.len();
    if n == 0 {
        return false;
    }

    let mut pt0 = &pointset[n - 1];
    let mut res = false;

    for pt1 in pointset.iter() {
        if (pt1.ycoord <= ptq.ycoord && ptq.ycoord < pt0.ycoord)
            || (pt0.ycoord <= ptq.ycoord && ptq.ycoord < pt1.ycoord)
        {
            let det = (*ptq - *pt0).cross(&(*pt1 - *pt0));
            if pt1.ycoord > pt0.ycoord {
                if det < T::zero() {
                    res = !res;
                }
            } else if det > T::zero() {
                res = !res;
            }
        }
        pt0 = pt1;
    }

    res
}

/// Determines if a polygon represented by points is oriented anticlockwise
pub fn polygon_is_anticlockwise<T>(pointset: &[Point<T, T>]) -> bool
where
    T: Clone + Num + Ord + Copy + PartialOrd,
{
    if pointset.len() < 3 {
        panic!("Polygon must have at least 3 points");
    }

    // Find the point with minimum coordinates
    let (min_index, _) = pointset
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
    let prev_point = pointset[(min_index + n - 1) % n];
    let current_point = pointset[min_index];
    let next_point = pointset[(min_index + 1) % n];

    // Calculate cross product
    (current_point - prev_point).cross(&(next_point - current_point)) > T::zero()
}

// Implement PartialEq for Polygon
impl<T: PartialEq> PartialEq for Polygon<T> {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.vecs == other.vecs
    }
}

// Implement AddAssign and SubAssign for Polygon
impl<T: AddAssign + Clone + Num> AddAssign<Vector2<T, T>> for Polygon<T> {
    fn add_assign(&mut self, rhs: Vector2<T, T>) {
        self.origin += rhs;
    }
}

impl<T: SubAssign + Clone + Num> SubAssign<Vector2<T, T>> for Polygon<T> {
    fn sub_assign(&mut self, rhs: Vector2<T, T>) {
        self.origin -= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::vector2::Vector2;

    #[test]
    fn test_polygon() {
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
            (-3, -3),
            (3, 3),
            (-3, -4),
            (1, 4),
        ];

        let mut pointset = Vec::new();
        for (x, y) in coords.iter() {
            pointset.push(Point::new(*x, *y));
        }

        let s = create_xmono_polygon(&pointset);
        assert!(polygon_is_anticlockwise(&s));

        let p = Polygon::from_pointset(&s);
        let mut q = Polygon::from_pointset(&s);
        q.add_assign(Vector2::new(4, 5));
        q.sub_assign(Vector2::new(4, 5));
        assert_eq!(q, p);
    }

    #[test]
    fn test_ymono_polygon() {
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
            (-3, -3),
            (3, 3),
            (-3, -4),
            (1, 4),
        ];

        let mut pointset = Vec::new();
        for (x, y) in coords.iter() {
            pointset.push(Point::new(*x, *y));
        }

        let s = create_ymono_polygon(&pointset);
        assert!(polygon_is_ymonotone(&s));
        assert!(!polygon_is_xmonotone(&s));
        assert!(polygon_is_anticlockwise(&s));

        let p = Polygon::from_pointset(&s);
        assert_eq!(p.signed_area_x2(), 102);
        assert!(p.is_anticlockwise());
    }

    #[test]
    fn test_xmono_polygon() {
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
            (-3, -3),
            (3, 3),
            (-3, -4),
            (1, 4),
        ];

        let mut pointset = Vec::new();
        for (x, y) in coords.iter() {
            pointset.push(Point::new(*x, *y));
        }

        let s = create_xmono_polygon(&pointset);
        assert!(polygon_is_xmonotone(&s));
        assert!(!polygon_is_ymonotone(&s));
        assert!(polygon_is_anticlockwise(&s));

        let p = Polygon::from_pointset(&s);
        assert_eq!(p.signed_area_x2(), 111);
        assert!(p.is_anticlockwise());
    }

    #[test]
    fn test_is_rectilinear() {
        // Create a rectilinear polygon
        let rectilinear_coords = [(0, 0), (0, 1), (1, 1), (1, 0)];
        let rectilinear_points: Vec<Point<i32, i32>> = rectilinear_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let rectilinear_polygon = Polygon::from_pointset(&rectilinear_points);
        assert!(rectilinear_polygon.is_rectilinear());

        // Create a non-rectilinear polygon
        let non_rectilinear_coords = [(0, 0), (1, 1), (2, 0)];
        let non_rectilinear_points: Vec<Point<i32, i32>> = non_rectilinear_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let non_rectilinear_polygon = Polygon::from_pointset(&non_rectilinear_points);
        assert!(!non_rectilinear_polygon.is_rectilinear());
    }

    #[test]
    fn test_is_convex() {
        // Test case 1: Convex polygon
        let convex_coords = [(0, 0), (2, 0), (2, 2), (0, 2)];
        let convex_points: Vec<Point<i32, i32>> = convex_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let convex_polygon = Polygon::from_pointset(&convex_points);
        assert!(convex_polygon.is_convex());

        // Test case 2: Non-convex polygon
        let non_convex_coords = [(0, 0), (2, 0), (1, 1), (2, 2), (0, 2)];
        let non_convex_points: Vec<Point<i32, i32>> = non_convex_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let non_convex_polygon = Polygon::from_pointset(&non_convex_points);
        assert!(!non_convex_polygon.is_convex());

        // Test case 3: Triangle (always convex)
        let triangle_coords = [(0, 0), (2, 0), (1, 2)];
        let triangle_points: Vec<Point<i32, i32>> = triangle_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let triangle = Polygon::from_pointset(&triangle_points);
        assert!(triangle.is_convex());
    }

    #[test]
    fn test_is_anticlockwise() {
        // Clockwise polygon
        let clockwise_coords = [(0, 0), (0, 1), (1, 1), (1, 0)];
        let clockwise_points: Vec<Point<i32, i32>> = clockwise_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let clockwise_polygon = Polygon::from_pointset(&clockwise_points);
        assert!(!clockwise_polygon.is_anticlockwise());

        // Counter-clockwise polygon
        let counter_clockwise_coords = [(0, 0), (1, 0), (1, 1), (0, 1)];
        let counter_clockwise_points: Vec<Point<i32, i32>> = counter_clockwise_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let counter_clockwise_polygon = Polygon::from_pointset(&counter_clockwise_points);
        assert!(counter_clockwise_polygon.is_anticlockwise());
    }

    #[test]
    fn test_is_convex_clockwise() {
        // Convex clockwise polygon
        let convex_coords = [(0, 0), (0, 2), (2, 2), (2, 0)];
        let convex_points: Vec<Point<i32, i32>> = convex_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let convex_polygon = Polygon::from_pointset(&convex_points);
        assert!(convex_polygon.is_convex());

        // Non-convex clockwise polygon
        let non_convex_coords = [(0, 0), (0, 2), (1, 1), (2, 2), (2, 0)];
        let non_convex_points: Vec<Point<i32, i32>> = non_convex_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let non_convex_polygon = Polygon::from_pointset(&non_convex_points);
        assert!(!non_convex_polygon.is_convex());
    }

    #[test]
    fn test_point_in_polygon_missed_branches() {
        let coords = [(0, 0), (10, 0), (10, 10), (0, 10)];
        let pointset: Vec<Point<i32, i32>> =
            coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();

        // Test case where ptq.ycoord == pt0.ycoord
        assert!(!point_in_polygon(&pointset, &Point::new(5, 10)));

        // Test case where ptq.ycoord == pt1.ycoord
        assert!(point_in_polygon(&pointset, &Point::new(5, 0)));

        // Test case where det == 0 (point on edge)
        assert!(point_in_polygon(&pointset, &Point::new(5, 0)));
    }

    #[test]
    #[should_panic(expected = "Polygon must have at least 3 points")]
    fn test_polygon_is_anticlockwise_less_than_3_points() {
        let coords = [(0, 0), (0, 1)];
        let points: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
        polygon_is_anticlockwise(&points);
    }

    #[test]
    #[should_panic(expected = "Polygon must have at least 3 points")]
    fn test_is_anticlockwise_less_than_3_points() {
        let coords = [(0, 0), (0, 1)];
        let points: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
        let polygon = Polygon::from_pointset(&points);
        polygon.is_anticlockwise();
    }

    #[test]
    fn test_is_convex_more() {
        // Non-convex anti-clockwise polygon
        let non_convex_coords = [(0, 0), (2, 0), (1, 1), (2, 2), (0, 2)];
        let non_convex_points: Vec<Point<i32, i32>> = non_convex_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let non_convex_polygon = Polygon::from_pointset(&non_convex_points);
        assert!(!non_convex_polygon.is_convex());

        // Convex anti-clockwise polygon
        let convex_coords = [(0, 0), (2, 0), (2, 2), (0, 2)];
        let convex_points: Vec<Point<i32, i32>> = convex_coords
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();
        let convex_polygon = Polygon::from_pointset(&convex_points);
        assert!(convex_polygon.is_convex());
    }

    #[test]
    fn test_point_in_polygon_more() {
        // Create a polygon that will trigger the missed branches
        let coords = [(0, 0), (10, 5), (0, 10)];
        let pointset: Vec<Point<i32, i32>> =
            coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();

        // This should trigger `det > 0`
        assert!(point_in_polygon(&pointset, &Point::new(1, 5)));

        // Create a clockwise polygon to trigger `det < 0`
        let coords_cw = [(0, 0), (0, 10), (10, 5)];
        let pointset_cw: Vec<Point<i32, i32>> =
            coords_cw.iter().map(|(x, y)| Point::new(*x, *y)).collect();
        assert!(point_in_polygon(&pointset_cw, &Point::new(1, 5)));
    }
}
