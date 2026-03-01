//! VLSI-specific geometric operations
//!
//! This module provides operations commonly used in VLSI physical design and layout.

use crate::{Point, Polygon};

/// Represents a rectangle in 2D space
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle<T> {
    pub min: Point<T, T>,
    pub max: Point<T, T>,
}

impl<T: Clone + Ord + Copy + std::ops::Add<Output = T>> Rectangle<T> {
    /// Creates a new rectangle from min and max points
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum (bottom-left) corner
    /// * `max` - The maximum (top-right) corner
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::{Point, vlsi_ops::Rectangle};
    ///
    /// let rect = Rectangle::new(
    ///     Point::new(0, 0),
    ///     Point::new(10, 20)
    /// );
    /// ```
    pub fn new(min: Point<T, T>, max: Point<T, T>) -> Self {
        assert!(min.xcoord <= max.xcoord && min.ycoord <= max.ycoord);
        Rectangle { min, max }
    }

    /// Creates a rectangle from origin, width, and height
    pub fn from_dimensions(origin: Point<T, T>, width: T, height: T) -> Self {
        Rectangle::new(
            origin,
            Point::new(origin.xcoord + width, origin.ycoord + height),
        )
    }

    /// Checks if this rectangle overlaps with another
    ///
    /// # Examples
    ///
    /// ```
    /// use physdes::{Point, vlsi_ops::Rectangle};
    ///
    /// let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
    /// let rect2 = Rectangle::new(Point::new(5, 5), Point::new(15, 15));
    /// assert!(rect1.overlaps(&rect2));
    ///
    /// let rect3 = Rectangle::new(Point::new(20, 20), Point::new(30, 30));
    /// assert!(!rect1.overlaps(&rect3));
    /// ```
    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.xcoord <= other.max.xcoord
            && self.max.xcoord >= other.min.xcoord
            && self.min.ycoord <= other.max.ycoord
            && self.max.ycoord >= other.min.ycoord
    }

    /// Checks if this rectangle contains another rectangle
    pub fn contains(&self, other: &Self) -> bool {
        self.min.xcoord <= other.min.xcoord
            && self.max.xcoord >= other.max.xcoord
            && self.min.ycoord <= other.min.ycoord
            && self.max.ycoord >= other.max.ycoord
    }

    /// Checks if this rectangle contains a point
    pub fn contains_point(&self, point: &Point<T, T>) -> bool {
        point.xcoord >= self.min.xcoord
            && point.xcoord <= self.max.xcoord
            && point.ycoord >= self.min.ycoord
            && point.ycoord <= self.max.ycoord
    }

    /// Computes the intersection with another rectangle
    ///
    /// Returns `None` if the rectangles don't overlap
    pub fn intersect(&self, other: &Self) -> Option<Self>
    where
        T: std::ops::Add<Output = T>,
    {
        if !self.overlaps(other) {
            return None;
        }

        Some(Rectangle::new(
            Point::new(
                self.min.xcoord.max(other.min.xcoord),
                self.min.ycoord.max(other.min.ycoord),
            ),
            Point::new(
                self.max.xcoord.min(other.max.xcoord),
                self.max.ycoord.min(other.max.ycoord),
            ),
        ))
    }

    /// Computes the area of the rectangle
    pub fn area(&self) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
    {
        let width = self.max.xcoord - self.min.xcoord;
        let height = self.max.ycoord - self.min.ycoord;
        width * height
    }

    /// Computes the bounding rectangle of two rectangles
    pub fn bounding_rect(&self, other: &Self) -> Self {
        Rectangle::new(
            Point::new(
                self.min.xcoord.min(other.min.xcoord),
                self.min.ycoord.min(other.min.ycoord),
            ),
            Point::new(
                self.max.xcoord.max(other.max.xcoord),
                self.max.ycoord.max(other.max.ycoord),
            ),
        )
    }

    /// Converts to a Polygon
    pub fn to_polygon(&self) -> Polygon<T>
    where
        T: Clone + std::ops::Add<Output = T> + num_traits::Num + std::ops::AddAssign + Ord,
    {
        Polygon::new(&[
            self.min,
            Point::new(self.max.xcoord, self.min.ycoord),
            self.max,
            Point::new(self.min.xcoord, self.max.ycoord),
        ])
    }
}

/// Calculates Manhattan distance between two points
///
/// The Manhattan distance is the sum of the absolute differences of coordinates.
/// This is commonly used in VLSI routing where wires can only run horizontally or vertically.
///
/// # Arguments
///
/// * `p1` - First point
/// * `p2` - Second point
///
/// # Examples
///
/// ```
/// use physdes::{Point, vlsi_ops::manhattan_distance};
///
/// let p1 = Point::new(0, 0);
/// let p2 = Point::new(3, 4);
/// let dist = manhattan_distance(&p1, &p2);
/// assert_eq!(dist, 7); // |3-0| + |4-0| = 7
/// ```
pub fn manhattan_distance<T>(p1: &Point<T, T>, p2: &Point<T, T>) -> T
where
    T: Ord + Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    let dx = if p1.xcoord > p2.xcoord {
        p1.xcoord - p2.xcoord
    } else {
        p2.xcoord - p1.xcoord
    };
    let dy = if p1.ycoord > p2.ycoord {
        p1.ycoord - p2.ycoord
    } else {
        p2.ycoord - p1.ycoord
    };
    dx + dy
}

