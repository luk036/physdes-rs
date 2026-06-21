//! Targeted micro-benchmarks comparing Rust patterns relevant to EDA tool development.
//!
//! Experiments:
//! 1. Linked list traversal: raw pointer vs index-based arena
//! 2. Trait dispatch: static (monomorphization) vs dynamic (dyn trait) vs free function
//! 3. Error handling: Result vs panic vs Option
//! 4. Polygon cut: measure the real polygon decomposition cost
//! 5. Vec reallocation invalidation: measure cost of pre-reserved vs dynamic

use std::hint::black_box;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use physdes::generic::MinDist;
use physdes::interval::Interval;
use physdes::rpolygon_cut::{rpolygon_cut_convex, rpolygon_cut_explicit, rpolygon_cut_rectangle};
use physdes::rpolygon_hull::rpolygon_make_convex_hull;
use physdes::Point;

// ============================================================
// EXPERIMENT 1: Linked list traversal — raw pointer vs arena index
// ============================================================

/// A Dllink using raw pointers (matching physdes-rs current design)
#[repr(C)]
struct PtrNode {
    next: *mut PtrNode,
    prev: *mut PtrNode,
    data: usize,
}

/// A Dllink using arena indices (the alternative)
struct IndexNode {
    next: usize,
    prev: usize,
    data: usize,
}

/// Arena using raw pointers — the current physdes-rs approach
struct PtrArena {
    nodes: Vec<PtrNode>,
}

impl PtrArena {
    fn new(n: usize) -> Self {
        let mut nodes: Vec<PtrNode> = Vec::with_capacity(n);
        for i in 0..n {
            nodes.push(PtrNode {
                next: std::ptr::null_mut(),
                prev: std::ptr::null_mut(),
                data: i,
            });
        }
        // Link them circularly
        let len = nodes.len();
        for i in 0..len {
            let next_i = if i == len - 1 { 0 } else { i + 1 };
            let prev_i = if i == 0 { len - 1 } else { i - 1 };
            let next_ptr: *mut PtrNode = &mut nodes[next_i];
            let prev_ptr: *mut PtrNode = &mut nodes[prev_i];
            nodes[i].next = next_ptr;
            nodes[i].prev = prev_ptr;
        }
        PtrArena { nodes }
    }

    /// Traverse all nodes via raw pointer chasing
    fn traverse_ptr(&self) -> usize {
        let head: *const PtrNode = &self.nodes[0];
        let mut sum = 0usize;
        let mut current = head;
        loop {
            unsafe {
                sum += (*current).data;
                current = (*current).next;
                if current == head {
                    break;
                }
            }
        }
        sum
    }
}

/// Arena using indices (no unsafe)
struct IndexArena {
    nodes: Vec<IndexNode>,
}

impl IndexArena {
    fn new(n: usize) -> Self {
        let mut nodes: Vec<IndexNode> = Vec::with_capacity(n);
        for i in 0..n {
            nodes.push(IndexNode {
                next: 0,
                prev: 0,
                data: i,
            });
        }
        let len = nodes.len();
        for (i, node) in nodes.iter_mut().enumerate() {
            let next_i = if i == len - 1 { 0 } else { i + 1 };
            let prev_i = if i == 0 { len - 1 } else { i - 1 };
            node.next = next_i;
            node.prev = prev_i;
        }
        IndexArena { nodes }
    }

    /// Traverse all nodes via index chasing
    fn traverse_index(&self) -> usize {
        let mut sum = 0usize;
        let mut current = 0usize;
        let len = self.nodes.len();
        for _ in 0..len {
            sum += self.nodes[current].data;
            current = self.nodes[current].next;
        }
        sum
    }
}

fn bench_ptr_arena_traverse(c: &mut Criterion) {
    let arena = PtrArena::new(1000);
    c.bench_function("ptr_arena_traverse_1k", |b| {
        b.iter(|| black_box(&arena).traverse_ptr())
    });
}

fn bench_idx_arena_traverse(c: &mut Criterion) {
    let arena = IndexArena::new(1000);
    c.bench_function("idx_arena_traverse_1k", |b| {
        b.iter(|| black_box(&arena).traverse_index())
    });
}

// ============================================================
// EXPERIMENT 2: Trait dispatch overhead
// ============================================================

// Use the physdes-rs MinDist trait for dispatch comparison
// which IS dyn-compatible (no Self in parameter)
fn bench_static_dispatch(c: &mut Criterion) {
    let iv_a = Interval::new(0, 100_000i32);
    let iv_b = Interval::new(50_000, 200_000i32);
    c.bench_function("trait_static_dispatch", |bench| {
        bench.iter(|| black_box(&iv_a).min_dist_with(black_box(&iv_b)))
    });
}

