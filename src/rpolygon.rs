use super::Point;

/**
 * @brief Rectilinear Polygon
 *
 * @tparam T
 */
pub struct RPolygon {
  private:
    Point<T> _origin;
    Vec<Vector2<T>> _vecs;  // @todo: add custom allocator support

  public:
    /**
     * @brief Construct a new RPolygon object
     *
     * @param[in] pointset
     */
    explicit constexpr RPolygon(&[Point<T>] pointset)
        : _origin{pointset.front()} {
        let mut it = pointset.begin();
        for (++it; it != pointset.end(); ++it) {
            self.vecs.push_back(*it - self.origin);
        }
    }

    /**
     * @brief
     *
     * @param[in] rhs
     * @return constexpr Point&
     */
    pub fn operator+=(&mut self, rhs: &Vector2<T>) -> RPolygon& {
        self.origin += rhs;
        return *this;
    }

    /**
     * @brief
     *
     * @return T
     */
    pub fn signed_area(&self) -> T {
        assert!(self.vecs.len() >= 1);
        let vs = &self.vecs;
        let n = vs.len();
        let mut res = vs[0].x() * vs[0].y();
        for i in 1..n {
            res += vs[i].x() * (vs[i].y() - vs[i - 1].y());
        }
        res
    }

    /**
     * @brief
     *
     * @return Point<T>
     */
    pub fn lb(&self) -> Point<T> { unimplemented!() }

    /**
     * @brief
     *
     * @return Point<T>
     */
    pub fn ub(&self) -> Point<T> { unimplemented!() }
}

impl<T> RPolygon<T> {

    /**
     * @brief Create a xmono RPolygon object
     *
     * @tparam FwIter
     * @param[in] first
     * @param[in] last
     * @return true
     * @return false
     */
    template <typename FwIter> pub fn create_xmono_rpolygon(FwIter&& first, FwIter&& last)
        -> bool {
        assert!(first != last);

        let leftmost = *std::min_element(first, last);
        let rightmost = *std::max_element(first, last);
        let is_anticlockwise = rightmost.y() <= leftmost.y();
        let mut r2l = [&](let& a) { return a.y() <= leftmost.y(); };
        let mut l2r = [&](let& a) { return a.y() >= leftmost.y(); };
        let middle = is_anticlockwise ? std::partition(first, last, std::move(r2l))
                                             : std::partition(first, last, std::move(l2r));
        std::sort(first, middle);
        std::sort(middle, last, std::greater<>());
        return is_anticlockwise;
    }

    /**
     * @brief Create a ymono RPolygon object
     *
     * @tparam FwIter
     * @param[in] first
     * @param[in] last
     * @return true
     * @return false
     */
    template <typename FwIter> pub fn create_ymono_rpolygon(FwIter&& first, FwIter&& last)
        -> bool {
        assert!(first != last);

        let mut upward = [](let& a, let& b) {
            return std::tie(a.y(), a.x()) < std::tie(b.y(), b.x());
        };
        let mut downward = [](let& a, let& b) {
            return std::tie(a.y(), a.x()) > std::tie(b.y(), b.x());
        };
        let botmost = *std::min_element(first, last, upward);
        let topmost = *std::max_element(first, last, upward);
        let is_anticlockwise = topmost.x() >= botmost.x();
        let mut r2l = [&](let& a) { return a.x() >= botmost.x(); };
        let mut l2r = [&](let& a) { return a.x() <= botmost.x(); };
        let middle = is_anticlockwise ? std::partition(first, last, std::move(r2l))
                                             : std::partition(first, last, std::move(l2r));
        std::sort(first, middle, std::move(upward));
        std::sort(middle, last, std::move(downward));
        return is_anticlockwise;
    }

    /**
     * @brief
     *
     * @tparam T
     * @tparam FwIter
     * @param[in] first
     * @param[in] last
     */
    template <typename FwIter> pub fn create_test_rpolygon(FwIter&& first, FwIter&& last) {
        assert!(first != last);

        let mut up = [](let& a, let& b) {
            return std::tie(a.y(), a.x()) < std::tie(b.y(), b.x());
        };
        let mut down = [](let& a, let& b) {
            return std::tie(a.y(), a.x()) > std::tie(b.y(), b.x());
        };
        let mut left = [](let& a, let& b) {
            return std::tie(a.x(), a.y()) < std::tie(b.x(), b.y());
        };
        let mut right = [](let& a, let& b) {
            return std::tie(a.x(), a.y()) > std::tie(b.x(), b.y());
        };

        let mut min_pt = *std::min_element(first, last, up);
        let mut max_pt = *std::max_element(first, last, up);
        let mut dx = max_pt.x() - min_pt.x();
        let mut dy = max_pt.y() - min_pt.y();
        let mut middle = std::partition(first, last, [&](let& a) {
            return dx * (a.y() - min_pt.y()) < (a.x() - min_pt.x()) * dy;
        });

        let mut max_pt1 = *std::max_element(first, middle, left);
        let mut middle2
            = std::partition(first, middle, [&](let& a) { return a.y() < max_pt1.y(); });

        let mut min_pt2 = *std::min_element(middle, last, left);
        let mut middle3
            = std::partition(middle, last, [&](let& a) { return a.y() > min_pt2.y(); });

        if (dx < 0)  // clockwise
        {
            std::sort(first, middle2, down);
            std::sort(middle2, middle, left);
            std::sort(middle, middle3, up);
            std::sort(middle3, last, right);
        } else  // anti-clockwise
        {
            std::sort(first, middle2, left);
            std::sort(middle2, middle, up);
            std::sort(middle, middle3, right);
            std::sort(middle3, last, down);
        }
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
    template <typename T>
    pub fn point_in_rpolygon(&[Point<T>] pointset, q: &Point<T>) -> bool {
        let mut c = false;
        let n = vs.len();
        let mut p0 = pointset[n - 1];
        for p1 in pointset {
            if (p1.y() <= q.y() && q.y() < p0.y()) || (p0.y() <= q.y() && q.y() < p1.y()) {
                if p1.x() > q.x() {
                    c = !c;
                }
            }
            p0 = p1;
        }
        return c;
    }

    /**
     * @brief Polygon is clockwise
     *
     * @tparam T
     * @param[in] pointset
     * @return true
     * @return false
     */
    template <typename T> pub fn rpolygon_is_clockwise(&[Point<T>] pointset) -> bool {
        let mut it1 = std::min_element(pointset.begin(), pointset.end());
        let mut it0 = it1 != pointset.begin() ? std::prev(it1) : pointset.end() - 1;
        if (it1->y() < it0->y()) return false;
        if (it1->y() > it0->y()) return true;
        // it1->y() == it0->y()
        let mut it2 = std::next(it1) != pointset.end() ? std::next(it1) : pointset.begin();
        return it2->y() > it1->y();
    }

} 
