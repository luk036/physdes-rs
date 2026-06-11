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
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
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

    /// Calculates the area of the polygon
    ///
    /// # Returns
    ///
    /// The area of the polygon as a value of type T
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::polygon::Polygon;
    ///
    /// // Create a unit square
    /// let points = vec![
    ///     Point::new(0, 0),
    ///     Point::new(1, 0),
    ///     Point::new(1, 1),
    ///     Point::new(0, 1),
    /// ];
    /// let poly = Polygon::new(&points);
    /// let area = poly.area();
    /// // Note: area returns signed area (may be negative depending on vertex order)
    /// ```
    pub fn area(&self) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + Copy,
    {
        let n = self.vecs.len();
        if n < 2 {
            return T::zero();
        }

        let vec0 = self.vecs[0];
        let vec1 = self.vecs[1];
        let itr = self.vecs.iter().skip(2);

        let mut res = vec0.x_ * vec1.y_ - self.vecs[n - 1].x_ * self.vecs[n - 2].y_;

        let mut vec0 = vec0;
        let mut vec1 = vec1;

        for vec2 in itr {
            res += vec1.x_ * (vec2.y_ - vec0.y_);
            vec0 = vec1;
            vec1 = *vec2;
        }

        res
    }

    /// Gets all vertices of the polygon as points
    ///
    /// # Returns
    ///
    /// A vector of all polygon vertices in order
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::polygon::Polygon;
    ///
    /// let points = vec![
    ///     Point::new(0, 0),
    ///     Point::new(1, 0),
    ///     Point::new(1, 1),
    ///     Point::new(0, 1),
    /// ];
    /// let poly = Polygon::new(&points);
    /// let vertices = poly.vertices();
    /// assert_eq!(vertices.len(), 4);
    /// assert_eq!(vertices[0], Point::new(0, 0));
    /// ```
    pub fn vertices(&self) -> Vec<Point<T, T>> {
        let mut result = Vec::with_capacity(self.vecs.len() + 1);
        result.push(self.origin);

        for vec in &self.vecs {
            result.push(self.origin + *vec);
        }

        result
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
    pub fn get_vertices(&self) -> Vec<Point<T, T>> {
        let mut result = Vec::with_capacity(self.vecs.len() + 1);
        result.push(self.origin);

        for vec in &self.vecs {
            result.push(self.origin + *vec);
        }

        result
    }

    /// Gets the bounding box of the polygon
    ///
    /// # Returns
    ///
    /// A tuple of (min_point, max_point) representing the bounding box
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::polygon::Polygon;
    ///
    /// let points = vec![
    ///     Point::new(0, 0),
    ///     Point::new(2, 0),
    ///     Point::new(2, 2),
    ///     Point::new(0, 2),
    /// ];
    /// let poly = Polygon::new(&points);
    /// let (min_pt, max_pt) = poly.bounding_box();
    /// assert_eq!(min_pt, Point::new(0, 0));
    /// assert_eq!(max_pt, Point::new(2, 2));
    /// ```
    pub fn bounding_box(&self) -> (Point<T, T>, Point<T, T>)
    where
        T: Ord + Copy,
    {
        let vertices = self.vertices();
        let mut min_x = vertices[0].xcoord;
        let mut min_y = vertices[0].ycoord;
        let mut max_x = vertices[0].xcoord;
        let mut max_y = vertices[0].ycoord;

        for pt in &vertices {
            if pt.xcoord < min_x {
                min_x = pt.xcoord;
            }
            if pt.ycoord < min_y {
                min_y = pt.ycoord;
            }
            if pt.xcoord > max_x {
                max_x = pt.xcoord;
            }
            if pt.ycoord > max_y {
                max_y = pt.ycoord;
            }
        }

        (Point::new(min_x, min_y), Point::new(max_x, max_y))
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
    ///
    /// A polygon is convex if all its interior angles are less than 180 degrees
    /// and no edges bend inward.
    ///
    /// # Returns
    ///
    /// `true` if the polygon is convex, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::point::Point;
    /// use physdes::polygon::Polygon;
    ///
    /// // Convex square
    /// let convex_points = vec![
    ///     Point::new(0, 0),
    ///     Point::new(1, 0),
    ///     Point::new(1, 1),
    ///     Point::new(0, 1),
    /// ];
    /// let convex_poly = Polygon::new(&convex_points);
    /// assert!(convex_poly.is_convex());
    ///
    /// // Concave polygon (L-shape)
    /// let concave_points = vec![
    ///     Point::new(0, 0),
    ///     Point::new(2, 0),
    ///     Point::new(2, 1),
    ///     Point::new(1, 1),
    ///     Point::new(1, 2),
    ///     Point::new(0, 2),
    /// ];
    /// let concave_poly = Polygon::new(&concave_points);
    /// assert!(!concave_poly.is_convex());
    /// ```
    pub fn is_convex(&self) -> bool
    where
        T: PartialOrd,
    {
        let n = self.vecs.len();
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }

        // Compute initial cross product sign using vecs[N-2] and vecs[0].
        // In pointset terms: pointset[N-2] = vecs[N-2], pointset[1] = vecs[0].
        // cross = -a.x*b.y + a.y*b.x  (rearranged to avoid unary negation)
        let cross_product_sign =
            self.vecs[n - 2].y_ * self.vecs[0].x_ - self.vecs[n - 2].x_ * self.vecs[0].y_;

        for i in 0..n - 1 {
            let v0 = if i == 0 {
                Vector2::new(T::zero(), T::zero())
            } else {
                self.vecs[i - 1]
            };
            let v1 = self.vecs[i];
            let v2 = self.vecs[i + 1];

            let current_cross =
                (v1.x_ - v0.x_) * (v2.y_ - v1.y_) - (v1.y_ - v0.y_) * (v2.x_ - v1.x_);

            if (cross_product_sign > T::zero()) != (current_cross > T::zero()) {
                return false;
            }
        }

        true
    }
}

