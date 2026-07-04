//! Memory profiling for the DME algorithm.
//!
//! Run:
//!   cargo run --example profile_dme
//!
//! dhat writes `dhat-heap.json` at exit.

use dhat::{HeapStats, Profiler};
use physdes::dme_algorithm::{
    DMEAlgorithm, LinearDelayCalculator, Sink,
};
use physdes::Point;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = Profiler::builder().testing().build();

    // Create sinks in a grid pattern (N×N = up to 100 sinks)
    let n = 10;
    let mut sinks = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            sinks.push(Sink::new(
                &format!("s{}_{}", i, j),
                Point::new((i * 10) as i32, (j * 10) as i32),
                0.01,
            ));
        }
    }

    let calculator = Box::new(LinearDelayCalculator::new(0.1, 0.0002));
    let mut dme = DMEAlgorithm::new(sinks, calculator);
    let root = dme.build_clock_tree();
    let tree = dme.get_tree();
    let _ = tree.get(root);

    let stats = HeapStats::get();
    println!("=== dhat HeapStats ===");
    println!("  Total bytes allocated: {}", stats.total_bytes);
    println!("  Total blocks:          {}", stats.total_blocks);
    println!("  Max bytes live:        {}", stats.max_bytes);
    println!("  Current bytes in use:  {}", stats.curr_bytes);
    println!("  Current blocks:        {}", stats.curr_blocks);
}
