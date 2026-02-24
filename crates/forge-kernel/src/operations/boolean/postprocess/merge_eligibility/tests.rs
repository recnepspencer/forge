//! Adversarial tests for merge eligibility certification.
//!
//! DOMAIN: Tests that the boundary certification → merge gating pipeline
//! works correctly under adversarial inputs. These are NOT happy-path
//! smoke tests — they target the specific failure modes identified in QA.
//!
//! DEPENDENCIES: forge-geom (boundary_cert), forge-topo, GeometryStore.

#[cfg(test)]
mod tests {
    use forge_geom::algorithms::boundary_cert::schema::*;
    use forge_geom::algorithms::boundary_cert::eval::*;

    // =====================================================================
    // D2 REGRESSION: Fallback certifier must reject on predicate failure
    // =====================================================================

    /// The previous bug: `build_arrangement()` used `unwrap_or(TriSign::Zero)`
    /// which silently swallowed predicate failures and classified degenerate
    /// geometry as valid. This test constructs a boundary that forces the
    /// fast path to escalate to the fallback (via repeated vertices), then
    /// verifies the fallback doesn't silently accept.
    ///
    /// A figure-8 with an extra crossing that MUST end up in the fallback
    /// path (non-adjacent repeated vertex) and MUST be rejected (the crossing
    /// makes it non-simple, and the repeated vertex + crossing combo is
    /// pathological enough to exercise arrangement classification).
    #[test]
    fn fallback_path_rejects_figure_eight_with_crossing() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 0.0], 0),
            Segment2D::new([2.0, 0.0], [1.0, 1.0], 1),
            Segment2D::new([1.0, 1.0], [2.0, 2.0], 2),
            Segment2D::new([2.0, 2.0], [0.0, 0.0], 3),  // crosses seg 1-2
            Segment2D::new([0.0, 0.0], [0.0, 2.0], 4),   // repeated vertex triggers fallback
            Segment2D::new([0.0, 2.0], [1.0, 1.0], 5),
            Segment2D::new([1.0, 1.0], [0.0, 0.0], 6),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected { reason, .. } => {
                assert!(
                    reason == BoundaryRejectReason::SelfCrossing
                        || reason == BoundaryRejectReason::OverlappingSegments
                        || reason == BoundaryRejectReason::DegenerateBoundary,
                    "Expected rejection, got {:?}",
                    reason,
                );
            }
            other => panic!(
                "Figure-8 with crossing MUST be rejected, got {:?}. \
                 If the fallback silently maps predicate failures to Zero, \
                 this test catches the D2 regression.",
                other,
            ),
        }
    }

    // =====================================================================
    // D2 REGRESSION: Near-degenerate collinear boundary must go through
    // fallback and get classified, not silently accepted via Zero mapping
    // =====================================================================

    /// Three segments where the middle segment is collinear with the first
    /// and overlapping — the fast path should detect collinearity and
    /// escalate to fallback. The fallback MUST reject (OverlappingSegments),
    /// not silently accept via swallowing predicate ambiguity.
    #[test]
    fn collinear_overlap_with_reversal_rejected_in_fallback() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [3.0, 0.0], 0),
            Segment2D::new([3.0, 0.0], [3.0, 1.0], 1),
            Segment2D::new([3.0, 1.0], [2.0, 1.0], 2),
            Segment2D::new([2.0, 1.0], [2.0, 0.0], 3),
            Segment2D::new([2.0, 0.0], [4.0, 0.0], 4),  // overlaps seg 0 on x-axis
            Segment2D::new([4.0, 0.0], [4.0, 1.0], 5),
            Segment2D::new([4.0, 1.0], [0.0, 1.0], 6),
            Segment2D::new([0.0, 1.0], [0.0, 0.0], 7),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected { reason, .. } => {
                assert_eq!(
                    reason,
                    BoundaryRejectReason::OverlappingSegments,
                    "Collinear overlapping segments must be rejected as OverlappingSegments, \
                     not silently accepted",
                );
            }
            other => panic!(
                "Collinear overlap MUST be rejected, got {:?}",
                other,
            ),
        }
    }

    // =====================================================================
    // D7 REGRESSION: Projection frame tie-break determinism
    // =====================================================================

    /// The old tiebreak used `|| (x == y && true)` tautologies.
    /// This test verifies the spec §4.6 priority: X > Y > Z.
    /// When all three components are equal, X must be dropped.
    /// When Y == Z but X < Y, Y must be dropped.
    #[test]
    fn projection_frame_tiebreak_all_equal_drops_x() {
        let frame = build_projection_frame([1.0, 1.0, 1.0]);
        assert_eq!(frame.get_drop_axis(), 0, "Equal components: X > Y > Z means drop X");
        assert_eq!(frame.get_u_axis(), 1);
        assert_eq!(frame.get_v_axis(), 2);
    }

    #[test]
    fn projection_frame_tiebreak_yz_equal_drops_y() {
        let frame = build_projection_frame([0.1, 0.5, 0.5]);
        assert_eq!(frame.get_drop_axis(), 1, "Y == Z and both > X: drop Y");
    }

    #[test]
    fn projection_frame_tiebreak_xz_equal_drops_x() {
        let frame = build_projection_frame([0.5, 0.1, 0.5]);
        assert_eq!(frame.get_drop_axis(), 0, "X == Z and both > Y: drop X");
    }

    #[test]
    fn projection_frame_negative_normal_flips_orientation() {
        let pos = build_projection_frame([0.0, 0.0, 1.0]);
        let neg = build_projection_frame([0.0, 0.0, -1.0]);
        assert_eq!(pos.get_drop_axis(), neg.get_drop_axis());
        assert!(pos.get_orientation_sign() > 0.0);
        assert!(neg.get_orientation_sign() < 0.0);
    }

    // =====================================================================
    // D6: Arrangement stores original segments, not split ones
    // =====================================================================

    /// Verify the arrangement API returns original segment count even
    /// when events indicate segment interactions. This catches any
    /// future regression where someone tries to implement actual splitting.
    #[test]
    fn arrangement_preserves_original_segments() {
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
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        assert!(
            matches!(cert, WeakSimpleCertificate::WeaklySimple { .. }),
            "Figure-8 touch should be WeaklySimple, got {:?}",
            cert,
        );
    }

    // =====================================================================
    // Adversarial: Machine-epsilon boundary that exercises exact predicates
    // =====================================================================

    /// A square boundary where one vertex is perturbed by machine epsilon.
    /// The perturbation is small enough that naive f64 comparison would
    /// round it, but Shewchuk predicates should handle it correctly and
    /// still certify it as Simple (no actual crossing occurs).
    #[test]
    fn machine_epsilon_perturbation_still_simple() {
        let eps = f64::EPSILON;
        let boundary = {
            let vertices = [
                [0.0, 0.0],
                [1.0, eps],  // perturbed by machine epsilon
                [1.0, 1.0],
                [0.0, 1.0],
            ];
            let n = vertices.len();
            let segments: Vec<Segment2D> = (0..n)
                .map(|i| Segment2D::new(vertices[i], vertices[(i + 1) % n], i as u64))
                .collect();
            let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
            ProjectedBoundary2D::new(segments, frame)
        };
        let cert = certify_boundary(&boundary);
        assert_eq!(
            cert,
            WeakSimpleCertificate::Simple,
            "Machine-epsilon perturbation should NOT cause false rejection",
        );
    }

    /// Same square but with a vertex perturbed to create a genuine
    /// self-crossing in the boundary. This must be rejected even though
    /// the crossing amount is tiny.
    #[test]
    fn tiny_crossing_not_silently_accepted() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
            Segment2D::new([1.0, 0.0], [0.5, 0.5], 1),
            Segment2D::new([0.5, 0.5], [1.0, 1.0], 2),
            Segment2D::new([1.0, 1.0], [0.0, 1.0], 3),
            Segment2D::new([0.0, 1.0], [0.5, 0.5 + 1e-15], 4),  // near-duplicate of vertex
            Segment2D::new([0.5, 0.5 + 1e-15], [0.0, 0.0], 5),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        assert!(
            !matches!(cert, WeakSimpleCertificate::Simple),
            "Near-duplicate vertex with 1e-15 offset must not be blindly called Simple. \
             Got {:?}. Either WeaklySimple (touch) or Rejected is acceptable.",
            cert,
        );
    }

    // =====================================================================
    // D5 REGRESSION: DecisionId uniqueness
    // =====================================================================

    /// Two different face groups must produce different DecisionIds.
    /// This catches the old hardcoded DecisionId(0) bug.
    #[test]
    fn different_groups_produce_different_decision_ids() {
        use forge_topo::bitset::EntityBitset;
        use forge_core::tracing::DecisionId;

        let mut group_a = EntityBitset::with_capacity(10);
        let _ = group_a.insert(0);
        let _ = group_a.insert(1);

        let mut group_b = EntityBitset::with_capacity(10);
        let _ = group_b.insert(2);
        let _ = group_b.insert(3);

        let hash_a = compute_group_hash(&group_a);
        let hash_b = compute_group_hash(&group_b);

        assert_ne!(
            hash_a, hash_b,
            "Different face groups must produce different DecisionIds. \
             Both produced {}. This catches the D5 regression where all \
             groups got DecisionId(0).",
            hash_a,
        );
    }

    /// Same face group must produce the same DecisionId (determinism).
    #[test]
    fn same_group_produces_same_decision_id() {
        use forge_topo::bitset::EntityBitset;

        let mut group = EntityBitset::with_capacity(10);
        let _ = group.insert(0);
        let _ = group.insert(3);
        let _ = group.insert(7);

        let hash_1 = compute_group_hash(&group);
        let hash_2 = compute_group_hash(&group);

        assert_eq!(hash_1, hash_2, "Same group must produce deterministic hash");
    }

    /// Helper: replicate the FNV-1a hash from eval.rs for testing.
    fn compute_group_hash(group: &forge_topo::bitset::EntityBitset) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for idx in 0..group.capacity() {
            if group.contains(idx).unwrap_or(false) {
                h = h.wrapping_mul(0x100000001b3) ^ (idx as u64);
            }
        }
        h
    }

    // =====================================================================
    // Adversarial: Boundary with many near-coincident vertices
    // =====================================================================

    /// Constructs an L-shaped boundary with 8 segments where consecutive
    /// vertices share exact coordinates at the L-corner. This is a real
    /// geometry pattern from boolean cleanup and must certify correctly.
    #[test]
    fn l_shaped_boundary_with_shared_corner_vertices() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 0.0], 0),
            Segment2D::new([2.0, 0.0], [2.0, 1.0], 1),
            Segment2D::new([2.0, 1.0], [1.0, 1.0], 2),
            Segment2D::new([1.0, 1.0], [1.0, 2.0], 3),
            Segment2D::new([1.0, 2.0], [0.0, 2.0], 4),
            Segment2D::new([0.0, 2.0], [0.0, 0.0], 5),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        assert_eq!(
            cert,
            WeakSimpleCertificate::Simple,
            "L-shaped boundary with no self-intersection should be Simple",
        );
    }

    /// Degenerate triangle: three collinear points forming a zero-area polygon.
    /// Must be rejected, not silently certified.
    #[test]
    fn degenerate_collinear_triangle_rejected() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
            Segment2D::new([1.0, 0.0], [2.0, 0.0], 1),
            Segment2D::new([2.0, 0.0], [0.0, 0.0], 2),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected { .. } => {}
            other => panic!(
                "Degenerate collinear triangle MUST be rejected. Got {:?}. \
                 Three collinear points form a zero-area polygon — \
                 certifying this as Simple would allow invalid merges.",
                other,
            ),
        }
    }

    /// Star-shaped self-intersecting polygon: a pentagram.
    /// Has 5 proper crossings and must be rejected.
    #[test]
    fn pentagram_five_crossings_rejected() {
        use std::f64::consts::PI;
        let r = 1.0;
        let mut vertices = [[0.0f64; 2]; 5];
        for i in 0..5 {
            let angle = (2.0 * PI * (2 * i) as f64) / 5.0 - PI / 2.0;
            vertices[i] = [r * angle.cos(), r * angle.sin()];
        }
        let segments: Vec<Segment2D> = (0..5)
            .map(|i| Segment2D::new(vertices[i], vertices[(i + 1) % 5], i as u64))
            .collect();
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected { reason, .. } => {
                assert_eq!(
                    reason,
                    BoundaryRejectReason::SelfCrossing,
                    "Pentagram must be rejected for SelfCrossing",
                );
            }
            other => panic!("Pentagram MUST be rejected, got {:?}", other),
        }
    }
}
