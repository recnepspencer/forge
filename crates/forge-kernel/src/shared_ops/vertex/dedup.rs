//! Point deduplication by spatial tolerance.
//!
//! DOMAIN: Remove near-coincident 3D points from a flat list within a
//! caller-supplied tolerance. Used by any algorithm that accumulates
//! candidate points from multiple geometric sources and needs to eliminate
//! near-duplicates before pairing or sorting them.
//!
//! CONSUMERS: Boolean split (cut point lists), Fillet arc endpoint lists,
//! Shell offset vertex accumulation.
//!
//! INVARIANT: `point_coincidence_tol` is always supplied from `ToleranceConfig`
//! at the kernel layer — no magic numbers live here.

/// Deduplicate a list of 3D points within a spatial tolerance.
///
/// Iterates the input and retains each point only if it is farther than
/// `tolerance` from every already-retained point (Euclidean distance).
/// Order of the output matches the first occurrence of each unique point.
///
/// `tolerance` should be `ToleranceConfig::get_spatial_tolerance()` or
/// `min_edge_length` depending on context — always passed in from the kernel.
pub fn dedup_points_by_tolerance(mut pts: Vec<[f64; 3]>, tolerance: f64) -> Vec<[f64; 3]> {
    let tol_sq = tolerance * tolerance;
    let mut out: Vec<[f64; 3]> = Vec::new();
    for p in pts.drain(..) {
        let dup = out.iter().any(|q: &[f64; 3]| {
            let dx = p[0] - q[0];
            let dy = p[1] - q[1];
            let dz = p[2] - q[2];
            dx * dx + dy * dy + dz * dz <= tol_sq
        });
        if !dup {
            out.push(p);
        }
    }
    out
}