/// Creates a monotone polygon from a set of points using a custom comparison function
///
/// # Arguments
///
/// * `pointset` - A slice of points to create the polygon from
/// * `f` - A closure that defines the ordering of points
pub fn create_mono_polygon<T, F>(pointset: &[Point<T, T>], func: F) -> Vec<Point<T, T>>
where
    T: Clone + Num + Ord + Copy + PartialOrd,
    F: Fn(&Point<T, T>) -> (T, T),
{
    let max_pt = pointset
        .iter()
        .max_by(|a, b| func(a).partial_cmp(&func(b)).unwrap())
        .unwrap();
    let min_pt = pointset
        .iter()
        .min_by(|a, b| func(a).partial_cmp(&func(b)).unwrap())
        .unwrap();
    let diff = *max_pt - *min_pt;

    let (mut lst1, mut lst2): (Vec<Point<T, T>>, Vec<Point<T, T>>) = pointset
        .iter()
        .partition(|&a| diff.cross(&(*a - *min_pt)) <= T::zero());

    lst1.sort_by_key(|a| func(a));
    lst2.sort_by_key(|a| func(a));
    lst2.reverse();
    lst1.append(&mut lst2);
    lst1
}

/// Creates an x-monotone polygon from a set of points
///
/// Points are ordered primarily by x-coordinate, secondarily by y-coordinate
///
/// # Arguments
///
/// * `pointset` - A slice of points to order
///
/// # Returns
///
/// A vector of points ordered to form an x-monotone polygon
///
/// # Examples
///
/// ```
/// use physdes::point::Point;
/// use physdes::polygon::create_xmono_polygon;
///
/// let points = vec![
///     Point::new(1, 1),
///     Point::new(3, 2),
///     Point::new(2, 0),
///     Point::new(0, 2),
/// ];
/// let ordered = create_xmono_polygon(&points);
/// // Result is ordered to create x-monotone polygon
/// assert_eq!(ordered.len(), 4);
/// ```
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
///
/// # Arguments
///
/// * `pointset` - A slice of points to order
///
/// # Returns
///
/// A vector of points ordered to form a y-monotone polygon
///
/// # Examples
///
/// ```
/// use physdes::point::Point;
/// use physdes::polygon::create_ymono_polygon;
///
/// let points = vec![
///     Point::new(1, 1),
///     Point::new(2, 3),
///     Point::new(0, 2),
///     Point::new(2, 0),
/// ];
/// let ordered = create_ymono_polygon(&points);
/// // Result is ordered to create y-monotone polygon
/// assert_eq!(ordered.len(), 4);
/// ```
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
        for (x_coord, y_coord) in coords.iter() {
            pointset.push(Point::new(*x_coord, *y_coord));
        }

        let poly_points = create_xmono_polygon(&pointset);
        assert!(polygon_is_anticlockwise(&poly_points));

        let poly = Polygon::from_pointset(&poly_points);
        let mut poly2 = Polygon::from_pointset(&poly_points);
        poly2.add_assign(Vector2::new(4, 5));
        poly2.sub_assign(Vector2::new(4, 5));
        assert_eq!(poly2, poly);
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
        for (x_coord, y_coord) in coords.iter() {
            pointset.push(Point::new(*x_coord, *y_coord));
        }

        let poly_points = create_ymono_polygon(&pointset);
        assert!(polygon_is_ymonotone(&poly_points));
        assert!(!polygon_is_xmonotone(&poly_points));
        assert!(polygon_is_anticlockwise(&poly_points));

        let poly = Polygon::from_pointset(&poly_points);
        assert_eq!(poly.signed_area_x2(), 102);
        assert!(poly.is_anticlockwise());
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
        for (x_coord, y_coord) in coords.iter() {
            pointset.push(Point::new(*x_coord, *y_coord));
        }

        let poly_points = create_xmono_polygon(&pointset);
        assert!(polygon_is_xmonotone(&poly_points));
        assert!(!polygon_is_ymonotone(&poly_points));
        assert!(polygon_is_anticlockwise(&poly_points));

        let poly = Polygon::from_pointset(&poly_points);
        assert_eq!(poly.signed_area_x2(), 111);
        assert!(poly.is_anticlockwise());
    }

    #[test]
    fn test_is_rectilinear() {
        // Create a rectilinear polygon
        let rectilinear_coords = [(0, 0), (0, 1), (1, 1), (1, 0)];
        let rectilinear_points: Vec<Point<i32, i32>> = rectilinear_coords
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
            .collect();
        let rectilinear_polygon = Polygon::from_pointset(&rectilinear_points);
        assert!(rectilinear_polygon.is_rectilinear());

        // Create a non-rectilinear polygon
        let non_rectilinear_coords = [(0, 0), (1, 1), (2, 0)];
        let non_rectilinear_points: Vec<Point<i32, i32>> = non_rectilinear_coords
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
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
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
            .collect();
        let convex_polygon = Polygon::from_pointset(&convex_points);
        assert!(convex_polygon.is_convex());

        // Test case 2: Non-convex polygon
        let non_convex_coords = [(0, 0), (2, 0), (1, 1), (2, 2), (0, 2)];
        let non_convex_points: Vec<Point<i32, i32>> = non_convex_coords
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
            .collect();
        let non_convex_polygon = Polygon::from_pointset(&non_convex_points);
        assert!(!non_convex_polygon.is_convex());

        // Test case 3: Triangle (always convex)
        let triangle_coords = [(0, 0), (2, 0), (1, 2)];
        let triangle_points: Vec<Point<i32, i32>> = triangle_coords
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
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
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
            .collect();
        let clockwise_polygon = Polygon::from_pointset(&clockwise_points);
        assert!(!clockwise_polygon.is_anticlockwise());

        // Counter-clockwise polygon
        let counter_clockwise_coords = [(0, 0), (1, 0), (1, 1), (0, 1)];
        let counter_clockwise_points: Vec<Point<i32, i32>> = counter_clockwise_coords
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
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
        let pointset: Vec<Point<i32, i32>> = coords
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
            .collect();

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
        let pointset: Vec<Point<i32, i32>> = coords
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
            .collect();

        // This should trigger `det > 0`
        assert!(point_in_polygon(&pointset, &Point::new(1, 5)));

        // Create a clockwise polygon to trigger `det < 0`
        let coords_cw = [(0, 0), (0, 10), (10, 5)];
        let pointset_cw: Vec<Point<i32, i32>> = coords_cw
            .iter()
            .map(|(x_coord, y_coord)| Point::new(*x_coord, *y_coord))
            .collect();
        assert!(point_in_polygon(&pointset_cw, &Point::new(1, 5)));
    }

    #[test]
    fn test_is_rectilinear_non_rectilinear_last_edge() {
        // Last edge from last vec to origin is diagonal, not axis-aligned
        let coords = [(0, 0), (4, 0), (4, 4), (1, 3)];
        let points: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
        let poly = Polygon::from_pointset(&points);
        assert!(!poly.is_rectilinear());
    }

    #[test]
    fn test_is_convex_less_than_two_vecs() {
        // Polygon with only 1 vec (2 vertices), is_convex should return false
        let origin = Point::new(0, 0);
        let vecs = vec![Vector2::new(4, 0)];
        let poly = Polygon::from_origin_and_vectors(origin, vecs);
        assert!(!poly.is_convex());
    }

    #[test]
    fn test_bounding_box_branches() {
        // Polygon where y varies both decreasing and increasing to test min/max branches
        let coords = [(3, 5), (8, 2), (10, 7), (6, 9), (1, 4)];
        let points: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
        let poly = Polygon::from_pointset(&points);
        let (min_pt, max_pt) = poly.bounding_box();
        assert_eq!(min_pt, Point::new(1, 2));
        assert_eq!(max_pt, Point::new(10, 9));
    }
}

