use super::{Point, Vector2};

use num_traits::{Num, Zero};

/// The `Polygon` struct represents a polygon with a generic type `T` and contains an origin point and a
/// vector of 2D vectors.
///
/// Properties:
///
/// * `origin`: The origin property represents the starting point or the reference point of the polygon.
/// It is of type Point<T>, where T is the generic type parameter of the Polygon struct.
/// * `vecs`: vecs is a vector that stores the vectors representing the sides of the polygon. Each
/// vector represents the direction and magnitude of a side of the polygon.
pub struct Polygon<T> {
    pub origin: Point<T>,
    pub vecs: Vec<Vector2<T>>,
}

impl<T: Clone + Num + Copy + std::ops::AddAssign> Polygon<T> {
    /// The `new` function constructs a new `Polygon` object by calculating the vectors between each
    /// coordinate and the origin.
    ///
    /// Arguments:
    ///
    /// * `coords`: An array slice of Point<T> objects, representing the coordinates of the polygon. The
    /// first element of the slice is considered the origin of the polygon, and the remaining elements
    /// are treated as vectors relative to the origin.
    ///
    /// Returns:
    ///
    /// The `new` function returns a new instance of the `Polygon` object.
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
    pub fn new(coords: &[Point<T>]) -> Self {
        // let origin = coords[0];
        // let mut vecs = vec![];
        // for pt in coords.iter().skip(1) {
        //     vecs.push(pt - origin);
        // }
        let (&origin, coords) = coords.split_first().unwrap();
        let vecs = coords.iter().map(|pt| pt - origin).collect();
        Polygon { origin, vecs }
    }

    /// The `signed_area_x2` function calculates the signed area multiplied by 2 of a polygon.
    ///
    /// Returns:
    ///
    /// The function `signed_area_x2` returns the signed area multiplied by 2.
    /// Signed area x 2
    ///
    /// # Panics
    ///
    /// Panics if n < 2
    ///
    /// Returns:
    ///
    /// The `new` function returns a new instance of the `Polygon` object.
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
        // let (mut vec0, vecs) = vecs.split_first().unwrap();
        // let (mut vec1, vecs) = vecs.split_first().unwrap();
        let n = vecs.len();
        let mut itr = vecs.iter();
        let mut vec0 = itr.next().unwrap();
        let mut vec1 = itr.next().unwrap();
        let mut res = vec0.x_ * vec1.y_ - vecs[n - 1].x_ * vecs[n - 2].y_;
        for vec2 in itr {
            res += vec1.x_ * (vec2.y_ - vec0.y_);
            vec0 = vec1;
            vec1 = vec2;
        }
        res
    }

    /// The function `lb` returns a `Point<T>`.
    ///
    /// Returns:
    ///
    /// a value of type `Point<T>`.
    pub fn lb(&self) -> Point<T> {
        unimplemented!()
    }

    /// The function `ub` returns a `Point<T>`.
    ///
    /// Returns:
    ///
    /// a value of type `Point<T>`.
    pub fn ub(&self) -> Point<T> {
        unimplemented!()
    }
}

impl<T: Clone + Num + Ord + Copy> Polygon<T> {
    /// The function `create_mono_polygon` takes a set of points and a function, and returns a sorted
    /// list of points that form a monotonic polygon.
    ///
    /// Arguments:
    ///
    /// * `pointset`: pointset is a slice of Point<T> objects, representing a set of points in a 2D
    /// space.
    /// * `f`: The parameter `f` is a closure that takes a reference to a `Point<T>` and returns a tuple
    /// `(T, T)`. It is used to determine the ordering of the points in the polygon.
    ///
    /// Returns:
    ///
    /// The function `create_mono_polygon` returns a `Vec<Point<T>>`, which is a vector of points.
    /// Create a x-mono Polygon object
    pub fn create_mono_polygon<F>(pointset: &[Point<T>], f: F) -> Vec<Point<T>>
    where
        F: Fn(&&Point<T>) -> (T, T),
    {
        let max_pt = pointset.iter().max_by_key(&f).unwrap();
        let min_pt = pointset.iter().min_by_key(&f).unwrap();
        let d = max_pt - min_pt;
        let (mut lst1, mut lst2): (Vec<Point<T>>, Vec<Point<T>>) = pointset
            .iter()
            .partition(|&a| d.cross(&(a - min_pt)) <= Zero::zero());
        lst1.sort_by_key(|a| f(&a));
        lst2.sort_by_key(|a| f(&a));
        lst2.reverse();
        lst1.append(&mut lst2);
        lst1
    }

