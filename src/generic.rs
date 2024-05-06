pub trait Overlap<T> {
    fn overlaps(&self, other: &T) -> bool;
}

impl Overlap<i32> for i32 {
    fn overlaps(&self, other: &i32) -> bool {
        self == other
    }
}

