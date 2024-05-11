/// The `trait Overlap<T>` defines a method `overlaps` that checks if two objects of type `T` overlap
/// with each other. The `overlaps` method takes a reference to another object of type `T` as a
/// parameter and returns a boolean value indicating whether the two objects overlap or not. This trait
/// can be implemented for any type that needs to support the `overlaps` functionality.
pub trait Overlap<T> {
    fn overlaps(&self, other: &T) -> bool;
}

impl Overlap<i32> for i32 {
    fn overlaps(&self, other: &i32) -> bool {
        self == other
    }
}

/// The `trait Contain<T>` defines a method `contains` that checks if an object of type `T` is
/// contained within another object. The `contains` method takes a reference to another object of type
/// `T` as a parameter and returns a boolean value indicating whether the object is contained within the
/// other object or not. This trait can be implemented for any type that needs to support the `contains`
/// functionality.
pub trait Contain<T> {
    fn contains(&self, other: &T) -> bool;
}

impl Contain<i32> for i32 {
    fn contains(&self, other: &i32) -> bool {
        self == other
    }
}

pub trait MinDist<T> {
    fn min_dist(&self, other: &T) -> u32;
}

impl MinDist<i32> for i32 {
    fn min_dist(&self, other: &i32) -> u32 {
        (self - other).unsigned_abs()
    }
}
