//! Point utility helpers.
//!
//! DOMAIN: Stateless comparisons and arithmetic on `[f64; 3]` point values.

/// Return `true` when all coordinate deltas are within `tolerance`.
pub fn is_same_point_within(a: &[f64; 3], b: &[f64; 3], tolerance: f64) -> bool {
    (a[0] - b[0]).abs() < tolerance
        && (a[1] - b[1]).abs() < tolerance
        && (a[2] - b[2]).abs() < tolerance
}
