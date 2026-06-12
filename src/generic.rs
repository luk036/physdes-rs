/// Trait for checking whether two values overlap.
///
/// # Examples
///
/// ```
/// use physdes::generic::Overlap;
///
/// let a: i32 = 42;
/// let b: i32 = 42;
/// assert!(a.overlaps(&b));
///
/// let a: i32 = 42;
/// let b: i32 = 24;
/// assert!(!a.overlaps(&b));
/// ```
pub trait Overlap<T> {
    fn overlaps(&self, other: &T) -> bool;
}

/// Checks if two `i32` values are equal.
///
/// # Examples
///
/// ```
/// use physdes::generic::Overlap;
///
/// let a: i32 = 42;
/// let b: i32 = 42;
/// assert!(a.overlaps(&b));
///
/// let a: i32 = 42;
/// let b: i32 = 24;
/// assert!(!a.overlaps(&b));
/// ```
impl Overlap<i32> for i32 {
    /// Checks if two `i32` values are equal.
    #[inline]
    fn overlaps(&self, other: &i32) -> bool {
        self == other
    }
}

/// Trait for checking if one value contains another.
///
/// # Examples
///
/// ```
/// use physdes::generic::Contain;
///
/// let a: i32 = 42;
/// let b: i32 = 42;
/// assert!(a.contains(&b));
///
/// let a: i32 = 42;
/// let b: i32 = 24;
/// assert!(!a.contains(&b));
/// ```
pub trait Contain<T> {
    fn contains(&self, other: &T) -> bool;
}

/// `Contain` implementation for `i32`: returns true when values are equal.
///
/// # Examples
///
/// ```
/// use physdes::generic::Contain;
///
/// let a: i32 = 42;
/// let b: i32 = 42;
/// assert!(a.contains(&b));
///
/// let a: i32 = 42;
/// let b: i32 = 24;
/// assert!(!a.contains(&b));
/// ```
impl Contain<i32> for i32 {
    /// Checks if the current `i32` value equals the provided value.
    #[inline]
    fn contains(&self, other: &i32) -> bool {
        self == other
    }
}

/// Trait for calculating the minimum distance between two values.
///
/// # Examples
///
/// ```
/// use physdes::generic::MinDist;
///
/// let a: i32 = 10;
/// let b: i32 = 5;
/// let distance = a.min_dist_with(&b);
/// assert_eq!(distance, 5);
/// ```
pub trait MinDist<T> {
    /// Calculates the minimum distance between the current value and the provided value.
    ///
    /// # Arguments
    /// * `other` - A reference to the other value to compare against.
    ///
    /// # Returns
    /// The minimum distance between the current value and the provided value, as a `u32`.
    fn min_dist_with(&self, other: &T) -> u32;
}

/// Computes the absolute difference between two `i32` values.
///
/// # Examples
///
/// ```
/// use physdes::generic::MinDist;
///
/// let a: i32 = 10;
/// let b: i32 = 5;
/// let distance = a.min_dist_with(&b);
/// assert_eq!(distance, 5);
/// ```
impl MinDist<i32> for i32 {
    #[inline]
    fn min_dist_with(&self, other: &i32) -> u32 {
        (self - other).unsigned_abs()
    }
}

/// Trait for computing the displacement between two values.
///
/// # Examples
///
/// ```
/// use physdes::generic::Displacement;
///
/// let a: i32 = 10;
/// let b: i32 = 5;
/// let displacement = a.displace(&b);
/// assert_eq!(displacement, 5);
/// ```
pub trait Displacement<T: ?Sized> {
    type Output;

    /// Displace the current value by the provided `other` value.
    fn displace(&self, other: &T) -> Self::Output;
}

/// Computes the displacement (difference) between two `i32` values: `self - other`.
///
/// # Examples
///
/// ```
/// use physdes::generic::Displacement;
///
/// let a: i32 = 10;
/// let b: i32 = 5;
/// let displacement = a.displace(&b);
/// assert_eq!(displacement, 5);
/// ```
impl Displacement<i32> for i32 {
    type Output = i32;

    #[inline]
    fn displace(&self, other: &i32) -> Self::Output {
        self - other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        assert!(1.overlaps(&1));
        assert!(!1.overlaps(&2));
    }

    #[test]
    fn test_contain() {
        assert!(1.contains(&1));
        assert!(!1.contains(&2));
    }

    #[test]
    fn test_min_dist() {
        assert_eq!(1.min_dist_with(&1), 0);
        assert_eq!(1.min_dist_with(&2), 1);
        assert_eq!(2.min_dist_with(&1), 1);
    }

    #[test]
    fn test_displace() {
        assert_eq!(1.displace(&1), 0);
        assert_eq!(1.displace(&2), -1);
        assert_eq!(2.displace(&1), 1);
    }
}
