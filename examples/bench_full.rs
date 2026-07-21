use std::hint::black_box;
use std::time::Instant;

use physdes::dme_algorithm::{
    DMEAlgorithm, LinearDelayCalculator, Sink,
};
use physdes::global_router::GlobalRoutingTree;
use physdes::steiner_forest_grid::steiner_forest_grid;
use physdes::Point;

fn bench_dme(sinks: &[Sink]) -> f64 {
    let n = 1000;
    let mut accum = 0usize;
    let start = Instant::now();
    for _ in 0..n {
        let calc = Box::new(LinearDelayCalculator::new(0.5, 0.1));
        let mut dme = DMEAlgorithm::new(sinks.to_vec(), calc);
        let root = dme.build_clock_tree();
        accum += black_box(root);
    }
    let ns = start.elapsed().as_nanos() as f64 / n as f64;
    black_box(accum);
    ns
}

fn main() {
    println!("=== Rust (physdes-rs) — Full Algorithm Benchmarks ===\n");

    // 1. DME Algorithm
    {
        let sinks: Vec<Sink> = (0..16)
            .map(|i| {
                let x = i as i32 * 33 + 7;
                let y = (i as i32 * 17) % 200;
                Sink::new(&format!("s{}", i), Point::new(x, y), 1.0)
            })
            .collect();
        let ns = bench_dme(&sinks);
        println!("  {:<40} {:>8.1} ns/op  ({} sinks)", "DME build_clock_tree (16 sinks)", ns, sinks.len());
    }

    {
        let sinks: Vec<Sink> = (0..64)
            .map(|i| {
                let x = i as i32 * 8 + 5;
                let y = (i as i32 * 31 + 7) % 500;
                Sink::new(&format!("s{}", i), Point::new(x, y), 1.0)
            })
            .collect();
        let ns = bench_dme(&sinks);
        println!("  {:<40} {:>8.1} ns/op  ({} sinks)", "DME build_clock_tree (64 sinks)", ns, sinks.len());
    }

    // 2. Global Routing
    {
        let terminals = vec![
            Point::new(10, 20), Point::new(30, 50), Point::new(60, 10),
            Point::new(80, 40), Point::new(40, 70), Point::new(90, 90),
            Point::new(20, 80), Point::new(70, 30), Point::new(50, 60),
            Point::new(100, 100),
        ];
        let n = 50000u32;
        let start = Instant::now();
        for _ in 0..n {
            let mut tree = GlobalRoutingTree::new(Point::new(0, 0));
            for &pt in &terminals {
                tree.insert_terminal_node(pt, None);
            }
            black_box(tree.calculate_total_wirelength());
        }
        let ns = start.elapsed().as_nanos() as f64 / n as f64;
        println!("  {:<40} {:>8.1} ns/op  (10 terminals)", "Global routing (10 terminals)", ns);
    }

    // 3. Steiner Forest Grid
    {
        let pairs = vec![
            ((0, 0), (5, 5)), ((2, 1), (7, 3)), ((1, 4), (6, 2)),
            ((3, 0), (8, 5)), ((0, 3), (4, 7)), ((2, 6), (9, 1)),
            ((5, 2), (7, 6)), ((3, 5), (6, 8)), ((1, 7), (9, 9)),
        ];
        let n = 10000;
        let start = Instant::now();
        for _ in 0..n {
            black_box(steiner_forest_grid(10, 10, &pairs));
        }
        let ns = start.elapsed().as_nanos() as f64 / n as f64;
        println!("  {:<40} {:>8.1} ns/op  (10x10 grid, 9 pairs)", "Steiner Forest Grid (small)", ns);
    }

    {
        let pairs: Vec<((usize, usize), (usize, usize))> = (0..49)
            .map(|i| {
                let sx = (i * 3) % 20;
                let sy = (i * 7) % 20;
                let tx = (i * 11 + 5) % 20;
                let ty = (i * 13 + 3) % 20;
                ((sx, sy), (tx, ty))
            })
            .collect();
        let n = 500;
        let start = Instant::now();
        for _ in 0..n {
            black_box(steiner_forest_grid(20, 20, &pairs));
        }
        let ns = start.elapsed().as_nanos() as f64 / n as f64;
        println!("  {:<40} {:>8.1} ns/op  (20x20 grid, 49 pairs)", "Steiner Forest Grid (large)", ns);
    }
}
