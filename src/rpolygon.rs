use super::{Point, Vector2};
use num_traits::Num;
// use core::ops::{Add, Neg, Sub};

/**
 * @brief Rectilinear Polygon
 *
 * @tparam T
 */
pub struct RPolygon<T> {
    pub origin: Point<T>,
    vecs: Vec<Vector2<T>>,
}

impl<T: Clone + Num + Copy> RPolygon<T> {
    /**
     * @brief Construct a new RPolygon object
     *
     * @param[in] coords
     */
    pub fn new(coords: &[Point<T>]) -> Self {
        let origin = coords[0];
        let mut vecs = vec![];
        for pt in coords.iter().skip(1) {
            vecs.push(pt - origin);
        }
        RPolygon { origin, vecs }
    }

    /**
     * @brief
     *
     * @return T
     */
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
    pub fn create_xmono_rpolygon(pointset: &[Point<T>]) -> (Vec<Point<T>>, bool) {
        let rightmost = pointset.iter().max_by_key(|&a| (a.x_, a.y_)).unwrap();
        let leftmost = pointset.iter().min_by_key(|&a| (a.x_, a.y_)).unwrap();
        let is_anticlockwise = rightmost.y_ <= leftmost.y_;
        let (mut lst1, mut lst2): (Vec<Point<T>>, Vec<Point<T>>) = if is_anticlockwise {
            pointset.iter().partition(|&pt| (pt.y_ <= leftmost.y_))
        } else {
            pointset.iter().partition(|&pt| (pt.y_ >= leftmost.y_))
        };
        lst1.sort_by_key(|&a| (a.x_, a.y_));
        lst2.sort_by_key(|&a| (a.x_, a.y_));
        lst2.reverse();
        lst1.append(&mut lst2);
        (lst1, is_anticlockwise)
    }

    pub fn create_ymono_rpolygon(pointset: &[Point<T>]) -> (Vec<Point<T>>, bool) {
        let topmost = pointset.iter().max_by_key(|&a| (a.y_, a.x_)).unwrap();
        let botmost = pointset.iter().min_by_key(|&a| (a.y_, a.x_)).unwrap();
        let is_anticlockwise = topmost.y_ >= botmost.y_;
        let (mut lst1, mut lst2): (Vec<Point<T>>, Vec<Point<T>>) = if is_anticlockwise {
            pointset.iter().partition(|&pt| (pt.x_ >= botmost.x_))
        } else {
            pointset.iter().partition(|&pt| (pt.x_ <= botmost.x_))
        };
        lst1.sort_by_key(|&a| (a.y_, a.x_));
        lst2.sort_by_key(|&a| (a.y_, a.x_));
        lst2.reverse();
        lst1.append(&mut lst2);
        (lst1, is_anticlockwise)
    }

    /**
     * @brief determine if a Point is within a Polygon
     *
     * The code below is from Wm. Randolph Franklin <wrf@ecse.rpi.edu>
     * (see URL below) with some minor modifications for rectilinear. It returns
     * true for strictly interior points, false for strictly exterior, and ub
     * for points on the boundary.  The boundary behavior is complex but
     * determined; in particular, for a partition of a region into polygons,
     * each Point is "in" exactly one Polygon.
     * (See p.243 of [O'Rourke (C)] for a discussion of boundary behavior.)
     *
     * See http://www.faqs.org/faqs/graphics/algorithms-faq/ Subject 2.03
     *
     * @tparam T
     * @param[in] pointset
     * @param[in] q
     * @return true
     * @return false
     */
    pub fn point_in_rpolygon(pointset: &[Point<T>], q: &Point<T>) -> bool {
        let mut c = false;
        let n = pointset.len();
        let mut p0 = &pointset[n - 1];
        for p1 in pointset.iter() {
            if ((p1.y_ <= q.y_ && q.y_ < p0.y_) || (p0.y_ <= q.y_ && q.y_ < p1.y_)) && p1.x_ > q.x_
            {
                c = !c;
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
    pub fn test_ymono_rpolygon() {
        let coords = vec![
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
        let (pointset, is_anticw) = RPolygon::<i32>::create_ymono_rpolygon(&pointset);
        for p in pointset.iter() {
            print!("({}, {}) ", p.x_, p.y_);
        }
        let poly = RPolygon::<i32>::new(&pointset);
        assert!(is_anticw);
        assert_eq!(poly.signed_area(), 45);
    }

    #[test]
    pub fn test_xmono_rpolygon() {
        let coords = vec![
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
            print!("({}, {}) ", p.x_, p.y_);
        }
        let poly = RPolygon::<i32>::new(&pointset);
        assert!(!is_anticw);
        assert_eq!(poly.signed_area(), -53);
    }
}
