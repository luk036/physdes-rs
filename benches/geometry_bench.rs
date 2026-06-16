// Criterion benchmarks for physdes-rs geometric operations

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use physdes::generic::{MinDist, Overlap};
use physdes::interval::Intersect;
use physdes::interval::Interval;
use physdes::{Point, Polygon, Vector2};

fn bench_point_creation(c: &mut Criterion) {
    c.bench_function("point_creation", |b| {
        b.iter(|| Point::new(black_box(100), black_box(200)))
    });
}

fn bench_vector_operations(c: &mut Criterion) {
    let v1 = Vector2::new(100, 200);
    let v2 = Vector2::new(50, 75);

    c.bench_function("vector_addition", |b| {
        b.iter(|| black_box(v1) + black_box(v2))
    });

    c.bench_function("vector_dot_product", |b| {
        b.iter(|| black_box(v1).dot(black_box(&v2)))
    });

    c.bench_function("vector_norm", |b| b.iter(|| black_box(v1).norm_inf()));
}

fn bench_interval_operations(c: &mut Criterion) {
    let interval_a = Interval::new(0, 100);
    let interval_b = Interval::new(50, 150);

    c.bench_function("interval_overlap_check", |b| {
        b.iter(|| black_box(&interval_a).overlaps(black_box(&interval_b)))
    });

    c.bench_function("interval_intersection", |b| {
        b.iter(|| black_box(&interval_a).intersect_with(black_box(&interval_b)))
    });
}

fn bench_polygon_creation(c: &mut Criterion) {
    let points = vec![
        Point::new(0, 0),
        Point::new(10, 0),
        Point::new(10, 10),
        Point::new(0, 10),
    ];

    c.bench_function("polygon_creation", |b| {
        b.iter(|| Polygon::new(black_box(&points)))
    });
}

fn bench_polygon_area(c: &mut Criterion) {
    let points = vec![
        Point::new(0, 0),
        Point::new(100, 0),
        Point::new(100, 100),
        Point::new(0, 100),
    ];
    let polygon = Polygon::new(&points);

    c.bench_function("polygon_area_calculation", |b| {
        b.iter(|| black_box(&polygon).area())
    });
}

fn bench_polygon_convex_check(c: &mut Criterion) {
    let points = vec![
        Point::new(0, 0),
        Point::new(100, 0),
        Point::new(100, 100),
        Point::new(0, 100),
    ];
    let polygon = Polygon::new(&points);

    c.bench_function("polygon_convex_check", |b| {
        b.iter(|| black_box(&polygon).is_convex())
    });
}

fn bench_point_distance(c: &mut Criterion) {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(100, 200);

    c.bench_function("point_distance_calculation", |b| {
        b.iter(|| black_box(&p1).min_dist_with(black_box(&p2)))
    });
}

criterion_group!(
    benches,
    bench_point_creation,
    bench_vector_operations,
    bench_interval_operations,
    bench_polygon_creation,
    bench_polygon_area,
    bench_polygon_convex_check,
    bench_point_distance
);
criterion_main!(benches);
