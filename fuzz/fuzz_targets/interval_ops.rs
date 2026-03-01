#![no_main]
use libfuzzer_sys::fuzz_target;
use physdes::interval::Interval;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }

    // Create intervals from fuzzer input
    let a = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let b = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);

    let lower = a.min(b);
    let upper = a.max(b);

    let interval = Interval::new(lower, upper);

    // Test interval operations (should not panic)
    let _length = interval.length();
    let _lb = interval.lb();
    let _ub = interval.ub();

    // Test with another interval if we have enough data
    if data.len() >= 16 {
        let c = i32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let d = i32::from_le_bytes([data[12], data[13], data[14], data[15]]);

        let lower2 = c.min(d);
        let upper2 = c.max(d);

        let interval2 = Interval::new(lower2, upper2);

        // Test overlap (should not panic)
        let _overlaps = interval.overlaps(&interval2);

        // Test intersection (should not panic)
        let _intersection = interval.intersect(&interval2);

        // Test convex hull (should not panic)
        let _hull = interval.convex_hull(&interval2);

        // Test contains (should not panic)
        let _contains = interval.contains(&c);
    }
});