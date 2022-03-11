use super::Point;

/**
 * @brief Polygon
 *
 * @tparam T
 */
pub struct Polygon {
  private:
    Point<T> _origin;
    Vec<Vector2<T>> _vecs;

  public:
    /**
     * @brief Construct a new Polygon object
     *
     * @param[in] pointset
     */
    explicit constexpr Polygon(&[Point<T>] pointset) : _origin{pointset.front()} {
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
    pub fn operator+=(&mut self, rhs: &Vector2<T>) -> Polygon& {
        self.origin += rhs;
        return *this;
    }

    /**
     * @brief
     *
     * @return T
     */
    pub fn signed_area_x2(&self) -> T {
        let vs = &self.vecs;
        let n = vs.len();
        assert!(n >= 2);
        let mut res = vs[0].x() * vs[1].y() - vs[n - 1].x() * vs[n - 2].y();
        for i in 1..n-1 {
            res += vs[i].x() * (vs[i + 1].y() - vs[i - 1].y());
        }
        res;
    }

    /**
     * @brief
     *
     * @return Point<T>
     */
    pub fn lb(&self) -> Point<T>;

    /**
     * @brief
     *
     * @return Point<T>
     */
    pub fn ub(&self) -> Point<T>;
}

impl<T> Polygon<T> {

    /**
     * @brief
     *
     * @tparam Stream
     * @tparam T
     * @param[out] out
     * @param[in] r
     * @return Stream&
     */
    template <class Stream, typename T> let mut operator<<(Stream& out, r: &Polygon<T>)
        -> Stream& {
        for (let p : r) {
            out << "  \\draw " << p << ";\n";
        }
        return out;
    }

    /**
     * @brief Create a mono Polygon object
     *
     * @tparam FwIter
     * @tparam Compare
     * @param[in] first
     * @param[in] last
     * @param dir
     */
    template <typename FwIter, typename Compare>
    pub fn create_mono_polygon(FwIter&& first, FwIter&& last, Compare&& dir) {
        assert!(first != last);

        let mut max_pt = *std::max_element(first, last, dir);
        let mut min_pt = *std::min_element(first, last, dir);
        let mut d = max_pt - min_pt;
        let mut middle
            = std::partition(first, last, [&](let& a) { return d.cross(a - min_pt) <= 0; });
        std::sort(first, middle, dir);
        std::sort(middle, last, dir);
        std::reverse(middle, last);
    }

    /**
     * @brief Create a xmono Polygon object
     *
     * @tparam FwIter
     * @param[in] first
     * @param[in] last
     */
    template <typename FwIter> pub fn create_xmono_polygon(FwIter&& first, FwIter&& last) {
        return create_mono_polygon(first, last, std::less<>());
    }

    /**
     * @brief Create a ymono Polygon object
     *
     * @tparam FwIter
     * @param[in] first
     * @param[in] last
     */
    template <typename FwIter> pub fn create_ymono_polygon(FwIter&& first, FwIter&& last) {
        return create_mono_polygon(first, last, [](let& a, let& b) {
            return std::tie(a.y(), a.x()) < std::tie(b.y(), b.x());
        });
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
     * See http://www.faqs.org/faqs/graphics/algorithms-faq/ Subject 2.03
     *
     * @tparam T
     * @param[in] pointset
     * @param[in] q
     * @return true
     * @return false
     */
    template <typename T>
    pub fn point_in_polygon(&[Point<T>] pointset, q: &Point<T>) -> bool {
        let mut c = false;
        let n = vs.len();
        let mut p0 = pointset[n - 1];
        for p1 in pointset {
            if (p1.y() <= q.y() && q.y() < p0.y()) || (p0.y() <= q.y() && q.y() < p1.y()) {
                let mut d = (q - p0).cross(p1 - p0);
                if p1.y() > p0.y() {
                    if d < 0 {
                        c = !c;
                    }
                } else {  // v1.y() < v0.y()
                    if d > 0 {
                        c = !c;
                    }
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
    template <typename T> pub fn polygon_is_clockwise(&[Point<T>] pointset) -> bool {
        let mut it1 = std::min_element(pointset.begin(), pointset.end());
        let mut it0 = it1 != pointset.begin() ? std::prev(it1) : pointset.end() - 1;
        let mut it2 = std::next(it1) != pointset.end() ? std::next(it1) : pointset.begin();
        return (*it1 - *it0).cross(*it2 - *it1) < 0;
    }
}
