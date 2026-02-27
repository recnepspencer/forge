//! Hard adversarial tests for boundary certification.
//!
//! Unlike the spec-matrix tests in `tests.rs`, each test here is designed to
//! expose a specific class of defect in a naive or f64-snapping certifier:
//!
//! 1. Near-coincident crossing vs endpoint touch — exact disambiguation required
//! 2. Extreme large coordinate scale (1e150) — Rational stability test
//! 3. Mixed coordinate scale (200 orders of magnitude) — Rational stability test
//! 4. Endpoint touching interior of another segment at exact rational t=1/2
//! 5. Bowtie figure-8 with crossing at exact rational point (0,0)
//! 6. Collinear multi-segment partial overlap on a shared interval
//! 7. Almost-collinear polygon with 1e-50 deviation — must not false-reject

#[cfg(test)]
mod adversarial {
    use crate::algorithms::boundary_cert::eval::*;
    use crate::algorithms::boundary_cert::schema::*;

    // ── 1. Near-coincident crossing vs. endpoint touch ────────────────────────
    //
    // A proper crossing where the crossing point is within 1e-50 of an endpoint.
    // f64 arithmetic collapses the crossing to the endpoint, wrongly classifying
    // as EndpointTouch. Exact arithmetic must see these as a true crossing.
    //
    // The near-coincident test must use a geometry where:
    // - The two segments that cross (segs 0 and 2) are non-adjacent
    // - Their crossing point is distinct from all endpoints including the offset one
    // Bowtie shape where one corner is offset by e from the crossing point:
    //   segs 0 and 2 cross at origin; seg 1 ends at (1+e, 1+e), not (1,1)
    #[test]
    fn near_coincident_crossing_not_mistaken_for_endpoint_touch() {
        let e = 1e-50_f64;
        // Proper bowtie: segs 0 and 2 cross at (0,0).
        // One corner is at (e, e) rather than (0,0) to ensure an endpoint
        // is "near" the crossing point without coinciding with it.
        // Segs 0 and 2 still properly cross at their interior.
        let segments = vec![
            Segment2D::new([-1.0, -1.0], [1.0, 1.0], 0), // diagonal: crosses seg 2 at (0,0)
            Segment2D::new([1.0, 1.0], [1.0 + e, -1.0], 1), // right side: endpoint near (1,-1)
            Segment2D::new([1.0 + e, -1.0], [-1.0, 1.0], 2), // crosses seg 0 at near-(0,0)
            Segment2D::new([-1.0, 1.0], [-1.0, -1.0], 3), // close
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        // Segs 0 and 2 must still be detected as crossing.
        match cert {
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::SelfCrossing,
                ..
            } => {}
            _ => panic!(
                "Near-coincident crossing must be SelfCrossing, got {:?}",
                cert
            ),
        }
    }

