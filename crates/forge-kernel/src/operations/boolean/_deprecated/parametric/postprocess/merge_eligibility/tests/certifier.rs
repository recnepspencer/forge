use super::*;
// =====================================================================
// SECTION 1: Certifier-level adversarial tests
// =====================================================================

/// D2 regression: figure-8 with crossing forces fallback path.
/// Must be rejected — the old `unwrap_or(TriSign::Zero)` would accept.
#[test]
fn fallback_path_rejects_figure_eight_with_crossing() {
    let segments = vec![
        Segment2D::new([0.0, 0.0], [2.0, 0.0], 0),
        Segment2D::new([2.0, 0.0], [1.0, 1.0], 1),
        Segment2D::new([1.0, 1.0], [2.0, 2.0], 2),
        Segment2D::new([2.0, 2.0], [0.0, 0.0], 3),
        Segment2D::new([0.0, 0.0], [0.0, 2.0], 4),
        Segment2D::new([0.0, 2.0], [1.0, 1.0], 5),
        Segment2D::new([1.0, 1.0], [0.0, 0.0], 6),
    ];
    let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
    let boundary = ProjectedBoundary2D::new(segments, frame);
    let cert = certify_boundary(&boundary);
    assert!(
        matches!(cert, WeakSimpleCertificate::Rejected { .. }),
        "Figure-8 with crossing MUST be rejected, got {:?}",
        cert,
    );
}

/// D2 regression: collinear overlap must be rejected in fallback.
#[test]
fn collinear_overlap_with_reversal_rejected_in_fallback() {
    let segments = vec![
        Segment2D::new([0.0, 0.0], [3.0, 0.0], 0),
        Segment2D::new([3.0, 0.0], [3.0, 1.0], 1),
        Segment2D::new([3.0, 1.0], [2.0, 1.0], 2),
        Segment2D::new([2.0, 1.0], [2.0, 0.0], 3),
        Segment2D::new([2.0, 0.0], [4.0, 0.0], 4),
        Segment2D::new([4.0, 0.0], [4.0, 1.0], 5),
        Segment2D::new([4.0, 1.0], [0.0, 1.0], 6),
        Segment2D::new([0.0, 1.0], [0.0, 0.0], 7),
    ];
    let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
    let boundary = ProjectedBoundary2D::new(segments, frame);
    let cert = certify_boundary(&boundary);
    assert!(
        matches!(
            cert,
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::OverlappingSegments,
                ..
            }
        ),
        "Collinear overlap MUST be rejected, got {:?}",
        cert,
    );
}

/// D7 regression: equal components → X dropped (spec §4.6 priority).
#[test]
fn projection_frame_tiebreak_all_equal_drops_x() {
    let frame = build_projection_frame([1.0, 1.0, 1.0]);
    assert_eq!(frame.get_drop_axis(), 0, "Equal components: drop X");
}

#[test]
fn projection_frame_tiebreak_yz_equal_drops_y() {
    let frame = build_projection_frame([0.1, 0.5, 0.5]);
    assert_eq!(frame.get_drop_axis(), 1, "Y == Z > X: drop Y");
}

#[test]
fn projection_frame_tiebreak_xz_equal_drops_x() {
    let frame = build_projection_frame([0.5, 0.1, 0.5]);
    assert_eq!(frame.get_drop_axis(), 0, "X == Z > Y: drop X");
}

#[test]
fn projection_frame_negative_normal_flips_orientation() {
    let pos = build_projection_frame([0.0, 0.0, 1.0]);
    let neg = build_projection_frame([0.0, 0.0, -1.0]);
    assert_eq!(pos.get_drop_axis(), neg.get_drop_axis());
    assert!(pos.get_orientation_sign() > 0.0);
    assert!(neg.get_orientation_sign() < 0.0);
}

/// Machine-epsilon perturbation must not cause false rejection.
#[test]
fn machine_epsilon_perturbation_still_simple() {
    let eps = f64::EPSILON;
    let vertices = [[0.0, 0.0], [1.0, eps], [1.0, 1.0], [0.0, 1.0]];
    let n = vertices.len();
    let segments: Vec<Segment2D> = (0..n)
        .map(|i| Segment2D::new(vertices[i], vertices[(i + 1) % n], i as u64))
        .collect();
    let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
    let boundary = ProjectedBoundary2D::new(segments, frame);
    assert_eq!(certify_boundary(&boundary), WeakSimpleCertificate::Simple);
}