/// Checks if two rectangles satisfy minimum spacing requirements
///
/// # Arguments
///
/// * `rect1` - First rectangle
/// * `rect2` - Second rectangle
/// * `min_spacing` - Minimum required spacing
///
/// # Examples
///
/// ```
/// use physdes::{Point, vlsi_ops::{Rectangle, check_spacing}};
///
/// let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
/// let rect2 = Rectangle::new(Point::new(10, 0), Point::new(20, 10));
///
/// // These rectangles touch (spacing = 0)
/// assert!(!check_spacing(&rect1, &rect2, 1));
///
/// let rect3 = Rectangle::new(Point::new(12, 0), Point::new(22, 10));
/// // These have spacing of 2
/// assert!(check_spacing(&rect1, &rect3, 1));
/// ```
pub fn check_spacing<T>(rect1: &Rectangle<T>, rect2: &Rectangle<T>, min_spacing: T) -> bool
where
    T: Ord + Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    if rect1.overlaps(rect2) {
        return false;
    }

    // Calculate horizontal spacing
    let horiz_spacing = if rect1.max.xcoord <= rect2.min.xcoord {
        rect2.min.xcoord - rect1.max.xcoord
    } else if rect2.max.xcoord <= rect1.min.xcoord {
        rect1.min.xcoord - rect2.max.xcoord
    } else {
        T::clone(&min_spacing) // Overlapping in x, check y
    };

    // Calculate vertical spacing
    let vert_spacing = if rect1.max.ycoord <= rect2.min.ycoord {
        rect2.min.ycoord - rect1.max.ycoord
    } else if rect2.max.ycoord <= rect1.min.ycoord {
        rect1.min.ycoord - rect2.max.ycoord
    } else {
        T::clone(&min_spacing) // Overlapping in y, check x
    };

    horiz_spacing >= min_spacing && vert_spacing >= min_spacing
}

/// Computes the minimum bounding rectangle of a set of rectangles
///
/// # Arguments
///
/// * `rects` - Slice of rectangles
///
/// # Examples
///
/// ```
/// use physdes::{Point, vlsi_ops::{Rectangle, bounding_rect}};
///
/// let rects = vec![
///     Rectangle::new(Point::new(0, 0), Point::new(10, 10)),
///     Rectangle::new(Point::new(5, 5), Point::new(15, 15)),
/// ];
///
/// let bbox = bounding_rect(&rects).unwrap();
/// assert_eq!(bbox.min, Point::new(0, 0));
/// assert_eq!(bbox.max, Point::new(15, 15));
/// ```
pub fn bounding_rect<T>(rects: &[Rectangle<T>]) -> Option<Rectangle<T>>
where
    T: Ord + Copy + std::ops::Add<Output = T>,
{
    if rects.is_empty() {
        return None;
    }

    let mut min_x = rects[0].min.xcoord;
    let mut min_y = rects[0].min.ycoord;
    let mut max_x = rects[0].max.xcoord;
    let mut max_y = rects[0].max.ycoord;

    for rect in &rects[1..] {
        min_x = min_x.min(rect.min.xcoord);
        min_y = min_y.min(rect.min.ycoord);
        max_x = max_x.max(rect.max.xcoord);
        max_y = max_y.max(rect.max.ycoord);
    }

    Some(Rectangle::new(
        Point::new(min_x, min_y),
        Point::new(max_x, max_y),
    ))
}

/// Computes total area covered by rectangles, accounting for overlaps
///
/// # Examples
///
/// ```
/// use physdes::{Point, vlsi_ops::{Rectangle, total_area}};
///
/// let rects = vec![
///     Rectangle::new(Point::new(0, 0), Point::new(10, 10)),
///     Rectangle::new(Point::new(5, 5), Point::new(15, 15)),
/// ];
///
/// let area = total_area(&rects);
/// // First rect: 100, second rect: 100, overlap: 25, total: 175
/// assert_eq!(area, 175);
/// ```
pub fn total_area(rects: &[Rectangle<i32>]) -> i32 {
    if rects.is_empty() {
        return 0;
    }

    let mut total = 0;
    for (i, rect) in rects.iter().enumerate() {
        total += rect.area();
        // Subtract overlaps with previously counted rectangles
        for other in &rects[..i] {
            if let Some(intersection) = rect.intersect(other) {
                total -= intersection.area();
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_creation() {
        let rect = Rectangle::new(Point::new(0, 0), Point::new(10, 20));
        assert_eq!(rect.min, Point::new(0, 0));
        assert_eq!(rect.max, Point::new(10, 20));
    }

    #[test]
    fn test_rectangle_overlap() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(5, 5), Point::new(15, 15));
        assert!(rect1.overlaps(&rect2));

        let rect3 = Rectangle::new(Point::new(20, 20), Point::new(30, 30));
        assert!(!rect1.overlaps(&rect3));
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(Point::new(0, 0), Point::new(10, 20));
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_eq!(manhattan_distance(&p1, &p2), 7);

        let p3 = Point::new(5, 0);
        assert_eq!(manhattan_distance(&p1, &p3), 5);
    }

    #[test]
    fn test_check_spacing() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(10, 0), Point::new(20, 10));
        assert!(!check_spacing(&rect1, &rect2, 1));

        let rect3 = Rectangle::new(Point::new(12, 0), Point::new(22, 10));
        assert!(check_spacing(&rect1, &rect3, 1));
    }

    #[test]
    fn test_bounding_rect() {
        let rects = vec![
            Rectangle::new(Point::new(0, 0), Point::new(10, 10)),
            Rectangle::new(Point::new(5, 5), Point::new(15, 15)),
        ];

        let bbox = bounding_rect(&rects).unwrap();
        assert_eq!(bbox.min, Point::new(0, 0));
        assert_eq!(bbox.max, Point::new(15, 15));
    }

    #[test]
    fn test_total_area() {
        let rects = vec![
            Rectangle::new(Point::new(0, 0), Point::new(10, 10)),
            Rectangle::new(Point::new(5, 5), Point::new(15, 15)),
        ];

        let area = total_area(&rects);
        assert_eq!(area, 175);
    }
}
