use crate::point::Point;

/// Removes concave vertices iteratively until polygon becomes monotone in `dir`.
pub fn rpolygon_make_monotone_hull<T, F>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
    _dir: F,
) -> Vec<Point<T, T>>
where
    T: Clone + Copy + PartialOrd + Ord + num_traits::Num
       + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
    F: Fn(&Point<T, T>) -> (T, T),
{
    if pointset.len() <= 3 { return pointset.to_vec(); }
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
                let concave = if is_anticlockwise { cross < T::zero() } else { cross > T::zero() };
                if concave {
                    pts.remove(i);
                    removed = true;
                    break;
                }
            }
            i += 1;
        }
        if !removed { break; }
    }
    pts
}

pub fn rpolygon_make_xmonotone_hull<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Point<T, T>>
where
    T: Clone + Copy + PartialOrd + Ord + num_traits::Num
       + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    rpolygon_make_monotone_hull(pointset, is_anticlockwise, |p: &Point<T, T>| (p.xcoord, p.ycoord))
}

pub fn rpolygon_make_ymonotone_hull<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Point<T, T>>
where
    T: Clone + Copy + PartialOrd + Ord + num_traits::Num
       + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    rpolygon_make_monotone_hull(pointset, is_anticlockwise, |p: &Point<T, T>| (p.ycoord, p.xcoord))
}

/// Convex hull via monotone chain (Andrew's algorithm).
/// Area property: convex hull area >= original polygon area.
pub fn rpolygon_make_convex_hull<T>(
    pointset: &[Point<T, T>],
    _is_anticlockwise: bool,
) -> Vec<Point<T, T>>
where
    T: Clone + Copy + PartialOrd + Ord + num_traits::Num
       + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    if pointset.len() <= 3 { return pointset.to_vec(); }

    let mut pts = pointset.to_vec();
    pts.sort_by(|a, b| a.xcoord.cmp(&b.xcoord).then(a.ycoord.cmp(&b.ycoord)));

    let cross = |o: &Point<T, T>, a: &Point<T, T>, b: &Point<T, T>| -> T {
        (a.xcoord - o.xcoord) * (b.ycoord - o.ycoord)
            - (a.ycoord - o.ycoord) * (b.xcoord - o.xcoord)
    };

    let mut lower = Vec::new();
    for p in &pts {
        while lower.len() >= 2 && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], p) <= T::zero() {
            lower.pop();
        }
        lower.push(*p);
    }

    let mut upper = Vec::new();
    for p in pts.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], p) <= T::zero() {
            upper.pop();
        }
        upper.push(*p);
    }

    lower.pop();
    upper.pop();
    lower.append(&mut upper);
    lower
}