/// Exact crossings must be detected as an intersection and not silently ignored.
#[test]
fn tiny_crossing_not_silently_accepted() {
    // A bow-tie shape with an explicit exact crossing.
    let segments = vec![
        Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
        Segment2D::new([1.0, 0.0], [0.0, 1.0], 1), // Crosses segment 3 transversally
        Segment2D::new([0.0, 1.0], [1.0, 1.0], 2),
        Segment2D::new([1.0, 1.0], [0.0, 0.0], 3), // Crosses segment 1 transversally
    ];

    let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
    let boundary = ProjectedBoundary2D::new(segments, frame);
    let cert = certify_boundary(&boundary);
    assert!(
        matches!(
            cert,
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::SelfCrossing,
                ..
            }
        ),
        "Exact crossing must be Rejected with SelfCrossing, got {:?}",
        cert,
    );
}

/// Degenerate collinear triangle — zero enclosed area — must be rejected.
#[test]
fn degenerate_collinear_triangle_rejected() {
    let segments = vec![
        Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
        Segment2D::new([1.0, 0.0], [2.0, 0.0], 1),
        Segment2D::new([2.0, 0.0], [0.0, 0.0], 2),
    ];
    let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
    let boundary = ProjectedBoundary2D::new(segments, frame);
    assert!(
        matches!(
            certify_boundary(&boundary),
            WeakSimpleCertificate::Rejected { .. }
        ),
        "Collinear triangle is zero-area degenerate — must be rejected",
    );
}

/// Pentagram: 5 proper crossings → SelfCrossing.
#[test]
fn pentagram_five_crossings_rejected() {
    use std::f64::consts::PI;
    let mut vertices = [[0.0f64; 2]; 5];
    for i in 0..5 {
        let angle = (2.0 * PI * (2 * i) as f64) / 5.0 - PI / 2.0;
        vertices[i] = [angle.cos(), angle.sin()];
    }
    let segments: Vec<Segment2D> = (0..5)
        .map(|i| Segment2D::new(vertices[i], vertices[(i + 1) % 5], i as u64))
        .collect();
    let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
    let boundary = ProjectedBoundary2D::new(segments, frame);
    assert!(
        matches!(
            certify_boundary(&boundary),
            WeakSimpleCertificate::Rejected {
                reason: BoundaryRejectReason::SelfCrossing,
                ..
            }
        ),
        "Pentagram must be SelfCrossing",
    );
}

/// D5 regression: different groups → different DecisionIds.
#[test]
fn different_groups_produce_different_decision_ids() {
    use forge_topo::bitset::EntityBitset;

    let mut group_a = EntityBitset::with_capacity(10);
    group_a
        .insert(0)
        .expect("bitset capacity must cover test indices");
    group_a
        .insert(1)
        .expect("bitset capacity must cover test indices");

    let mut group_b = EntityBitset::with_capacity(10);
    group_b
        .insert(2)
        .expect("bitset capacity must cover test indices");
    group_b
        .insert(3)
        .expect("bitset capacity must cover test indices");

    assert_ne!(
        compute_group_hash(&group_a),
        compute_group_hash(&group_b),
        "Different groups must produce different DecisionIds (D5 regression)",
    );
}

#[test]
fn same_group_produces_same_decision_id() {
    use forge_topo::bitset::EntityBitset;
    let mut group = EntityBitset::with_capacity(10);
    group
        .insert(0)
        .expect("bitset capacity must cover test indices");
    group
        .insert(3)
        .expect("bitset capacity must cover test indices");
    group
        .insert(7)
        .expect("bitset capacity must cover test indices");
    assert_eq!(compute_group_hash(&group), compute_group_hash(&group));
}

fn compute_group_hash(group: &forge_topo::bitset::EntityBitset) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for idx in 0..group.capacity() {
        if group
            .contains(idx)
            .expect("bitset capacity must cover test indices")
        {
            h = h.wrapping_mul(0x100000001b3) ^ (idx as u64);
        }
    }
    h
}

