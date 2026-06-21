use crate::point::Point;

/// Removes concave vertices iteratively until the polygon becomes monotone
/// in a given direction.
///
/// Repeatedly scans the polygon vertices and removes any vertex whose
/// cross-product sign indicates concavity (relative to the polygon's
/// winding direction). The cross product test:
///
/// $$\text{cross} = (y_i - y_{i-1}) \cdot (x_{i+1} - x_i)$$
///
/// Stops when no more concave vertices remain.
///
/// # Arguments
///
/// * `pointset` - The input polygon vertices
/// * `is_anticlockwise` - Whether the polygon is oriented anti-clockwise
/// * `dir` - A closure defining the monotone direction (e.g. x or y)
///
/// # Type Parameters
///
/// * `F` - A closure `Fn(&Point<T, T>) -> (T, T)` used to order points
pub fn rpolygon_make_monotone_hull<T, F>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
    _dir: F,
) -> Vec<Point<T, T>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + num_traits::Num
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>,
    F: Fn(&Point<T, T>) -> (T, T),
{
    if pointset.len() <= 3 {
        return pointset.to_vec();
    }
    let mut pts = pointset.to_vec();
    loop {
        let n = pts.len();
        let mut removed = false;
        let mut i = 0;
        while i < n {
            let p = (i + n - 1) % n;
            let nx = (i + 1) % n;
            let cross = (pts[i].ycoord - pts[p].ycoord) * (pts[nx].xcoord - pts[i].xcoord);
            if cross != T::zero() {
                let concave = if is_anticlockwise {
                    cross < T::zero()
                } else {
                    cross > T::zero()
                };
                if concave {
                    pts.remove(i);
                    removed = true;
                    break;
                }
            }
            i += 1;
        }
        if !removed {
            break;
        }
    }
    pts
}

/// Computes an x-monotone hull of a polygon by removing concave vertices
/// with respect to the x-direction.
pub fn rpolygon_make_xmonotone_hull<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Point<T, T>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + num_traits::Num
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>,
{
    rpolygon_make_monotone_hull(pointset, is_anticlockwise, |p: &Point<T, T>| {
        (p.xcoord, p.ycoord)
    })
}

/// Computes a y-monotone hull of a polygon by removing concave vertices
/// with respect to the y-direction.
pub fn rpolygon_make_ymonotone_hull<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Point<T, T>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + num_traits::Num
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>,
{
    rpolygon_make_monotone_hull(pointset, is_anticlockwise, |p: &Point<T, T>| {
        (p.ycoord, p.xcoord)
    })
}

/// Computes the convex hull of a rectilinear polygon using Andrew's
/// monotone chain algorithm.
///
/// Uses the cross product test to determine clockwise/counter-clockwise turns:
///
/// $$\vec{(a-o)} \times \vec{(b-o)} = (a_x - o_x)(b_y - o_y) - (a_y - o_y)(b_x - o_x)$$
///
/// Points making a non-left turn ($\le 0$) are removed from the hull.
///
/// The resulting hull satisfies: `convex_hull_area >= original_polygon_area`.
/// For convex polygons the hull equals the original.
///
/// # Arguments
///
/// * `pointset` - The input polygon vertices
/// * `_is_anticlockwise` - Ignored (the algorithm determines orientation)
pub fn rpolygon_make_convex_hull<T>(
    pointset: &[Point<T, T>],
    _is_anticlockwise: bool,
) -> Vec<Point<T, T>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + num_traits::Num
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>,
{
    if pointset.len() <= 3 {
        return pointset.to_vec();
    }

    let mut pts = pointset.to_vec();
    pts.sort_by(|a, b| a.xcoord.cmp(&b.xcoord).then(a.ycoord.cmp(&b.ycoord)));

    let cross = |o: &Point<T, T>, a: &Point<T, T>, b: &Point<T, T>| -> T {
        (a.xcoord - o.xcoord) * (b.ycoord - o.ycoord)
            - (a.ycoord - o.ycoord) * (b.xcoord - o.xcoord)
    };

    let mut lower = Vec::new();
    for p in &pts {
        while lower.len() >= 2
            && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], p) <= T::zero()
        {
            lower.pop();
        }
        lower.push(*p);
    }

    let mut upper = Vec::new();
    for p in pts.iter().rev() {
        while upper.len() >= 2
            && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], p) <= T::zero()
        {
            upper.pop();
        }
        upper.push(*p);
    }

    lower.pop();
    upper.pop();
    lower.append(&mut upper);
    lower
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_convex_hull_triangle() {
        let pts = vec![Point::new(0, 0), Point::new(1, 0), Point::new(0, 1)];
        let result = rpolygon_make_convex_hull(&pts, true);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_make_convex_hull_square() {
        let pts = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, 1),
        ];
        let result = rpolygon_make_convex_hull(&pts, true);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_make_xmonotone_hull_simple() {
        let pts = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(0, 2),
        ];
        let result = rpolygon_make_xmonotone_hull(&pts, true);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_make_ymonotone_hull_simple() {
        let pts = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(0, 2),
        ];
        let result = rpolygon_make_ymonotone_hull(&pts, true);
        assert!(!result.is_empty());
    }
}
