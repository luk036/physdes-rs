use super::{Point, Vector2};

use num_traits::{Num, Zero};

/**
 * @brief Polygon
 *
 * @tparam T
 */
pub struct Polygon<T> {
    pub origin: Point<T>,
    vecs: Vec<Vector2<T>>,
}

impl<T: Clone + Num + Copy> Polygon<T> {
    /**
     * @brief Construct a new Polygon object
     *
     * @param[in] coords
     */
    pub fn new(coords: &[Point<T>]) -> Self {
        let origin = coords[0];
        let mut vecs = vec![];
        for pt in coords.iter().skip(1) {
            vecs.push(pt - origin);
        }
        Polygon { origin, vecs }
    }

    pub fn signed_area_x2(&self) -> T {
        let vs = &self.vecs;
        let n = vs.len();
        assert!(n >= 2);
        let mut res = vs[0].x_ * vs[1].y_ - vs[n - 1].x_ * vs[n - 2].y_;
        for i in 1..n - 1 {
            res = res + vs[i].x_ * (vs[i + 1].y_ - vs[i - 1].y_);
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

impl<T: Clone + Num + Ord + Copy> Polygon<T> {
    /**
     * @brief Create a x-mono Polygon object
     */
    pub fn create_xmono_polygon(pointset: &[Point<T>]) -> Vec<Point<T>> {
        let max_pt = pointset.iter().max_by_key(|&a| (a.x_, a.y_)).unwrap();
        let min_pt = pointset.iter().min_by_key(|&a| (a.x_, a.y_)).unwrap();
        let d = max_pt - min_pt;
        let (mut lst1, mut lst2): (Vec<Point<T>>, Vec<Point<T>>) = pointset
            .iter()
            .partition(|&a| d.cross(&(a - min_pt)) <= Zero::zero());
        lst1.sort_by_key(|&a| (a.x_, a.y_));
        lst2.sort_by_key(|&a| (a.x_, a.y_));
        lst2.reverse();
        lst1.append(&mut lst2);
        lst1
    }

    /**
     * @brief Create a y-mono Polygon object
     */
    pub fn create_ymono_polygon(pointset: &[Point<T>]) -> Vec<Point<T>> {
        let max_pt = pointset.iter().max_by_key(|&a| (a.y_, a.x_)).unwrap();
        let min_pt = pointset.iter().min_by_key(|&a| (a.y_, a.x_)).unwrap();
        let d = max_pt - min_pt;
        let (mut lst1, mut lst2): (Vec<Point<T>>, Vec<Point<T>>) = pointset
            .iter()
            .partition(|&a| d.cross(&(a - min_pt)) <= Zero::zero());
        lst1.sort_by_key(|&a| (a.y_, a.x_));
        lst2.sort_by_key(|&a| (a.y_, a.x_));
        lst2.reverse();
        lst1.append(&mut lst2);
        lst1
    }

    /**
     * @brief determine if a Point is within a Polygon
     *
     * The code below is from Wm. Randolph Franklin <wrf@ecse.rpi.edu>
     * (see URL below) with some minor modifications for integer. It returns
     * true for strictly interior points, false for strictly exterior, and ub
     * for points on the boundary.  The boundary behavior is complex but
     * determined; in particular, for a partition of a region into polygons,
     * each Point is "in" exactly one Polygon.
     * (See p.243 of [O'Rourke (C)] for a discussion of boundary behavior.)
     *
     * See <http://www.faqs.org/faqs/graphics/algorithms-faq/> Subject 2.03
     *
     * @tparam T
     * @param[in] coords
     * @param[in] q
     * @return true
     * @return false
     */
    pub fn point_in_polygon(pointset: &[Point<T>], q: &Point<T>) -> bool {
        let n = pointset.len();
        let mut p0 = &pointset[n - 1];
        let mut c = false;
        for p1 in pointset.iter() {
            if (p1.y_ <= q.y_ && q.y_ < p0.y_) || (p0.y_ <= q.y_ && q.y_ < p1.y_) {
                let d = (q - p0).cross(&(p1 - p0));
                if p1.y_ > p0.y_ {
                    if d < Zero::zero() {
                        c = !c;
                    }
                } else {
                    // v1.y_ < v0.y_
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
            print!("({}, {}) ", p.x_, p.y_);
        }
        let poly = Polygon::<i32>::new(&pointset);
        assert_eq!(poly.signed_area_x2(), 102);
    }

    #[test]
    fn test_xmono_polygon() {
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
            print!("({}, {}) ", p.x_, p.y_);
        }
        let poly = Polygon::<i32>::new(&pointset);
        assert_eq!(poly.signed_area_x2(), 111);
    }
}