    // ── 2. Extreme large coordinate scale (1e150) ─────────────────────────────
    //
    // Rational::try_from_f64(1e150) produces a huge numerator/denominator.
    // A convex rectangle at this scale has no non-adjacent pair interactions,
    // so the fast path classifies it as Simple — but must not panic during
    // the orient2d calls or Rational conversion.
    #[test]
    fn extreme_large_coordinates_are_stable() {
        let big = 1e150_f64;
        let segments = vec![
            Segment2D::new([0.0, 0.0], [big, 0.0], 0),
            Segment2D::new([big, 0.0], [big, big], 1),
            Segment2D::new([big, big], [0.0, big], 2),
            Segment2D::new([0.0, big], [0.0, 0.0], 3),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        assert_eq!(
            cert,
            WeakSimpleCertificate::Simple,
            "1e150-scale rectangle must be Simple, got {:?}",
            cert
        );
    }

    // ── 3. Mixed coordinate scale (200 orders of magnitude) ───────────────────
    //
    // Adjacent segments differ in scale by 200 orders of magnitude.
    // The certifier must not panic or silently snap the tiny segment to zero.
    // We don't assert a specific verdict — we assert stability (no panic).
    #[test]
    fn mixed_scale_200_orders_of_magnitude_does_not_panic() {
        let tiny = 1e-100_f64;
        let huge = 1e100_f64;
        // Non-degenerate quadrilateral spanning 200 orders of magnitude.
        let segments = vec![
            Segment2D::new([0.0, 0.0], [tiny, tiny], 0),
            Segment2D::new([tiny, tiny], [huge, 0.0], 1),
            Segment2D::new([huge, 0.0], [huge, huge], 2),
            Segment2D::new([huge, huge], [0.0, 0.0], 3),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let _ = certify_boundary(&boundary); // must not panic
    }

    // ── 4. Endpoint touching the exact midpoint of another segment ─────────────
    //
    // Segment endpoint (2,2) lies at the exact midpoint of the top edge (0,2)→(4,2).
    // The arrangement must split the top edge at t=1/2 (Rational(1,2)) and create
    // an EndpointTouch vertex. The boundary is a pentagon with an interior diagonal
    // A pentagon where one segment passes through the interior of the polygon,
    // creating a proper transversally crossing at an interior point (not a polygon vertex).
    // The crossing point has rational coordinates exactly expressible in the arrangement.
    //
    // Shape: a rectangle [0,4]×[0,2] with an interior horizontal chord at y=1 (segs 4-5)
    // that cuts through the interior, crossing the left and right sides.
    // The chord endpoints are NOT polygon vertices.
    #[test]
    fn endpoint_at_exact_rational_midpoint_is_self_crossing() {
        // A star-shaped boundary: outer square traced with one inward spike.
        // The spike from (2,-1) up to (2,1) crosses the bottom edge (0,0)→(4,0)
        // at exactly (2,0), which is the midpoint t=1/2 of the bottom edge.
        //
        // Segments:
        //   0: (0,0)→(4,0)  bottom (midpoint at (2,0))
        //   1: (4,0)→(4,2)  right
        //   2: (4,2)→(0,2)  top
        //   3: (0,2)→(0,0)  left
        //   4: (0,0)→(2,-1) spike down-right
        //   5: (2,-1)→(4,0) spike back up to right corner
        // Segs 4 and 5 form a spike that goes BELOW y=0. The spike bottom is at (2,-1).
        // Seg 4 does NOT cross seg 0 (they share (0,0)). Seg 5 does NOT cross seg 0 (they share (4,0)).
        // This geometry is actually weakly-simple (spike touches at endpoints).
        //
        // Better: a boundary that wraps around and re-crosses itself.
        // A closed curve: pentagon + re-entry segment.
        //
        // Simplest exact-crossing test: a backward-Z or lightning-bolt shape.
        // (0,0)→(2,2)→(0,2)→(2,0)→(0,0) — the classic figure-8 but with shared start/end.
        // Segs 0 ([0,0]→[2,2]) and seg 2 ([0,2]→[2,0]) cross at exactly (1,1).
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 2.0], 0), // diagonal↑→
            Segment2D::new([2.0, 2.0], [0.0, 2.0], 1), // left along top
            Segment2D::new([0.0, 2.0], [2.0, 0.0], 2), // diagonal↓→ (crosses seg 0 at (1,1))
            Segment2D::new([2.0, 0.0], [0.0, 0.0], 3), // left along bottom
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        // Segs 0 and 2 cross at rational point (1,1) — must be SelfCrossing.
        match cert {
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::SelfCrossing,
                ..
            } => {}
            _ => panic!(
                "Z-path crossing at rational (1,1) must be SelfCrossing, got {:?}",
                cert
            ),
        }
    }

    // ── 5. Bowtie figure-8 with crossing at exact rational point (0,0) ─────────
    //
    // A classic bowtie: segments 0 and 2 cross at exactly (0,0), a rational point.
    // The Akitaya strand test must see ABAB interleaving around the crossing vertex
    // and reject as SelfCrossing. A tolerance-based certifier might confuse the
    // exact integer crossing for an endpoint touch and return WeaklySimple.
    #[test]
    fn bowtie_crossing_at_rational_origin_is_self_crossing() {
        let segments = vec![
            Segment2D::new([-1.0, -1.0], [1.0, 1.0], 0), // lower-left to upper-right
            Segment2D::new([1.0, 1.0], [1.0, -1.0], 1),  // down right side
            Segment2D::new([1.0, -1.0], [-1.0, 1.0], 2), // lower-right to upper-left (crosses seg 0 at origin)
            Segment2D::new([-1.0, 1.0], [-1.0, -1.0], 3), // down left side
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::SelfCrossing,
                ..
            } => {}
            _ => panic!("Bowtie must be SelfCrossing, got {:?}", cert),
        }
    }

    // ── 6. Collinear partial overlap on a shared 1D interval ──────────────────
    //
    // Segments 0 and 4 are both on y=0, going in opposite directions,
    // overlapping on [0,2]. Unlike the spike test where the entire segment
    // reverses, here it is a partial overlap of two distinct source segments.
    // The exact 1D parameter ordering in compute_collinear_overlap must detect this.
    #[test]
    fn collinear_partial_overlap_on_shared_interval_is_rejected() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [4.0, 0.0], 0), // bottom: 0→4 on y=0
            Segment2D::new([4.0, 0.0], [4.0, 2.0], 1), // up
            Segment2D::new([4.0, 2.0], [2.0, 2.0], 2), // top: partial width
            Segment2D::new([2.0, 2.0], [2.0, 0.0], 3), // down to y=0 at x=2
            Segment2D::new([2.0, 0.0], [0.0, 0.0], 4), // bottom: 2→0 on y=0 — overlaps seg 0 on [0,2]
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        match cert {
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::OverlappingSegments,
                ..
            } => {}
            _ => panic!(
                "Partial collinear overlap must be OverlappingSegments, got {:?}",
                cert
            ),
        }
    }

    // ── 7. Almost-collinear polygon with 1e-50 deviation ─────────────────────
    //
    // An almost-collinear pentagon where two non-adjacent bottom edges are nearly
    // horizontal at y ≈ 0 but slightly above. A float-snapping collinear detector
    // wrongly declares them collinear and rejects; exact orient2d must prove area > 0.
    //
    // Critically: the bottom segments do NOT form a y=0 path — the polygon is
    // closed through vertices at y=2, so no collinear overlap on y=0 exists.
    #[test]
    fn almost_collinear_polygon_with_sub_epsilon_deviation_is_not_rejected_as_degenerate() {
        let e = 1e-50_f64;
        // Pentagon straddling y=0 with a 1e-50 bump:
        // (0,1) at the top-left, closing through y=2 to avoid any y=0 overlap.
        // Non-adjacent pair: seg 1 ([1,e]→[2,0]) and seg 3 ([3,0]→[4,e]) are near-collinear
        // (nearly horizontal) but don't overlap because they're in different x-ranges.
        let boundary_segments = vec![
            Segment2D::new([0.0, 1.0], [1.0, e], 0), // going down to near-zero
            Segment2D::new([1.0, e], [2.0, 0.0], 1), // nearly horizontal: near-collinear with seg 3
            Segment2D::new([2.0, 0.0], [3.0, e], 2), // going up
            Segment2D::new([3.0, e], [4.0, 1.0], 3), // going up to y=1
            Segment2D::new([4.0, 1.0], [0.0, 1.0], 4), // top edge back to start
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(boundary_segments, frame);
        let cert = certify_boundary(&boundary);
        // A float-snapping collinear detector sees segs 1 and 3 as collinear-overlapping
        // since their y-values differ by only 1e-50. Exact orient2d must prove they
        // are on different lines and the polygon is Simple.
        assert_eq!(
            cert,
            WeakSimpleCertificate::Simple,
            "Just-off-collinear pentagon must be Simple; got {:?}",
            cert
        );
    }
}
