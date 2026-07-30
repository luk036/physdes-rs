use std::hint::black_box;
use std::time::Instant;

use physdes::Point;
use physdes::Polygon;
use physdes::RPolygon;

fn make_square_poly(i: i32) -> Polygon<i32> {
    Polygon::new(&[
        Point::new(i * 10, i * 10),
        Point::new(i * 10 + 100, i * 10),
        Point::new(i * 10 + 100, i * 10 + 100),
        Point::new(i * 10, i * 10 + 100),
    ])
}

fn make_rpoly(i: i32) -> RPolygon<i32> {
    RPolygon::new(&[
        Point::new(i * 10, i * 10),
        Point::new(i * 10 + 100, i * 10 + 100),
    ])
}

fn bench_polygon_signed_area_x2() {
    let polys: Vec<Polygon<i32>> = (0..1000).map(make_square_poly).collect();
    let mut accum = 0i64;
    let n = 100_000;
    let start = Instant::now();
    for _ in 0..n {
        for p in &polys {
            accum += black_box(p.signed_area_x2()) as i64;
        }
    }
    let ns = start.elapsed().as_nanos() as f64 / (n as f64 * polys.len() as f64);
    println!(
        "  {:<35} {:>8.2} ns/op  (accum={})",
        "Polygon signed_area_x2", ns, accum
    );
}

fn bench_rpolygon_signed_area() {
    let polys: Vec<RPolygon<i32>> = (0..1000).map(make_rpoly).collect();
    let mut accum = 0i64;
    let n = 100_000;
    let start = Instant::now();
    for _ in 0..n {
        for p in &polys {
            accum += black_box(p.signed_area()) as i64;
        }
    }
    let ns = start.elapsed().as_nanos() as f64 / (n as f64 * polys.len() as f64);
    println!(
        "  {:<35} {:>8.2} ns/op  (accum={})",
        "RPolygon signed_area", ns, accum
    );
}

fn main() {
    println!("=== Rust (physdes-rs) — Polygon Area Benchmarks ===\n");
    bench_polygon_signed_area_x2();
    bench_rpolygon_signed_area();
}
