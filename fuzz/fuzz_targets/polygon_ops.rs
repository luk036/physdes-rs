#![no_main]
use libfuzzer_sys::fuzz_target;
use physdes::{Point, Polygon};

fuzz_target!(|data: &[u8]| {
    if data.len() < 6 {
        return;
    }

    // Create points from fuzzer input
    let mut points = Vec::new();
    let chunks = data.chunks(8);

    for chunk in chunks.take(10) {
        if chunk.len() >= 8 {
            let x = i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let y = i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
            points.push(Point::new(x, y));
        }
    }

    if points.len() < 3 {
        return;
    }

    // Test polygon creation and operations
    let polygon = Polygon::new(&points);

    // Test area calculation (should not panic)
    let _area = polygon.area();

    // Test convexity check (should not panic)
    let _is_convex = polygon.is_convex();

    // Test bounding box (should not panic)
    let _bbox = polygon.bounding_box();

    // Test vertices (should not panic)
    let _vertices = polygon.vertices();

    // Test rectilinear check (should not panic)
    let _is_rectilinear = polygon.is_rectilinear();
});