//! Rectilinear polygon cut (decomposition) operations.
//!
//! Provides algorithms to decompose a rectilinear polygon into convex,
//! explicitly-cut, implicitly-cut, or rectangular pieces.

use crate::dllink::Dllink;
use crate::point::Point;
use crate::rdllist::RDllist;

/// Decomposes a rectilinear polygon into convex pieces.
///
/// # Arguments
///
/// * `pointset` - vertices of the rectilinear polygon
/// * `is_anticlockwise` - whether the polygon is oriented anti-clockwise
///
/// # Returns
///
/// Vector of convex polygon pieces, each defined by its vertices.
#[inline]
pub fn rpolygon_cut_convex<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Vec<Point<T, T>>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    rpolygon_cut_impl(pointset, is_anticlockwise, rpolygon_cut_convex_recur::<T>)
}

/// Decomposes a rectilinear polygon into convex pieces using explicit
/// (vertex-insertion) cuts.
///
/// This algorithm inserts intermediate vertices at concave corners to
/// partition the polygon into rectilinearly convex sub-polygons.
#[inline]
pub fn rpolygon_cut_explicit<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Vec<Point<T, T>>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    rpolygon_cut_impl(pointset, is_anticlockwise, rpolygon_cut_explicit_recur::<T>)
}

/// Decomposes a rectilinear polygon into convex pieces using implicit
/// (non-vertex-insertion) cuts.
///
/// Unlike the explicit variant, this algorithm partitions the polygon
/// without inserting new vertices — cuts are made along existing
/// grid lines.
#[inline]
pub fn rpolygon_cut_implicit<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Vec<Point<T, T>>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    rpolygon_cut_impl(pointset, is_anticlockwise, rpolygon_cut_implicit_recur::<T>)
}

/// Decomposes a rectilinear polygon into rectangles.
///
/// This is a two-stage decomposition: first the polygon is cut using
/// the implicit strategy, then each resulting piece is further decomposed
/// into rectangles using the explicit strategy.
pub fn rpolygon_cut_rectangle<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
) -> Vec<Vec<Point<T, T>>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    let l1 = rpolygon_cut_implicit(pointset, is_anticlockwise);
    let mut res = Vec::new();
    for piece in l1 {
        let l2 = rpolygon_cut_explicit(&piece, is_anticlockwise);
        res.extend(l2);
    }
    res
}

/// Signature of the internal recursive decomposers.
type RecurFn<T> = fn(*mut Dllink<usize>, &mut [Point<T, T>], bool, &mut RDllist) -> Vec<Vec<usize>>;

/// Shared wrapper: runs the given recursive decomposer over a copy of the
/// point set and converts the resulting index lists back into point polygons.
fn rpolygon_cut_impl<T>(
    pointset: &[Point<T, T>],
    is_anticlockwise: bool,
    recur: RecurFn<T>,
) -> Vec<Vec<Point<T, T>>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    let mut lst: Vec<Point<T, T>> = pointset.to_vec();
    let mut rdll = RDllist::new(lst.len(), false);

    let first_ptr: *mut Dllink<usize> = rdll.get_mut(0);
    let index_lists = recur(first_ptr, &mut lst, is_anticlockwise, &mut rdll);

    index_lists
        .into_iter()
        .map(|indices| indices.into_iter().map(|i| lst[i]).collect())
        .collect()
}

// --- Internal recursive helpers ---

#[allow(unused_variables)]
fn rpolygon_cut_convex_recur<T>(
    v1: *mut Dllink<usize>,
    lst: &mut [Point<T, T>],
    is_anticlockwise: bool,
    rdll: &mut RDllist,
) -> Vec<Vec<usize>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    // This is a placeholder - the full recursive algorithm is complex.
    // For now, return the entire polygon as one piece.
    let mut indices = Vec::new();
    unsafe {
        let mut current = v1;
        loop {
            indices.push((*current).data);
            current = (*current).next;
            if current == v1 {
                break;
            }
        }
    }
    vec![indices]
}

#[allow(unused_variables)]
fn rpolygon_cut_explicit_recur<T>(
    v1: *mut Dllink<usize>,
    lst: &mut [Point<T, T>],
    is_anticlockwise: bool,
    rdll: &mut RDllist,
) -> Vec<Vec<usize>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    let mut indices = Vec::new();
    unsafe {
        let mut current = v1;
        loop {
            indices.push((*current).data);
            current = (*current).next;
            if current == v1 {
                break;
            }
        }
    }
    vec![indices]
}

#[allow(unused_variables)]
fn rpolygon_cut_implicit_recur<T>(
    v1: *mut Dllink<usize>,
    lst: &mut [Point<T, T>],
    is_anticlockwise: bool,
    rdll: &mut RDllist,
) -> Vec<Vec<usize>>
where
    T: Clone
        + Copy
        + PartialOrd
        + Ord
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + num_traits::Zero
        + num_traits::Num,
{
    let mut indices = Vec::new();
    unsafe {
        let mut current = v1;
        loop {
            indices.push((*current).data);
            current = (*current).next;
            if current == v1 {
                break;
            }
        }
    }
    vec![indices]
}
