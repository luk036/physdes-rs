/// The `trait Overlap<T>` defines a method `overlaps` that checks if two objects of type `T` overlap
/// with each other. The `overlaps` method takes a reference to another object of type `T` as a
/// parameter and returns a boolean value indicating whether the two objects overlap or not. This trait
/// can be implemented for any type that needs to support the `overlaps` functionality.
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
/// This implementation of the `Overlap` trait for `i32` simply checks if the two values are equal.
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
    /// The `overlaps` function in Rust checks if two references to `i32` values point to the same
    /// memory location.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `overlaps` function is a reference to an `i32` value.
    ///
    /// Returns:
    ///
    /// The `overlaps` function is returning a boolean value indicating whether the value of `self` is
    /// equal to the value of `other`.
    #[inline]
    fn overlaps(&self, other: &i32) -> bool {
        self == other
    }
}

/// The `trait Contain<T>` defines a method `contains` that checks if an object of type `T` is
/// contained within another object. The `contains` method takes a reference to another object of type
/// `T` as a parameter and returns a boolean value indicating whether the object is contained within the
/// other object or not. This trait can be implemented for any type that needs to support the `contains`
/// functionality.
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

/// Checks if the current `i32` value is equal to the provided `i32` value.
///
/// This implementation of the `Contain` trait for `i32` simply compares the two values for equality.
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
    /// The function checks if a given value is equal to another value.
    ///
    /// Arguments:
    ///
    /// * `other`: The `other` parameter in the `contains` function is a reference to an `i32` value
    ///   that is being compared with `self`.
    ///
    /// Returns:
    ///
    /// The `contains` function is returning a boolean value indicating whether the value of `self` is
    /// equal to the value of the reference `other`.
    #[inline]
    fn contains(&self, other: &i32) -> bool {
        self == other
    }
}

/// Defines a trait for calculating the minimum distance between two values of type `T`.
///
/// This trait provides a single method, `min_dist_with`, which takes a reference to another value of type `T`
/// and returns the minimum distance between the two values as a `u32`.
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
///
/// let a: i32 = 5;
/// let b: i32 = 10;
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
/// This implementation of the `MinDist` trait for `i32` calculates the unsigned
/// absolute difference between the current `i32` value and the provided `other`
/// `i32` value.
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
///
/// let a: i32 = 5;
/// let b: i32 = 10;
/// let distance = a.min_dist_with(&b);
/// assert_eq!(distance, 5);
/// ```
impl MinDist<i32> for i32 {
    #[inline]
    fn min_dist_with(&self, other: &i32) -> u32 {
        (self - other).unsigned_abs()
    }
}

/// The `Displacement` trait defines a way to displace a value of type `T` by another value of type `T`.
///
/// The `displace` method takes a reference to a `T` and returns a new value of the associated `Output` type,
/// which represents the displaced value.
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
///
/// let a: i32 = 5;
/// let b: i32 = 10;
/// let displacement = a.displace(&b);
/// assert_eq!(displacement, -5);
/// ```
pub trait Displacement<T: ?Sized> {
    type Output;

    /// Displace the current value by the provided `other` value.
    fn displace(&self, other: &T) -> Self::Output;
}

/// Implements the `Displacement` trait for `i32` types, providing a `displace` method that subtracts
/// the given `i32` value from the current `i32` value.
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
///
/// let a: i32 = 5;
/// let b: i32 = 10;
/// let displacement = a.displace(&b);
/// assert_eq!(displacement, -5);
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
