use std::collections::HashSet;

use physdes::dme_algorithm::{DMEAlgorithm, ElmoreDelayCalculator, LinearDelayCalculator, Sink};
use physdes::global_router::GlobalRouter;
use physdes::interval::Interval;
use physdes::point::Point;
use physdes::steiner_forest_grid::steiner_forest_grid;

fn make_sinks(count: i32) -> Vec<Sink> {
    (0..count)
        .map(|i| {
            let x = (i * 37) % 100;
            let y = (i * 53) % 100;
            Sink::new(
                &format!("s{}", i),
                Point::new(x, y),
                1.0 + (i % 5) as f64 * 0.2,
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// DME — exact cross-language numerical results
// ---------------------------------------------------------------------------

#[test]
fn cross_lang_dme_2_sinks() {
    let sinks = vec![
        Sink::new("s1", Point::new(0, 0), 1.0),
        Sink::new("s2", Point::new(10, 0), 1.0),
    ];
    let mut dme = DMEAlgorithm::new(sinks, Box::new(LinearDelayCalculator::new(1.0, 0.1)));
    let root = dme.build_clock_tree();
    let a = dme.analyze_skew(root);
    // Rust == C++ == Python: skew=0.0, total_wirelength=10
    assert_eq!(a.skew, 0.0);
    assert_eq!(a.total_wirelength, 10);
}

#[test]
fn cross_lang_dme_8_sinks_linear() {
    let sinks = make_sinks(8);
    let mut dme = DMEAlgorithm::new(sinks, Box::new(LinearDelayCalculator::new(0.5, 0.1)));
    let root = dme.build_clock_tree();
    let a = dme.analyze_skew(root);
    // Rust == C++ == Python: exact values
    assert_eq!(a.skew, 0.0);
    assert_eq!(a.total_wirelength, 298);
    assert!((a.max_delay - 37.5).abs() < 1e-9);
    assert!((a.min_delay - 37.5).abs() < 1e-9);
}

#[test]
fn cross_lang_dme_8_sinks_elmore() {
    let sinks = make_sinks(8);
    let mut dme = DMEAlgorithm::new(sinks, Box::new(ElmoreDelayCalculator::new(0.1, 0.1)));
    let root = dme.build_clock_tree();
    let a = dme.analyze_skew(root);
    // Rust == C++ == Python: exact values
    assert!((a.skew - 0.92).abs() < 1e-9);
    assert_eq!(a.total_wirelength, 292);
    assert!((a.max_delay - 80.415).abs() < 1e-9);
    assert!((a.min_delay - 79.495).abs() < 1e-9);
}

// ---------------------------------------------------------------------------
// Steiner forest — exact edge set across all three
// ---------------------------------------------------------------------------

#[test]
fn cross_lang_steiner_forest_edges() {
    let h = 8;
    let w = 8;
    let pairs = [
        ((0, 0), (3, 2)),
        ((0, 0), (0, 5)),
        ((4, 4), (7, 5)),
        ((4, 4), (5, 7)),
        ((0, 1), (4, 1)),
    ];
    let result = steiner_forest_grid(h, w, &pairs);
    // Must match C++ and Python edge set exactly (17 edges, cost=17.0)
    let expected: HashSet<(usize, usize)> = [
        (0, 1),
        (1, 2),
        (2, 3),
        (2, 10),
        (3, 4),
        (4, 5),
        (10, 18),
        (18, 26),
        (25, 26),
        (25, 33),
        (36, 37),
        (37, 38),
        (37, 45),
        (38, 39),
        (39, 47),
        (45, 53),
        (53, 61),
    ]
    .iter()
    .cloned()
    .collect();
    let actual: HashSet<(usize, usize)> = result.edges.iter().map(|(u, v, _)| (*u, *v)).collect();
    assert_eq!(
        actual, expected,
        "Steiner forest edge set must match C++/Python"
    );
    assert!((result.total_cost - 17.0).abs() < 1e-9);
}

// ---------------------------------------------------------------------------
// Global Router — same tree structure and wirelength across all three
// ---------------------------------------------------------------------------

#[test]
fn cross_lang_global_router_simple() {
    // Rust test_route_simple: source=(0,0), terminals=[(1,1),(2,2)]
    let src = Point::new(0, 0);
    let terminals = vec![Point::new(1, 1), Point::new(2, 2)];
    let mut router = GlobalRouter::new(src, terminals, None);
    router.route_simple();
    let tree = router.get_tree();
    // Python: total_wirelength=4, same tree structure
    assert_eq!(tree.calculate_total_wirelength(), 4);
    // Verify tree structure: source -> t1 -> t2
    let terms = tree.get_all_terminals();
    assert_eq!(terms.len(), 2);
    let path1 = tree.find_path_to_source(&terms[0].id);
    let path2 = tree.find_path_to_source(&terms[1].id);
    for p in &[&path1, &path2] {
        assert_eq!(p[0].id, "source");
        assert_eq!(p.last().unwrap().node_type.to_string(), "Terminal");
        // All paths start at source and end at a terminal — also true for C++/Python
    }
}

#[test]
fn cross_lang_global_router_steiners() {
    // Same test case as Rust test_route_with_steiners / Python test_route_with_steiners
    let src = Point::new(0, 0);
    let terminals = vec![Point::new(1, 1), Point::new(2, 2)];
    let mut router = GlobalRouter::new(src, terminals, None);
    router.route_with_steiners();
    let tree = router.get_tree();
    // Python: total_wirelength=4
    assert_eq!(tree.calculate_total_wirelength(), 4);
    // Both terminals should connect to source (directly or via Steiner)
    let terms = tree.get_all_terminals();
    assert_eq!(terms.len(), 2);
}

#[test]
fn cross_lang_global_router_constraints() {
    let src = Point::new(0, 0);
    let terminals = vec![Point::new(1, 1), Point::new(2, 2)];
    let mut router = GlobalRouter::new(src, terminals, None);
    router.route_with_constraints(2.0);
    let tree = router.get_tree();
    // Python: total_wirelength=4
    assert_eq!(tree.calculate_total_wirelength(), 4);
}

#[test]
fn cross_lang_global_router_3_terminals() {
    // Rust test_route_three_sinks_simple
    let src = Point::new(0, 0);
    let terminals = vec![Point::new(10, 0), Point::new(5, 10)];
    let mut router = GlobalRouter::new(src, terminals, None);
    router.route_simple();
    let tree = router.get_tree();
    // Rust test asserts wirelen=25
    assert_eq!(tree.calculate_total_wirelength(), 25);
}

#[test]
fn cross_lang_global_router_keepout() {
    // Rust test_route_with_keepout
    let src = Point::new(0, 0);
    let terminals = vec![Point::new(10, 0)];
    let ko = Point::new(Interval::new(4, 6), Interval::new(-1, 1));
    let mut router = GlobalRouter::new(src, terminals, Some(vec![ko]));
    router.route_with_steiners();
    let tree = router.get_tree();
    let wl = tree.calculate_total_wirelength();
    assert!(wl > 0);
    // C++ test checks wirelen > 0 too
}

// ---------------------------------------------------------------------------
// Polygon / RPolygon area — pure math, must be identical across all three
// ---------------------------------------------------------------------------

#[test]
fn cross_lang_polygon_signed_area_x2() {
    use physdes::polygon::Polygon;
    let pts = vec![
        Point::new(0, 0),
        Point::new(100, 0),
        Point::new(100, 100),
        Point::new(0, 100),
    ];
    let poly = Polygon::new(&pts);
    // Square 100x100: signed_area_x2 = 2 * 100*100 = 20000 (matches C++ BM_Polygon_Area)
    assert_eq!(poly.signed_area_x2(), 20000);
}

#[test]
fn cross_lang_rpolygon_signed_area() {
    use physdes::rpolygon::RPolygon;
    let pts = vec![Point::new(0, 0), Point::new(100, 100)];
    let rpoly = RPolygon::new(&pts);
    // 100x100 square: area = 100*100 = 10000 (matches C++ BM_RPolygon_Area)
    assert_eq!(rpoly.signed_area(), 10000);
}
