use super::{Point, Vector2};
use num_traits::Num;
// use core::ops::{Add, Neg, Sub};

/// The `RPolygon` struct represents a rectilinear polygon with an origin point and a vector of 2D
/// vectors.
///
/// Properties:
///
/// * `origin`: The origin property represents the starting point or the reference point of the
/// rectilinear polygon. It is of type Point<T>, where T is the type of the coordinates of the point
/// (e.g., integer or floating-point).
/// * `vecs`: vecs is a vector that stores the vectors representing the sides of the rectilinear
/// polygon.
pub struct RPolygon<T> {
    pub origin: Point<T>,
    vecs: Vec<Vector2<T>>,
}

impl<T: Clone + Num + Copy> RPolygon<T> {
    /// The `new` function constructs a new `RPolygon` object by calculating the origin and vectors
    /// based on the given coordinates.
    ///
    /// Arguments:
    ///
    /// * `coords`: The `coords` parameter is an array of `Point<T>` objects. It represents the
    /// coordinates of the points that define the polygon. The first element of the array (`coords[0]`)
    /// is considered as the origin of the polygon, and the remaining elements represent the vectors
    /// from the origin to the
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `RPolygon` struct.
    pub fn new(coords: &[Point<T>]) -> Self {
        let origin = coords[0];
        let mut vecs = vec![];
        for pt in coords.iter().skip(1) {
            vecs.push(pt - origin);
        }
        RPolygon { origin, vecs }
    }

    /// The `signed_area` function calculates the signed area of a polygon.
    ///
    /// Returns:
    ///
    /// The function `signed_area` returns a value of type `T`.
    pub fn signed_area(&self) -> T {
        // assert!(self.vecs.len() >= 1);
        let vs = &self.vecs;
        let n = vs.len();
        let mut res = vs[0].x_ * vs[0].y_;
        for i in 1..n {
            res = res + vs[i].x_ * (vs[i].y_ - vs[i - 1].y_);
        }
        res
    }

    /**
     * @brief
     *
     * @return Point<T>
     */
    pub fn lb(&self) -> Point<T> {
        unimplemented!()
    }

    /**
     * @brief
     *
     * @return Point<T>
     */
    pub fn ub(&self) -> Point<T> {
        unimplemented!()
    }
}

impl<T: Clone + Num + Ord + Copy> RPolygon<T> {
    /// The `create_mono_rpolygon` function creates a monotone polygon from a given set of points based
    /// on a provided comparison function.
    ///
    /// Arguments:
    ///
    /// * `pointset`: `pointset` is a slice of `Point<T>` elements. It represents a set of points in a
    /// two-dimensional space.
    /// * `f`: The parameter `f` is a closure that takes a reference to a reference of a `Point<T>` and
    /// returns a tuple of two values of type `T`. The closure is used to determine the ordering of the
    /// points in the `pointset`. The first value of the tuple represents the x-coordinate
    pub fn create_mono_rpolygon<F>(pointset: &[Point<T>], f: F) -> (Vec<Point<T>>, bool)
    where
        F: Fn(&&Point<T>) -> (T, T),
    {
        // Use x-mono as model
        let rightmost = pointset.iter().max_by_key(&f).unwrap();
        let leftmost = pointset.iter().min_by_key(&f).unwrap();
        let is_anticlockwise = f(&rightmost).1 <= f(&leftmost).1;
        let (mut lst1, mut lst2): (Vec<Point<T>>, Vec<Point<T>>) = if is_anticlockwise {
            pointset.iter().partition(|pt| (f(pt).1 <= f(&leftmost).1))
        } else {
            pointset.iter().partition(|pt| (f(pt).1 >= f(&leftmost).1))
        };
        lst1.sort_by_key(|a| f(&a));
        lst2.sort_by_key(|a| f(&a));
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
    pub fn create_xmono_rpolygon(pointset: &[Point<T>]) -> (Vec<Point<T>>, bool) {
        Self::create_mono_rpolygon(pointset, |a| (a.xcoord, a.ycoord))
    }

    /// The function `create_ymono_rpolygon` creates a y-monotone RPolygon object from a given point
    /// set.
    ///
    /// Arguments:
    ///
    /// * `pointset`: A slice of Point objects, where each Point object has two fields: ycoord and
    /// xcoord.
    #[inline]
    pub fn create_ymono_rpolygon(pointset: &[Point<T>]) -> (Vec<Point<T>>, bool) {
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
    /// coordinates.
    /// * `q`: The parameter `q` represents the point that we want to determine if it is within the
    /// polygon or not.
    ///
    /// Returns:
    ///
    /// The function `point_in_polygon` returns a boolean value. It returns `true` if the given point
    /// `q` is strictly inside the polygon defined by the `pointset` array, `false` if the point is
    /// strictly outside the polygon, and `ub` (undefined behavior) if the point lies on the boundary of
    /// the polygon.
    pub fn point_in_rpolygon(pointset: &[Point<T>], q: &Point<T>) -> bool {
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
            pointset.push(Point::<i32>::new(*x, *y));
        }
        let (pointset, is_cw) = RPolygon::<i32>::create_ymono_rpolygon(&pointset);
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
            pointset.push(Point::<i32>::new(*x, *y));
        }
        let (pointset, is_anticw) = RPolygon::<i32>::create_xmono_rpolygon(&pointset);
        for p in pointset.iter() {
            print!("({}, {}) ", p.xcoord, p.ycoord);
        }
        let poly = RPolygon::<i32>::new(&pointset);
        assert!(!is_anticw);
        assert_eq!(poly.signed_area(), -53);
    }
}
