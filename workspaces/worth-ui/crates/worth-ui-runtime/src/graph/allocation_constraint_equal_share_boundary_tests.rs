use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::projection_fact_test_support::{
    capability_report, host_result_text_intrinsic_size, host_result_viewport_extent_with_value,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiConstraintEqualShareDistributionPolicy,
    UiConstraintEqualSharePosture, UiConstraintPropagationDenialReason,
};
use crate::graph::allocation_constraint_equal_share_test_support::{
    graph_node_identity_for_provenance, peer_app, three_peer_app, viewport_basis_policy,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn zero_available_space_reaches_typed_equal_share_posture_on_ordinary_lane() {
    let (_, _, world_profile) =
        crate::evidence::projection_fact_test_support::display_field_projection_context(
            "allocation-constraint-equal-share-zero-space",
        );
    let app = peer_app(world_profile.clone(), "operator:grid", &[false, false, false]);
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(85);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-equal-share-zero-space"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(85),
        &viewport_basis_policy(false),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_viewport_extent_with_value(
                    850,
                    &report,
                    UiEvidenceAuthorityGeneration::new(85),
                    0.0,
                    60.0,
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("grid neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("zero-space equal-share should admit");

    assert_eq!(
        constraints
            .equal_share_distribution()
            .expect("grid peers should still carry equal-share posture")
            .posture(),
        UiConstraintEqualSharePosture::ZeroAvailableSpace
    );
}

#[test]
fn ordinary_lane_proves_remainder_policy_and_non_integral_denial_from_admitted_space() {
    let (_, _, world_profile) =
        crate::evidence::projection_fact_test_support::display_field_projection_context(
            "allocation-constraint-equal-share-remainder",
        );

    let grid_app = peer_app(world_profile.clone(), "operator:grid", &[false, false, false]);
    let grid_root = graph_node_identity_for_provenance(&grid_app, 0);
    let grid_peer_a = graph_node_identity_for_provenance(&grid_app, 1);
    let grid_peer_b = graph_node_identity_for_provenance(&grid_app, 2);
    let grid_snapshot =
        snapshot_with_admitted_layout(&grid_app, &[grid_root, grid_peer_a, grid_peer_b]);
    let grid_report = capability_report(86);
    let grid_basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-equal-share-remainder"),
        grid_root,
        world_profile.clone(),
        UiEvidenceAuthorityGeneration::new(86),
        &viewport_basis_policy(false),
        &[
            MeasurementEvidenceInput::host_capability_report(&grid_report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_viewport_extent_with_value(
                    860,
                    &grid_report,
                    UiEvidenceAuthorityGeneration::new(86),
                    5.0,
                    3.0,
                ),
            ),
        ],
    );
    let grid_neighborhood = grid_basis
        .admit_allocation_neighborhood_from_graph(&grid_snapshot)
        .expect("grid neighborhood should admit");
    let grid_constraints = grid_basis
        .admit_allocation_constraint_set(&grid_neighborhood)
        .expect("grid remainder case should admit");
    let grid_equal_share = grid_constraints
        .equal_share_distribution()
        .expect("grid peers should admit equal-share distribution");

    assert_eq!(
        grid_equal_share.policy(),
        UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity
    );
    assert_eq!(
        grid_equal_share.posture(),
        UiConstraintEqualSharePosture::DeterministicRemainderApplied
    );
    assert_eq!(
        grid_equal_share
            .members()
            .iter()
            .map(|member| member.remainder_rank())
            .collect::<Vec<_>>(),
        vec![Some(0), Some(1)]
    );

    let split_app = three_peer_app(world_profile.clone(), "operator:split", &[true, true, true, true]);
    let split_root = graph_node_identity_for_provenance(&split_app, 0);
    let split_peer_a = graph_node_identity_for_provenance(&split_app, 1);
    let split_peer_b = graph_node_identity_for_provenance(&split_app, 2);
    let split_peer_c = graph_node_identity_for_provenance(&split_app, 3);
    let split_snapshot = snapshot_with_admitted_layout(
        &split_app,
        &[split_root, split_peer_a, split_peer_b, split_peer_c],
    );
    let split_report = capability_report(87);
    let split_basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-equal-share-non-integral-denial"),
        split_root,
        world_profile,
        UiEvidenceAuthorityGeneration::new(87),
        &viewport_basis_policy(true),
        &[
            MeasurementEvidenceInput::host_capability_report(&split_report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_viewport_extent_with_value(
                    870,
                    &split_report,
                    UiEvidenceAuthorityGeneration::new(87),
                    100.0,
                    10.0,
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                split_peer_a,
                &host_result_text_intrinsic_size(
                    871,
                    &split_report,
                    UiEvidenceAuthorityGeneration::new(87),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                split_peer_b,
                &host_result_text_intrinsic_size(
                    872,
                    &split_report,
                    UiEvidenceAuthorityGeneration::new(87),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                split_peer_c,
                &host_result_text_intrinsic_size(
                    873,
                    &split_report,
                    UiEvidenceAuthorityGeneration::new(87),
                ),
            ),
        ],
    );
    let split_neighborhood = split_basis
        .admit_allocation_neighborhood_from_graph(&split_snapshot)
        .expect("split neighborhood should admit");
    let denial = split_basis
        .admit_allocation_constraint_set(&split_neighborhood)
        .expect_err("non-integral split case should deny before bounds");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::ContradictoryEqualShareRequirements
    );
}
