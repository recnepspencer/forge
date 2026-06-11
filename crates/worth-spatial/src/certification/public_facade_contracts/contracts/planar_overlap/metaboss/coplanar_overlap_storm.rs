use super::diagnostics::{assert_tiny_rotation_diagnostic, certify_tiny_rotation_diagnostic};
use super::outcome_matrix::assert_mb_m6_outcome_matrix;
use super::proof::{
    certify_storm, certify_storm_reversed_host_order, certify_storm_with_retained_replay,
    deny_tiny_rotation,
};
use super::scenario::{
    coplanar_equivalence_regions, coplanar_storm_regions, near_graze_region, StormTransform,
};

#[test]
fn mb_m6_1_coplanar_overlap_storm_end_to_end_receipts() {
    let regions = coplanar_storm_regions();
    let identity = certify_storm_with_retained_replay(
        "mb-real-coplanar-storm-identity",
        StormTransform::Identity,
        &regions,
    );
    assert_eq!(identity.signature.face_count, 216);
    assert_eq!(identity.signature.region_count, 12);
    assert!(identity.signature.partial_flush_regions >= 24);
    assert!(identity.signature.nested_hole_regions >= 24);
    assert!(identity.signature.boundary_touch_regions >= 24);
    assert!(identity.signature.collinear_run_regions >= 24);
    assert!(identity.signature.shared_intervals >= 54);
    assert!(identity.signature.ambiguous_contacts >= 27);
    assert!(identity.signature.containment_relations >= 27);
    assert_eq!(identity.signature.policy_required_exits, 0);

    assert!(!identity.retained_replay_digest.is_empty());

    for region in &identity.regions {
        assert_eq!(region.live_fact_digest, region.retained_replay_fact_digest);
        assert_eq!(
            region.projection_basis_digest,
            region.retained_projection_basis_digest
        );
        assert!(
            region.candidate_pair_breadth <= identity.max_candidate_pair_breadth,
            "region {} exceeded declared local breadth",
            region.region_identity
        );
    }
    assert!(
        identity.max_candidate_pair_breadth <= 220,
        "storm extraction must stay bounded per affected region, not scan the whole workload"
    );

    let tiny_rotation_denial =
        deny_tiny_rotation("mb-real-coplanar-storm-tiny-rotation", &near_graze_region());
    assert_eq!(
        tiny_rotation_denial.reason(),
        "movement and rotation posture must match before coplanar overlap extraction"
    );
    let diagnostic = certify_tiny_rotation_diagnostic(tiny_rotation_denial.reason());
    assert_tiny_rotation_diagnostic(&diagnostic, tiny_rotation_denial.reason());
}

#[test]
fn mb_m6_1_equivalent_motion_subset_converges_without_full_storm_replay() {
    let regions = coplanar_equivalence_regions();
    let identity = certify_storm(
        "mb-real-coplanar-subset-identity",
        StormTransform::Identity,
        &regions,
    );
    let translated = certify_storm(
        "mb-real-coplanar-subset-translated",
        StormTransform::Translated,
        &regions,
    );
    let half_turn = certify_storm(
        "mb-real-coplanar-subset-half-turn",
        StormTransform::HalfTurn,
        &regions,
    );
    let move_then_rotate = certify_storm(
        "mb-real-coplanar-subset-move-rotate",
        StormTransform::MoveThenRotate,
        &regions,
    );
    let rotate_then_move = certify_storm(
        "mb-real-coplanar-subset-rotate-move",
        StormTransform::RotateThenMove,
        &regions,
    );
    let reversed_host_order = certify_storm_reversed_host_order(
        "mb-real-coplanar-subset-reversed",
        StormTransform::Identity,
        &regions,
    );

    assert_eq!(identity.signature, translated.signature);
    assert_eq!(identity.signature, half_turn.signature);
    assert_eq!(identity.signature, move_then_rotate.signature);
    assert_eq!(identity.signature, rotate_then_move.signature);
    assert_eq!(identity.signature, reversed_host_order.signature);
    assert_eq!(identity.structural_digest, translated.structural_digest);
    assert_eq!(identity.structural_digest, half_turn.structural_digest);
    assert_eq!(
        move_then_rotate.structural_digest,
        rotate_then_move.structural_digest
    );
    assert_eq!(
        identity.structural_digest,
        reversed_host_order.structural_digest
    );
}

#[test]
fn mb_m6_1_user_outcome_matrix_branches_every_stop() {
    let tiny_rotation_denial = deny_tiny_rotation(
        "mb-real-coplanar-matrix-tiny-rotation",
        &near_graze_region(),
    );
    assert_mb_m6_outcome_matrix(&tiny_rotation_denial);
}