    /// The function `create_xmono_polygon` creates a monotone polygon object using a given point set,
    /// with the x-coordinate as the primary sorting criterion.
    ///
    /// Arguments:
    ///
    /// * `pointset`: A slice of Point objects, where each Point object has xcoord and ycoord
    /// properties.
    ///
    /// Returns:
    ///
    /// The function `create_xmono_polygon` returns a vector of `Point<T>`.
    #[inline]
    pub fn create_xmono_polygon(pointset: &[Point<T>]) -> Vec<Point<T>> {
        Self::create_mono_polygon(pointset, |a| (a.xcoord, a.ycoord))
    }

    /// The function creates a y-monotone polygon object using a given point set.
    ///
    /// Arguments:
    ///
    /// * `pointset`: A slice of Point objects, where each Point object has two fields: ycoord and
    /// xcoord.
    ///
    /// Returns:
    ///
    /// The function `create_ymono_polygon` returns a vector of `Point<T>` objects.
    #[inline]
    pub fn create_ymono_polygon(pointset: &[Point<T>]) -> Vec<Point<T>> {
        Self::create_mono_polygon(pointset, |a| (a.ycoord, a.xcoord))
    }

    /// The function `point_in_polygon` determines if a given point is within a polygon.
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
    pub fn point_in_polygon(pointset: &[Point<T>], q: &Point<T>) -> bool {
        let n = pointset.len();
        let mut p0 = &pointset[n - 1];
        let mut c = false;
        for p1 in pointset.iter() {
            if (p1.ycoord <= q.ycoord && q.ycoord < p0.ycoord)
                || (p0.ycoord <= q.ycoord && q.ycoord < p1.ycoord)
            {
                let d = (q - p0).cross(&(p1 - p0));
                if p1.ycoord > p0.ycoord {
                    if d < Zero::zero() {
                        c = !c;
                    }
                } else {
                    // v1.ycoord < v0.ycoord
                    if d > Zero::zero() {
                        c = !c;
                    }
                }
            }
            p0 = p1;
        }
        c
    }
}

#[cfg(test)]
mod test {
    #![allow(non_upper_case_globals)]

    use super::*;

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
        let mut pointset = vec![];
        for (x, y) in coords.iter() {
            pointset.push(Point::<i32>::new(*x, *y));
        }
        let pointset = Polygon::<i32>::create_ymono_polygon(&pointset);
        for p in pointset.iter() {
            print!("({}, {}) ", p.xcoord, p.ycoord);
        }
        let poly = Polygon::<i32>::new(&pointset);
        assert_eq!(poly.signed_area_x2(), 102);
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
        let mut pointset = vec![];
        for (x, y) in coords.iter() {
            pointset.push(Point::<i32>::new(*x, *y));
        }
        let pointset = Polygon::<i32>::create_xmono_polygon(&pointset);
        for p in pointset.iter() {
            print!("({}, {}) ", p.xcoord, p.ycoord);
        }
        let poly = Polygon::<i32>::new(&pointset);
        assert_eq!(poly.signed_area_x2(), 111);
    }

    #[test]
    fn test_point_in_polygon() {
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
        let mut pointset = vec![];
        for (x, y) in coords.iter() {
            pointset.push(Point::<i32>::new(*x, *y));
        }
        let pointset = Polygon::<i32>::create_xmono_polygon(&pointset);
        // let poly = Polygon::<i32>::new(&pointset);
        let q = Point::<i32>::new(0, -1);
        assert!(Polygon::<i32>::point_in_polygon(&pointset, &q));
    }
}