fn bench_dynamic_dispatch(c: &mut Criterion) {
    let iv_a = Interval::new(0, 100_000i32);
    let iv_b = Interval::new(50_000, 200_000i32);
    let da: &dyn MinDist<Interval<i32>> = &iv_a;
    c.bench_function("trait_dynamic_dispatch", |bench| {
        bench.iter(|| da.min_dist_with(black_box(&iv_b)))
    });
}

fn bench_free_function(c: &mut Criterion) {
    let val_a = 100_000i32;
    let val_b = 200_000i32;
    c.bench_function("trait_free_function", |bench| {
        bench.iter(|| black_box(val_a).abs_diff(black_box(val_b)))
    });
}

// ============================================================
// EXPERIMENT 3: Error handling overhead
// ============================================================

fn divide_result(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

fn divide_option(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn divide_panic(a: i32, b: i32) -> i32 {
    assert_ne!(b, 0, "division by zero");
    a / b
}

fn bench_error_result_ok(c: &mut Criterion) {
    c.bench_function("error_result_ok_path", |b| {
        b.iter(|| black_box(divide_result(black_box(100), black_box(5)).unwrap()))
    });
}

fn bench_error_option_ok(c: &mut Criterion) {
    c.bench_function("error_option_ok_path", |b| {
        b.iter(|| black_box(divide_option(black_box(100), black_box(5)).unwrap()))
    });
}

fn bench_error_panic_ok(c: &mut Criterion) {
    c.bench_function("error_panic_ok_path", |b| {
        b.iter(|| black_box(divide_panic(black_box(100), black_box(5))))
    });
}

// ============================================================
// EXPERIMENT 4: Real polygon decomposition cost
// ============================================================

fn make_pointset(n: usize) -> Vec<Point<i32, i32>> {
    // Generate a pseudo-random point set that forms a valid rectilinear polygon
    let mut pts = Vec::with_capacity(n);
    for i in 0..n {
        let x = (i * 7 % 100) as i32;
        let y = (i * 13 % 100) as i32;
        pts.push(Point::new(x, y));
    }
    pts
}

fn bench_rpolygon_cut_convex(c: &mut Criterion) {
    let pts = make_pointset(50);
    let is_anticw = true;
    c.bench_function("rpolygon_cut_convex_50pts", |b| {
        b.iter(|| {
            let result = rpolygon_cut_convex(black_box(&pts), black_box(is_anticw));
            black_box(result)
        })
    });
}

fn bench_rpolygon_cut_explicit(c: &mut Criterion) {
    let pts = make_pointset(50);
    let is_anticw = true;
    c.bench_function("rpolygon_cut_explicit_50pts", |b| {
        b.iter(|| {
            let result = rpolygon_cut_explicit(black_box(&pts), black_box(is_anticw));
            black_box(result)
        })
    });
}

fn bench_rpolygon_cut_rectangle(c: &mut Criterion) {
    let pts = make_pointset(50);
    let is_anticw = true;
    c.bench_function("rpolygon_cut_rectangle_50pts", |b| {
        b.iter(|| {
            let result = rpolygon_cut_rectangle(black_box(&pts), black_box(is_anticw));
            black_box(result)
        })
    });
}

fn bench_rpolygon_convex_hull(c: &mut Criterion) {
    let pts = make_pointset(50);
    let is_anticw = true;
    c.bench_function("rpolygon_convex_hull_50pts", |b| {
        b.iter(|| {
            let result = rpolygon_make_convex_hull(black_box(&pts), black_box(is_anticw));
            black_box(result)
        })
    });
}

// ============================================================
// EXPERIMENT 5: Vec reallocation cost (impacting arena safety)
// ============================================================

fn bench_vec_pre_reserved(c: &mut Criterion) {
    c.bench_function("vec_pre_reserved_1000_push", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(1000);
            for i in 0..1000 {
                v.push(black_box(i));
            }
            black_box(v.len())
        })
    });
}

fn bench_vec_no_reserve(c: &mut Criterion) {
    c.bench_function("vec_no_reserve_1000_push", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in 0..1000 {
                v.push(black_box(i));
            }
            black_box(v.len())
        })
    });
}

criterion_group! {
    name = eda_compare;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);
    targets =
        bench_ptr_arena_traverse,
        bench_idx_arena_traverse,
        bench_static_dispatch,
        bench_dynamic_dispatch,
        bench_free_function,
        bench_error_result_ok,
        bench_error_option_ok,
        bench_error_panic_ok,
        bench_rpolygon_cut_convex,
        bench_rpolygon_cut_explicit,
        bench_rpolygon_cut_rectangle,
        bench_rpolygon_convex_hull,
        bench_vec_pre_reserved,
        bench_vec_no_reserve,
}

criterion_main!(eda_compare);
