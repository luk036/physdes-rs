//! VLSI-specific geometric operations
//!
//! This module provides operations commonly used in VLSI physical design and layout.

use crate::{Point, Polygon};

/// Represents a rectangle in 2D space defined by its minimum (bottom-left)
/// and maximum (top-right) corner points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle<T> {
    /// The minimum (bottom-left) corner point
    pub min: Point<T, T>,
    /// The maximum (top-right) corner point
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

/// Detects if any pair of rectangles overlap using the line sweep algorithm.
///
/// The algorithm uses a sweep line approach:
/// 1. Create events for left and right edges of each rectangle
/// 2. Sort events by x-coordinate
/// 3. Sweep from left to right, maintaining active rectangles
/// 4. Check y-overlap when a new rectangle becomes active
///
/// # Arguments
///
/// * `rectangles` - A slice of rectangles to check for overlaps
///
/// # Returns
///
/// The indices of two overlapping rectangles if found, otherwise `None`
///
/// # Examples
///
/// ```
/// use physdes::{Point, vlsi_ops::{Rectangle, detect_overlap}};
///
/// let rects = vec![
///     Rectangle::new(Point::new(0, 0), Point::new(10, 10)),
///     Rectangle::new(Point::new(5, 5), Point::new(15, 15)),
///     Rectangle::new(Point::new(20, 20), Point::new(30, 30)),
/// ];
/// let result = detect_overlap(&rects);
/// assert!(result.is_some());
/// assert_eq!(result.unwrap(), (0, 1));
/// ```
pub fn detect_overlap<T>(rectangles: &[Rectangle<T>]) -> Option<(usize, usize)>
where
    T: Copy + Ord,
{
    if rectangles.len() < 2 {
        return None;
    }

    // Create events: (x_coord, is_start, rect_index)
    let mut events: Vec<(T, bool, usize)> = Vec::with_capacity(rectangles.len() * 2);
    for (idx, rect) in rectangles.iter().enumerate() {
        events.push((rect.min.xcoord, true, idx));
        events.push((rect.max.xcoord, false, idx));
    }

    events.sort_by_key(|a| a.0);

    // Active rectangles: (rect_index, y_min, y_max)
    let mut active: Vec<(usize, T, T)> = Vec::new();

    for &(_x, is_start, idx) in &events {
        let rect = &rectangles[idx];

        if is_start {
            // Check y-overlap with all active rectangles
            for &(other_idx, other_y_min, other_y_max) in &active {
                if rect.min.ycoord <= other_y_max && other_y_min <= rect.max.ycoord {
                    return Some((other_idx, idx));
                }
            }
            active.push((idx, rect.min.ycoord, rect.max.ycoord));
        } else {
            // O(1) removal: swap with back and pop
            for i in 0..active.len() {
                if active[i].0 == idx {
                    active.swap_remove(i);
                    break;
                }
            }
        }
    }

    None
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

    #[test]
    fn test_from_dimensions() {
        let origin = Point::new(5, 10);
        let rect = Rectangle::from_dimensions(origin, 20, 30);
        assert_eq!(rect.min, origin);
        assert_eq!(rect.max, Point::new(25, 40));
    }

    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let inner = Rectangle::new(Point::new(2, 2), Point::new(8, 8));
        let outer = Rectangle::new(Point::new(-5, -5), Point::new(15, 15));

        assert!(rect.contains(&inner));
        assert!(!rect.contains(&outer));
    }

    #[test]
    fn test_rectangle_contains_point() {
        let rect = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let inside = Point::new(5, 5);
        let outside = Point::new(15, 15);
        let on_edge = Point::new(10, 5);

        assert!(rect.contains_point(&inside));
        assert!(!rect.contains_point(&outside));
        assert!(rect.contains_point(&on_edge));
    }

    #[test]
    fn test_bounding_rect_method() {
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(5, 5), Point::new(15, 15));
        let bbox = rect1.bounding_rect(&rect2);

        assert_eq!(bbox.min, Point::new(0, 0));
        assert_eq!(bbox.max, Point::new(15, 15));
    }

    #[test]
    fn test_to_polygon() {
        let rect = Rectangle::new(Point::new(0, 0), Point::new(10, 20));
        let polygon = rect.to_polygon();

        assert_eq!(polygon.vertices().len(), 4);
        assert!(polygon.is_rectilinear());
    }

    #[test]
    fn test_bounding_rect_empty() {
        let rects: Vec<Rectangle<i32>> = vec![];
        assert!(bounding_rect(&rects).is_none());
    }

    #[test]
    fn test_total_area_empty() {
        let rects: Vec<Rectangle<i32>> = vec![];
        assert_eq!(total_area(&rects), 0);
    }

    #[test]
    fn test_manhattan_distance_reverse() {
        // Test case where p2 > p1 (different branch)
        let p1 = Point::new(5, 3);
        let p2 = Point::new(1, 0);
        assert_eq!(manhattan_distance(&p1, &p2), 7); // |5-1| + |3-0| = 7
    }

    #[test]
    fn test_manhattan_distance_same() {
        // Test case where coordinates are equal
        let p1 = Point::new(5, 5);
        let p2 = Point::new(5, 5);
        assert_eq!(manhattan_distance(&p1, &p2), 0);
    }

    #[test]
    fn test_check_spacing_vertical() {
        // Test vertical spacing
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(0, 10), Point::new(10, 20));
        assert!(!check_spacing(&rect1, &rect2, 1)); // touching

        let rect3 = Rectangle::new(Point::new(0, 12), Point::new(10, 22));
        assert!(check_spacing(&rect1, &rect3, 1)); // spacing of 2
    }

    #[test]
    fn test_check_spacing_overlapping() {
        // Test case where rectangles overlap
        let rect1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::new(Point::new(5, 5), Point::new(15, 15));
        assert!(!check_spacing(&rect1, &rect2, 1)); // overlapping
    }

    #[test]
    fn test_rectangle_intersect_non_overlapping() {
        let r1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let r2 = Rectangle::new(Point::new(20, 20), Point::new(30, 30));
        assert!(r1.intersect(&r2).is_none());
    }

    #[test]
    fn test_detect_overlap_empty() {
        let rects: Vec<Rectangle<i32>> = vec![];
        assert!(detect_overlap(&rects).is_none());
    }

    #[test]
    fn test_detect_overlap_single() {
        let rects = vec![Rectangle::new(Point::new(0, 0), Point::new(10, 10))];
        assert!(detect_overlap(&rects).is_none());
    }

    #[test]
    fn test_detect_overlap_non_overlapping() {
        let rects = vec![
            Rectangle::new(Point::new(0, 0), Point::new(10, 10)),
            Rectangle::new(Point::new(20, 20), Point::new(30, 30)),
        ];
        assert!(detect_overlap(&rects).is_none());
    }

    #[test]
    fn test_detect_overlap_three_rects() {
        // First and third overlap, second is in between
        let rects = vec![
            Rectangle::new(Point::new(0, 0), Point::new(10, 10)),
            Rectangle::new(Point::new(15, 15), Point::new(25, 25)),
            Rectangle::new(Point::new(5, 5), Point::new(12, 12)), // overlaps with first
        ];
        let result = detect_overlap(&rects);
        assert!(result.is_some());
        // Should find overlap between 0 and 2 (or 2 and 0)
        let (i, j) = result.unwrap();
        assert!((i == 0 && j == 2) || (i == 2 && j == 0));
    }

    #[test]
    fn test_check_spacing_x_overlap_y_separate() {
        // Rectangles that overlap in x but are separated in y
        let r1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let r2 = Rectangle::new(Point::new(5, 20), Point::new(15, 30));
        // x: [0,10] and [5,15] overlap; y: [0,10] and [20,30] separated by 10
        assert!(check_spacing(&r1, &r2, 10));
        assert!(!check_spacing(&r1, &r2, 11));
    }

    #[test]
    fn test_check_spacing_vertical_overlap_horizontal_separate() {
        // Rectangles that overlap in y but are separated in x
        let r1 = Rectangle::new(Point::new(0, 0), Point::new(10, 10));
        let r2 = Rectangle::new(Point::new(20, 5), Point::new(30, 15));
        assert!(check_spacing(&r1, &r2, 10));
        assert!(!check_spacing(&r1, &r2, 11));
    }
}
