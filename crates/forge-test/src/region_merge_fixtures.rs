//! Reusable fixtures for region-merge certifier/gate/policy tests.
//!
//! These are intentionally small deterministic builders for:
//! - certifier-rejected boundaries (proper crossing)
//! - weakly-simple boundaries (endpoint touch)
//! - simple boundaries
//! - deterministic face-group bitsets / hash inputs

use forge_geom::algorithms::boundary_cert::schema::{ProjectedBoundary2D, ProjectionFrame2D, Segment2D};
use forge_topo::bitset::EntityBitset;

/// Deterministic simple square boundary (strictly simple).
pub fn simple_square_boundary_2d() -> ProjectedBoundary2D {
    let segments = vec![
        Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
        Segment2D::new([1.0, 0.0], [1.0, 1.0], 1),
        Segment2D::new([1.0, 1.0], [0.0, 1.0], 2),
        Segment2D::new([0.0, 1.0], [0.0, 0.0], 3),
    ];
    ProjectedBoundary2D::new(segments, ProjectionFrame2D::new(2, 0, 1, 1.0))
}

/// Endpoint self-touch figure-8 boundary (weakly simple, non-crossing).
pub fn weakly_simple_endpoint_touch_boundary_2d() -> ProjectedBoundary2D {
    let segments = vec![
        Segment2D::new([0.0, 0.0], [2.0, 0.0], 0),
        Segment2D::new([2.0, 0.0], [2.0, 1.0], 1),
        Segment2D::new([2.0, 1.0], [1.0, 1.0], 2),
        Segment2D::new([1.0, 1.0], [2.0, 2.0], 3),
        Segment2D::new([2.0, 2.0], [0.0, 2.0], 4),
        Segment2D::new([0.0, 2.0], [1.0, 1.0], 5),
        Segment2D::new([1.0, 1.0], [0.0, 1.0], 6),
        Segment2D::new([0.0, 1.0], [0.0, 0.0], 7),
    ];
    ProjectedBoundary2D::new(segments, ProjectionFrame2D::new(2, 0, 1, 1.0))
}

/// Proper-crossing boundary (must be rejected by the certifier).
pub fn rejected_crossing_boundary_2d() -> ProjectedBoundary2D {
    let segments = vec![
        Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
        Segment2D::new([1.0, 0.0], [0.0, 1.0], 1),
        Segment2D::new([0.0, 1.0], [1.0, 1.0], 2),
        Segment2D::new([1.0, 1.0], [0.0, 0.0], 3),
    ];
    ProjectedBoundary2D::new(segments, ProjectionFrame2D::new(2, 0, 1, 1.0))
}

/// Build a deterministic face bitset from indices (all inserted in given order).
pub fn face_group_bitset(capacity: usize, indices: &[u32]) -> EntityBitset {
    let mut group = EntityBitset::with_capacity(
        u32::try_from(capacity).expect("fixture capacity must fit in u32"),
    );
    for &idx in indices {
        group.insert(idx).expect("fixture capacity must cover face indices");
    }
    group
}

/// Deterministic hash helper matching the FNV-1a face-index group hashing pattern
/// used by region-merge certifier decision IDs.
pub fn hash_face_group_indices(group: &EntityBitset) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for idx in 0..group.capacity() {
        if group.contains(idx).expect("fixture bitset contains must stay in-range") {
            h = h.wrapping_mul(0x100000001b3) ^ (idx as u64);
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_geom::algorithms::boundary_cert::eval::certify_boundary;
    use forge_geom::algorithms::boundary_cert::schema::{BoundaryRejectReason, WeakSimpleCertificate};

    #[test]
    fn fixture_rejected_boundary_produces_expected_cert_outcome() {
        let cert = certify_boundary(&rejected_crossing_boundary_2d());
        assert!(matches!(
            cert,
            WeakSimpleCertificate::Rejected { reason: BoundaryRejectReason::SelfCrossing, .. }
        ));
    }

    #[test]
    fn fixture_weakly_simple_boundary_produces_touch_count() {
        let cert = certify_boundary(&weakly_simple_endpoint_touch_boundary_2d());
        match cert {
            WeakSimpleCertificate::WeaklySimple { touch_count } => {
                assert!(touch_count > 0);
            }
            other => panic!("expected WeaklySimple fixture outcome, got {:?}", other),
        }
    }

    #[test]
    fn fixture_group_hash_is_deterministic() {
        let group = face_group_bitset(16, &[0, 3, 7]);
        assert_eq!(hash_face_group_indices(&group), hash_face_group_indices(&group));

        let same_set_different_insert_order = face_group_bitset(16, &[7, 0, 3]);
        assert_eq!(
            hash_face_group_indices(&group),
            hash_face_group_indices(&same_set_different_insert_order),
            "hash should depend on membership set, not insertion order"
        );
    }
}
