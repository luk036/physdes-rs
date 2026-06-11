//! Additional geometric algorithms
//!
//! This module provides additional geometric algorithms beyond the basic operations.

use crate::Polygon;

/// Checks if a point is inside a polygon using the ray casting algorithm
///
/// # Arguments
///
/// * `point` - The point to check
/// * `polygon` - The polygon to test against
///
/// # Returns
///
/// `true` if the point is inside the polygon, `false` otherwise
///
/// # Examples
///
/// ```
/// use physdes::{Point, algorithms::point_in_polygon};
/// use physdes::Polygon;
///
/// let points = vec![
///     Point::new(0, 0),
///     Point::new(10, 0),
///     Point::new(10, 10),
///     Point::new(0, 10),
/// ];
/// let polygon = Polygon::new(&points);
///
/// assert!(point_in_polygon(&Point::new(5, 5), &polygon));
/// assert!(!point_in_polygon(&Point::new(15, 15), &polygon));
/// ```
pub fn point_in_polygon<T>(point: &crate::Point<T, T>, polygon: &Polygon<T>) -> bool
where
    T: Clone
        + Copy
        + PartialOrd
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Add<Output = T>
        + num_traits::Zero
        + num_traits::Num
        + std::ops::AddAssign
        + Ord,
{
    let vertices = polygon.get_vertices();
    let n = vertices.len();

    if n < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = n - 1;

    for i in 0..n {
        let xi = vertices[i].xcoord;
        let yi = vertices[i].ycoord;
        let xj = vertices[j].xcoord;
        let yj = vertices[j].ycoord;

        let intersect = ((yi > point.ycoord) != (yj > point.ycoord))
            && (point.xcoord < (xj - xi) * (point.ycoord - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }

        j = i;
    }

    inside
}

/// Computes the centroid (geometric center) of a polygon
///
/// # Arguments
///
/// * `polygon` - The polygon
///
/// # Returns
///
/// The centroid point of the polygon
///
/// # Examples
///
/// ```
/// use physdes::{Point, algorithms::polygon_centroid};
/// use physdes::Polygon;
///
/// let points = vec![
///     Point::new(0, 0),
///     Point::new(4, 0),
///     Point::new(4, 3),
/// ];
/// let polygon = Polygon::new(&points);
///
/// let centroid = polygon_centroid(&polygon);
/// // For a triangle, centroid is at the average of vertices
/// ```
pub fn polygon_centroid<T>(polygon: &Polygon<T>) -> crate::Point<T, T>
where
    T: Clone
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Div<Output = T>
        + From<i32>
        + num_traits::Zero
        + num_traits::Num
        + std::ops::AddAssign
        + Ord,
{
    let vertices = polygon.get_vertices();
    let n = vertices.len();

    if n == 0 {
        return crate::Point::new(T::from(0), T::from(0));
    }

    let mut sum_x = T::from(0);
    let mut sum_y = T::from(0);

    for vertex in &vertices {
        sum_x += vertex.xcoord;
        sum_y += vertex.ycoord;
    }

    let n_t = T::from(n as i32);
    crate::Point::new(sum_x / n_t, sum_y / n_t)
}

/// Computes the perimeter of a polygon
///
/// # Arguments
///
/// * `polygon` - The polygon
///
/// # Returns
///
/// The perimeter length
///
/// # Examples
///
/// ```
/// use physdes::{Point, algorithms::polygon_perimeter};
/// use physdes::Polygon;
///
/// let points = vec![
///     Point::new(0, 0),
///     Point::new(3, 0),
///     Point::new(3, 4),
///     Point::new(0, 4),
/// ];
/// let polygon = Polygon::new(&points);
///
/// let perimeter = polygon_perimeter(&polygon);
/// assert_eq!(perimeter, 14); // 3 + 4 + 3 + 4
/// ```
pub fn polygon_perimeter<T>(polygon: &Polygon<T>) -> T
where
    T: Clone
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num
        + std::ops::AddAssign
        + Ord,
{
    let vertices = polygon.get_vertices();
    let n = vertices.len();

    if n < 2 {
        return T::zero();
    }

    let mut perimeter = T::zero();

    for i in 0..n {
        let current = vertices[i];
        let next = vertices[(i + 1) % n];

        let dx = if current.xcoord > next.xcoord {
            current.xcoord - next.xcoord
        } else {
            next.xcoord - current.xcoord
        };

        let dy = if current.ycoord > next.ycoord {
            current.ycoord - next.ycoord
        } else {
            next.ycoord - current.ycoord
        };

        perimeter = perimeter + dx + dy;
    }

    perimeter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_in_polygon() {
        let points = vec![
            crate::Point::new(0, 0),
            crate::Point::new(10, 0),
            crate::Point::new(10, 10),
            crate::Point::new(0, 10),
        ];
        let polygon = Polygon::new(&points);

        assert!(point_in_polygon(&crate::Point::new(5, 5), &polygon));
        assert!(!point_in_polygon(&crate::Point::new(15, 15), &polygon));
        assert!(!point_in_polygon(&crate::Point::new(-5, 5), &polygon));
    }

    #[test]
    fn test_polygon_centroid() {
        let points = vec![
            crate::Point::new(0, 0),
            crate::Point::new(3, 0),
            crate::Point::new(3, 3),
        ];
        let polygon = Polygon::new(&points);

        let centroid = polygon_centroid(&polygon);
        assert_eq!(centroid.xcoord, 2);
        assert_eq!(centroid.ycoord, 1);
    }

    #[test]
    fn test_polygon_perimeter() {
        let points = vec![
            crate::Point::new(0, 0),
            crate::Point::new(3, 0),
            crate::Point::new(3, 4),
            crate::Point::new(0, 4),
        ];
        let polygon = Polygon::new(&points);

        let perimeter = polygon_perimeter(&polygon);
        assert_eq!(perimeter, 14);
    }

    #[test]
    fn test_point_in_polygon_less_than_3_points() {
        let pts = vec![crate::Point::new(0, 0), crate::Point::new(1, 0)];
        let polygon = Polygon::new(&pts);
        assert!(!point_in_polygon(&crate::Point::new(0, 0), &polygon));
    }

    #[test]
    fn test_polygon_centroid_empty() {
        let polygon = Polygon::from_origin_and_vectors(crate::Point::new(0, 0), vec![]);
        let result = polygon_centroid(&polygon);
        assert_eq!(result, crate::Point::new(0, 0));
    }

    #[test]
    fn test_polygon_perimeter_less_than_2_points() {
        let polygon = Polygon::from_origin_and_vectors(crate::Point::new(0, 0), vec![]);
        assert_eq!(polygon_perimeter(&polygon), 0);

        let polygon2 = Polygon::new(&[crate::Point::new(0, 0)]);
        assert_eq!(polygon_perimeter(&polygon2), 0);
    }
}
