//! Distance and metric utilities.
//!
//! DOMAIN: Euclidean distance between points. Pure math, no
//! topology dependency.

/// Euclidean distance between two 3D points.
#[inline]
pub fn distance(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// Squared Euclidean distance between two 3D points.
///
/// Avoids the `sqrt` — use when only comparing distances.
#[inline]
pub fn distance_squared(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_distance() {
        assert!((distance(&[0.; 3], &[1., 0., 0.]) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn diagonal_distance() {
        let d = distance(&[0.; 3], &[1., 1., 1.]);
        assert!((d - 3.0_f64.sqrt()).abs() < 1e-14);
    }

    #[test]
    fn zero_distance() {
        assert_eq!(distance(&[5., 3., 1.], &[5., 3., 1.]), 0.0);
    }
}
