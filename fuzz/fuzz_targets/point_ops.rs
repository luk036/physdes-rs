#![no_main]
use libfuzzer_sys::fuzz_target;
use physdes::{Point, Vector2};

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 {
        return;
    }

    // Create points from fuzzer input
    let x1 = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let y1 = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let x2 = i32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    let y2 = i32::from_le_bytes([data[12], data[13], data[14], data[15]]);

    let p1 = Point::new(x1, y1);
    let p2 = Point::new(x2, y2);

    // Test point arithmetic (should not panic)
    let vec = p2 - p1;
    let p3 = p1 + vec;

    // Test point comparison (should not panic)
    let _eq = p1 == p2;
    let _lt = p1 < p2;
    let _le = p1 <= p2;

    // Test with vectors if we have more data
    if data.len() >= 24 {
        let vx = i32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let vy = i32::from_le_bytes([data[20], data[21], data[22], data[23]]);

        let v = Vector2::new(vx, vy);

        // Test point-vector arithmetic (should not panic)
        let p4 = p1 + v;
        let p5 = p2 - v;

        // Test distance calculation (should not panic)
        let _dist = p1.dist_to_point(&p2);
    }
});