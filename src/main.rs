mod point;
use crate::point::Point;

mod vector2;
use crate::vector2::Vector2;

extern crate interval;

use interval::ops::*;
use interval::Interval;

// mod rectangle;
// use crate::rectangle::Rect;

mod polygon;

mod rpolygon;

mod halton_int;

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

    let x = Interval::<i32>::new(12, 23);
    // let y = Interval::<i32>::new(42, 53);
    println!("{:?}", x);

    let x = Interval::<i32>::new(12, 23);
    let y = Interval::<i32>::new(42, 53);
    // let r = Rect::<i32>::new(x, y);
    // println!("{:?}", r);

    // let mm = Matrix2::<i32>::new(a, b);
    // println!("{:?}", mm);
}
