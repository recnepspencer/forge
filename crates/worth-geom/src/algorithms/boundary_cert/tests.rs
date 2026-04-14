//! Tests for boundary certification.
//!
//! DOMAIN: Spec §9.2 test matrix for planar weakly-simple certification.

#[cfg(test)]
mod tests {
    use crate::algorithms::boundary_cert::eval::*;
    use crate::algorithms::boundary_cert::schema::*;

    /// Helper: build a projected boundary from 2D vertex positions.
    ///
    /// Connects consecutive vertices as segments, closing the loop.
    fn boundary_from_vertices(vertices: &[[f64; 2]]) -> ProjectedBoundary2D {
        let n = vertices.len();
        let segments: Vec<Segment2D> = (0..n)
            .map(|i| Segment2D::new(vertices[i], vertices[(i + 1) % n], i as u64))
            .collect();
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        ProjectedBoundary2D::new(segments, frame)
    }

    #[test]
    fn clean_coplanar_merge_adjacent_quads_is_simple() {
        let boundary = boundary_from_vertices(&[[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
        let cert = certify_boundary(&boundary);
        assert_eq!(cert, WeakSimpleCertificate::Simple);
    }

    #[test]
    fn convex_pentagon_is_simple() {
        let boundary =
            boundary_from_vertices(&[[1.0, 0.0], [2.0, 0.5], [1.8, 1.5], [0.2, 1.5], [0.0, 0.5]]);
        let cert = certify_boundary(&boundary);
        assert_eq!(cert, WeakSimpleCertificate::Simple);
    }

    #[test]
    fn endpoint_self_touch_is_weakly_simple() {
        // Figure-8 boundary: two lobes meeting at vertex [1.0, 1.0].
        // The boundary touches itself at that point but does not cross.
        // Segments: (0,0)->(2,0)->(2,1)->(1,1)->(2,2)->(0,2)->(1,1)->(0,1)->(0,0)
        // The vertex [1,1] appears twice as segment endpoints but the
        // closing segment (0,1)->(0,0) has non-zero length.
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
        match cert {
            WeakSimpleCertificate::WeaklySimple { touch_count } => {
                assert!(
                    touch_count > 0,
                    "Expected touch_count > 0, got {}",
                    touch_count
                );
            }
            _ => panic!("Expected WeaklySimple, got {:?}", cert),
        }
    }

    #[test]
    fn proper_crossing_is_rejected() {
        let boundary = boundary_from_vertices(&[[0.0, 0.0], [1.0, 1.0], [1.0, 0.0], [0.0, 1.0]]);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected { reason, .. } => {
                assert_eq!(reason, BoundaryRejectReason::SelfCrossing);
            }
            _ => panic!("Expected Rejected with SelfCrossing, got {:?}", cert),
        }
    }

    #[test]
    fn overlapping_collinear_segments_rejected() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 0.0], 0),
            Segment2D::new([2.0, 0.0], [2.0, 1.0], 1),
            Segment2D::new([2.0, 1.0], [1.0, 1.0], 2),
            Segment2D::new([1.0, 1.0], [1.0, 0.0], 3),
            Segment2D::new([1.0, 0.0], [3.0, 0.0], 4),
            Segment2D::new([3.0, 0.0], [3.0, 1.0], 5),
            Segment2D::new([3.0, 1.0], [0.0, 1.0], 6),
            Segment2D::new([0.0, 1.0], [0.0, 0.0], 7),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected { reason, .. } => {
                assert_eq!(reason, BoundaryRejectReason::OverlappingSegments);
            }
            _ => panic!("Expected Rejected with OverlappingSegments, got {:?}", cert),
        }
    }

    #[test]
    fn degenerate_too_few_segments_rejected() {
        let boundary = boundary_from_vertices(&[[0.0, 0.0], [1.0, 0.0]]);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected { reason, .. } => {
                assert_eq!(reason, BoundaryRejectReason::DegenerateBoundary);
            }
            _ => panic!("Expected DegenerateBoundary, got {:?}", cert),
        }
    }

    #[test]
    fn determinism_same_input_identical_certificate() {
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

        let cert1 = certify_boundary(&boundary);
        let cert2 = certify_boundary(&boundary);
        assert_eq!(cert1, cert2);
    }

    #[test]
    fn projection_frame_deterministic_tiebreak() {
        let frame_equal_xy = build_projection_frame([1.0, 1.0, 0.0]);
        assert_eq!(frame_equal_xy.get_drop_axis(), 0);

        let frame_equal_all = build_projection_frame([1.0, 1.0, 1.0]);
        assert_eq!(frame_equal_all.get_drop_axis(), 0);

        let frame_equal_yz = build_projection_frame([0.0, 1.0, 1.0]);
        assert_eq!(frame_equal_yz.get_drop_axis(), 1);
    }

    #[test]
    fn project_boundary_to_2d_roundtrip() {
        let segments_3d = vec![
            ([0.0, 0.0, 5.0], [1.0, 0.0, 5.0], 0u64),
            ([1.0, 0.0, 5.0], [1.0, 1.0, 5.0], 1),
            ([1.0, 1.0, 5.0], [0.0, 1.0, 5.0], 2),
            ([0.0, 1.0, 5.0], [0.0, 0.0, 5.0], 3),
        ];
        let normal = [0.0, 0.0, 1.0];
        let boundary = project_boundary_to_2d(&segments_3d, normal);
        assert_eq!(boundary.segment_count(), 4);

        let cert = certify_boundary(&boundary);
        assert_eq!(cert, WeakSimpleCertificate::Simple);
    }

    #[test]
    fn adversarial_high_valence_touch_weakly_simple() {
        // A four-lobe "clover" shape that Touches at the origin 4 times but never crosses.
        // Segments alternate incoming and outgoing strictly adjacent to each other.
        let segments = vec![
            Segment2D::new([0.0, 0.0], [1.0, 1.0], 0),
            Segment2D::new([1.0, 1.0], [0.0, 1.0], 1),
            Segment2D::new([0.0, 1.0], [0.0, 0.0], 2), // Lobe 1 returns
            Segment2D::new([0.0, 0.0], [-1.0, 1.0], 3),
            Segment2D::new([-1.0, 1.0], [-1.0, 0.0], 4),
            Segment2D::new([-1.0, 0.0], [0.0, 0.0], 5), // Lobe 2 returns
            Segment2D::new([0.0, 0.0], [-1.0, -1.0], 6),
            Segment2D::new([-1.0, -1.0], [0.0, -1.0], 7),
            Segment2D::new([0.0, -1.0], [0.0, 0.0], 8), // Lobe 3 returns
            Segment2D::new([0.0, 0.0], [1.0, -1.0], 9),
            Segment2D::new([1.0, -1.0], [1.0, 0.0], 10),
            Segment2D::new([1.0, 0.0], [0.0, 0.0], 11), // Lobe 4 returns
        ];

        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);

        match cert {
            WeakSimpleCertificate::WeaklySimple { touch_count } => {
                assert_eq!(
                    touch_count, 1,
                    "Expected exactly 1 high-valence touch point at the origin"
                );
            }
            _ => panic!("Expected WeaklySimple, got {:?}", cert),
        }
    }

    #[test]
    fn adversarial_sub_epsilon_sliver_triangle() {
        // A triangle possessing an extremely tiny altitude (1e-50).
        // It's not degenerate (area > 0) and doesn't self-intersect.
        // The exact orient2d predicate must correctly recognize this as simple
        // without snapping it to collinear.
        let m = 1e-50;
        let boundary = boundary_from_vertices(&[[0.0, 0.0], [1.0, 0.0], [0.5, m]]);
        assert_eq!(certify_boundary(&boundary), WeakSimpleCertificate::Simple);
    }

    #[test]
    fn adversarial_multiple_intersections_same_segment() {
        // A long horizontal segment crossed by 3 distinct vertical segments.
        // Tests the arrangement graphs ability to correctly order atomic splits on the same domain.
        let segments = vec![
            Segment2D::new([0.0, 0.5], [4.0, 0.5], 0), // Long horizontal
            Segment2D::new([4.0, 0.5], [4.0, 0.0], 1),
            Segment2D::new([4.0, 0.0], [3.0, 0.0], 2),
            Segment2D::new([3.0, 0.0], [3.0, 1.0], 3), // Crosses at x=3
            Segment2D::new([3.0, 1.0], [2.0, 1.0], 4),
            Segment2D::new([2.0, 1.0], [2.0, 0.0], 5), // Crosses at x=2
            Segment2D::new([2.0, 0.0], [1.0, 0.0], 6),
            Segment2D::new([1.0, 0.0], [1.0, 1.0], 7), // Crosses at x=1
            Segment2D::new([1.0, 1.0], [0.0, 1.0], 8),
            Segment2D::new([0.0, 1.0], [0.0, 0.5], 9),
        ];

        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);

        match cert {
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::SelfCrossing,
                ..
            } => {}
            _ => panic!("Expected Rejected with SelfCrossing, got {:?}", cert),
        }
    }

    #[test]
    fn adversarial_collinear_spike_overlap() {
        // A segment goes out, and comes exactly back on itself.
        // Segments: (0,0) -> (2,0) -> (1,0) -> (0,1) -> (0,0)
        // (2,0) -> (1,0) overlaps collinear with (0,0) -> (2,0).
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 0.0], 0),
            Segment2D::new([2.0, 0.0], [1.0, 0.0], 1), // Overlaps
            Segment2D::new([1.0, 0.0], [0.0, 1.0], 2),
            Segment2D::new([0.0, 1.0], [0.0, 0.0], 3),
        ];

        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);

        match cert {
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::OverlappingSegments,
                ..
            } => {}
            _ => panic!("Expected OverlappingSegments, got {:?}", cert),
        }
    }

    #[test]
    fn adversarial_sub_epsilon_sliver_triangle_stays_on_fast_path() {
        // A 3-segment triangle with a 1e-50 altitude.
        // NOTE: For n=3, the fast-path pair loop has no non-adjacent pairs to check
        // (the only candidate (i=0, j=2) is skipped as the closing edge guard).
        // This test validates ONLY the fast-path exact orient2d predicate.
        // It does NOT exercise compute_splits, arrangement graph construction, or strand
        // classification. See `adversarial_sub_epsilon_sliver_forces_fallback_path` for that.
        let m = 1e-50;
        let boundary = boundary_from_vertices(&[[0.0, 0.0], [1.0, 0.0], [0.5, m]]);
        assert_eq!(certify_boundary(&boundary), WeakSimpleCertificate::Simple);
    }

    #[test]
    fn adversarial_sub_epsilon_sliver_forces_fallback_path() {
        // A hexagonal boundary where segments 1 and 4 are parallel at 2e-50 separation.
        // The fast-path orient2d sees them as collinear → NeedsFallback.
        // The exact arrangement must then prove they are disjoint (no overlap)
        // and return Simple — validating compute_splits + strand classification.
        //
        // Layout (not to scale, m = 1e-50):
        //   [0.0,  m] ——————— [1.0,  m]   seg 4 (top)
        //   [0.0,  0]         [1.0,  0]
        //   [0.0, -m] ——————— [1.0, -m]   seg 1 (bottom)
        let m = 1e-50;
        let segments = vec![
            Segment2D::new([0.0, 0.0], [0.0, -m], 0),
            Segment2D::new([0.0, -m], [1.0, -m], 1), // bottom — near-collinear with seg 4
            Segment2D::new([1.0, -m], [1.0, 0.0], 2),
            Segment2D::new([1.0, 0.0], [1.0, m], 3),
            Segment2D::new([1.0, m], [0.0, m], 4), // top — near-collinear with seg 1
            Segment2D::new([0.0, m], [0.0, 0.0], 5),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);

        // Segs 1 and 4 are exactly parallel and non-overlapping in x-projection
        // (both span [0,1]) but at different y values: -m vs +m.
        // A float-snapping certifier snaps them to the same line and may wrongly
        // reject as Overlap. The exact fallback must prove they are disjoint.
        assert_eq!(
            cert,
            WeakSimpleCertificate::Simple,
            "Sub-epsilon sliver hexagon must be Simple via exact fallback path; got {:?}",
            cert
        );
    }
}
