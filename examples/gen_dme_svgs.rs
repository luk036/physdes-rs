/// Example: generate DME clock tree SVGs with various sink configurations.
///
/// Run:  cargo run --example gen_dme_svgs
///
/// Produces: dme_diamond.svg, dme_circle.svg, dme_elbow.svg,
///           dme_random.svg, dme_long.svg, dme_varicap.svg

use physdes::dme_algorithm::{
    DMEAlgorithm, LinearDelayCalculator, Sink,
};
use physdes::dme_visualizer::ClockTreeVisualizer;
use physdes::point::Point;

fn main() {
    let calc = || Box::new(LinearDelayCalculator::new(0.5, 0.2));
    let viz = ClockTreeVisualizer::new();

    // Dataset 1: 4 sinks in a diamond pattern
    let s1 = vec![
        Sink::new("s1", Point::new(0, 40), 1.0),
        Sink::new("s2", Point::new(40, 0), 1.0),
        Sink::new("s3", Point::new(0, -40), 1.0),
        Sink::new("s4", Point::new(-40, 0), 1.0),
    ];
    let mut d1 = DMEAlgorithm::new(s1.clone(), calc());
    let r1 = d1.build_clock_tree();
    let a1 = d1.analyze_skew(r1);
    viz.visualize_tree(d1.get_tree(), r1, &s1, "dme_diamond.svg", 800, 600, Some(&a1));
    println!("Diamond: skew={:.3} ({:.2}%) wl={}", a1.skew, a1.skew / a1.max_delay * 100.0, a1.total_wirelength);

    // Dataset 2: 8 sinks on a circle
    let s2 = (0..8)
        .map(|i| {
            let angle = std::f64::consts::TAU * i as f64 / 8.0;
            let x = (angle.cos() * 50.0).round() as i32;
            let y = (angle.sin() * 50.0).round() as i32;
            Sink::new(&format!("s{}", i + 1), Point::new(x, y), 1.0)
        })
        .collect::<Vec<_>>();
    let mut d2 = DMEAlgorithm::new(s2.clone(), calc());
    let r2 = d2.build_clock_tree();
    let a2 = d2.analyze_skew(r2);
    viz.visualize_tree(d2.get_tree(), r2, &s2, "dme_circle.svg", 800, 600, Some(&a2));
    println!("Circle:  skew={:.3} ({:.2}%) wl={}", a2.skew, a2.skew / a2.max_delay * 100.0, a2.total_wirelength);

    // Dataset 3: 3 sinks in an L-shape
    let s3 = vec![
        Sink::new("s1", Point::new(0, 0), 2.0),
        Sink::new("s2", Point::new(60, 0), 1.0),
        Sink::new("s3", Point::new(60, 40), 1.0),
    ];
    let mut d3 = DMEAlgorithm::new(s3.clone(), calc());
    let r3 = d3.build_clock_tree();
    let a3 = d3.analyze_skew(r3);
    viz.visualize_tree(d3.get_tree(), r3, &s3, "dme_elbow.svg", 800, 600, Some(&a3));
    println!("Elbow:   skew={:.3} ({:.2}%) wl={}", a3.skew, a3.skew / a3.max_delay * 100.0, a3.total_wirelength);

    // Dataset 4: 5 unbalanced sinks
    let s4 = vec![
        Sink::new("s1", Point::new(10, 80), 1.0),
        Sink::new("s2", Point::new(30, 10), 1.0),
        Sink::new("s3", Point::new(50, 60), 1.0),
        Sink::new("s4", Point::new(70, 20), 1.0),
        Sink::new("s5", Point::new(90, 50), 1.0),
    ];
    let mut d4 = DMEAlgorithm::new(s4.clone(), calc());
    let r4 = d4.build_clock_tree();
    let a4 = d4.analyze_skew(r4);
    viz.visualize_tree(d4.get_tree(), r4, &s4, "dme_random.svg", 800, 600, Some(&a4));
    println!("Random:  skew={:.3} ({:.2}%) wl={}", a4.skew, a4.skew / a4.max_delay * 100.0, a4.total_wirelength);

    // Dataset 5: 2 sinks far apart
    let s5 = vec![
        Sink::new("s1", Point::new(0, 0), 1.0),
        Sink::new("s2", Point::new(100, 80), 1.0),
    ];
    let mut d5 = DMEAlgorithm::new(s5.clone(), calc());
    let r5 = d5.build_clock_tree();
    let a5 = d5.analyze_skew(r5);
    viz.visualize_tree(d5.get_tree(), r5, &s5, "dme_long.svg", 800, 600, Some(&a5));
    println!("Long:    skew={:.3} ({:.2}%) wl={}", a5.skew, a5.skew / a5.max_delay * 100.0, a5.total_wirelength);

    // Dataset 6: 3 sinks with varying capacitances
    let s6 = vec![
        Sink::new("s1", Point::new(0, 0), 0.5),
        Sink::new("s2", Point::new(40, 0), 3.0),
        Sink::new("s3", Point::new(20, 40), 1.0),
    ];
    let mut d6 = DMEAlgorithm::new(s6.clone(), calc());
    let r6 = d6.build_clock_tree();
    let a6 = d6.analyze_skew(r6);
    viz.visualize_tree(d6.get_tree(), r6, &s6, "dme_varicap.svg", 800, 600, Some(&a6));
    println!("Varicap: skew={:.3} ({:.2}%) wl={}", a6.skew, a6.skew / a6.max_delay * 100.0, a6.total_wirelength);

    println!("\nAll 6 SVG files generated. Not checked-in.");
}
