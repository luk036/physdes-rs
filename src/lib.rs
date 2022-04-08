pub mod vector2;
pub mod point;
pub mod halton_int;
pub mod polygon;
pub mod rpolygon;

pub use crate::point::Point;
pub use crate::vector2::Vector2;
pub use crate::polygon::Polygon;
pub use crate::rpolygon::RPolygon;

extern crate interval;
use interval::ops::*;
use interval::Interval;

// mod rectangle;
// use crate::rectangle::Rect;


pub fn it_works() {
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

    let x = Interval::<i32>::new(12, 23);
    // let y = Interval::<i32>::new(42, 53);
    println!("{:?}", x);

    let _x = Interval::<i32>::new(12, 23);
    let _y = Interval::<i32>::new(42, 53);
    // let r = Rect::<i32>::new(x, y);
    // println!("{:?}", r);

    // let mm = Matrix2::<i32>::new(a, b);
    // println!("{:?}", mm);
}
