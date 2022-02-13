mod point;
use crate::point::Point;

mod vector2;
use crate::vector2::Vector2;

fn main() {
    let a = Point::<i32>::new(12, 23);
    let b = Vector2::<i32>::new(34, 45);
    println!("{:?}", a + b);
    println!("{:?}", a - b);

    let mut a = Point::<i32>::new(42, 53);
    a += b;
    a -= b;
    println!("{:?}", -a);

    let c = Point::<i32>::new(12, 23);
    let mm = Point::<Point<i32>>::new(a, c);
    println!("{:?}", mm);

    // let mm = Matrix2::<i32>::new(a, b);
    // println!("{:?}", mm);
}