#[test]
fn test_polygon_signed_area_x2() {
    // Test signed_area_x2 for different polygons
    let coords = [(0, 0), (4, 0), (4, 3), (0, 3)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let poly = Polygon::from_pointset(&pointset);
    assert_eq!(poly.signed_area_x2(), 24); // 2 * (4*3) = 24
}

#[test]
fn test_polygon_vertices() {
    let coords = [(0, 0), (4, 0), (4, 3), (0, 3)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let poly = Polygon::from_pointset(&pointset);
    let vertices = poly.vertices();
    assert_eq!(vertices.len(), 4);
}

#[test]
fn test_polygon_bounding_box() {
    let coords = [(1, 1), (5, 1), (5, 4), (1, 4)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let poly = Polygon::from_pointset(&pointset);
    let (min, max) = poly.bounding_box();
    assert_eq!(min, Point::new(1, 1));
    assert_eq!(max, Point::new(5, 4));
}

#[test]
fn test_polygon_add_assign() {
    let coords = [(0, 0), (4, 0), (4, 3), (0, 3)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let mut poly = Polygon::from_pointset(&pointset);
    poly.add_assign(Vector2::new(1, 2));
    assert_eq!(poly.origin, Point::new(1, 2));
}

#[test]
fn test_polygon_sub_assign() {
    let coords = [(1, 2), (5, 2), (5, 5), (1, 5)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let mut poly = Polygon::from_pointset(&pointset);
    poly.sub_assign(Vector2::new(1, 2));
    assert_eq!(poly.origin, Point::new(0, 0));
}

#[test]
fn test_polygon_partial_eq() {
    let coords = [(0, 0), (4, 0), (4, 3), (0, 3)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let poly1 = Polygon::from_pointset(&pointset);
    let poly2 = Polygon::from_pointset(&pointset);
    assert_eq!(poly1, poly2);

    let coords2 = [(0, 0), (5, 0), (5, 3), (0, 3)];
    let pointset2: Vec<Point<i32, i32>> = coords2.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let poly3 = Polygon::from_pointset(&pointset2);
    assert_ne!(poly1, poly3);
}

#[test]
fn test_polygon_from_origin_and_vectors() {
    let origin = Point::new(0, 0);
    let vecs = vec![Vector2::new(4, 0), Vector2::new(0, 3), Vector2::new(-4, 0)];
    let poly = Polygon::from_origin_and_vectors(origin, vecs);
    assert_eq!(poly.origin, Point::new(0, 0));
    assert_eq!(poly.vecs.len(), 3);
}

#[test]
fn test_polygon_default() {
    let poly: Polygon<i32> = Polygon::default();
    assert_eq!(poly.origin, Point::new(0, 0));
}

#[test]
fn test_polygon_is_monotone_custom() {
    // Test polygon_is_monotone with custom direction
    let coords = [(0, 0), (1, 0), (2, 1), (1, 2), (0, 2)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    // x-monotone
    assert!(polygon_is_monotone(&pointset, |pt| (pt.xcoord, pt.ycoord)));
}

#[test]
fn test_create_mono_polygon_custom() {
    // Test create_mono_polygon with custom function
    let coords = [(0, 0), (2, 1), (1, 2), (3, 2)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let result = create_mono_polygon(&pointset, |pt| (pt.xcoord, pt.ycoord));
    assert!(!result.is_empty());
}

#[test]
fn test_polygon_empty_vecs_is_rectilinear() {
    // Edge case: polygon with no vecs should be rectilinear
    let origin = Point::new(0, 0);
    let poly = Polygon::from_origin_and_vectors(origin, vec![]);
    assert!(poly.is_rectilinear());
}

#[test]
fn test_polygon_signed_area_x2_triangle() {
    // Triangle area calculation
    let coords = [(0, 0), (3, 0), (0, 4)];
    let pointset: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let poly = Polygon::from_pointset(&pointset);
    // Area = 0.5 * |0*(0-4) + 3*(4-0) + 0*(0-0)| = 0.5 * 12 = 6
    // signed_area_x2 = 2 * area = 12
    assert_eq!(poly.signed_area_x2(), 12);
}

#[test]
fn test_polygon_signed_area_x2_single_vec() {
    // Polygon with just 1 vec (2 vertices), signed_area_x2 should return 0 (n < 2)
    let origin = Point::new(0, 0);
    let vecs = vec![Vector2::new(4, 0)];
    let poly = Polygon::from_origin_and_vectors(origin, vecs);
    assert_eq!(poly.signed_area_x2(), 0);
}

#[test]
fn test_polygon_area_multi_vertex() {
    // Pentagon to exercise the area() calculation loop body
    let coords = [(0, 0), (4, 0), (5, 3), (2, 5), (-1, 2)];
    let points: Vec<Point<i32, i32>> = coords.iter().map(|(x, y)| Point::new(*x, *y)).collect();
    let poly = Polygon::from_pointset(&points);
    // area is signed, should be nonzero for a valid polygon
    assert_ne!(poly.area(), 0);
}
