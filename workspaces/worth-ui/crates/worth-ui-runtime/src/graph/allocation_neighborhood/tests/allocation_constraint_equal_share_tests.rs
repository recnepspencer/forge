use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_text_intrinsic_size,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiAllocationNeighborhood,
    UiConstraintEqualSharePosture, UiConstraintEqualShareSolveOrder,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdgePayload,
};
use crate::graph::allocation_constraint_equal_share_test_support::{
    graph_node_identity_for_provenance, intrinsic_policy, open_policy, peer_app, three_peer_app,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn equal_share_is_explicit_and_records_solve_order() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-equal-share");
    let app = peer_app(
        world_profile.clone(),
        "operator:grid",
        &[false, false, false],
    );
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(81);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-equal-share"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(81),
        &open_policy(),
        &[MeasurementEvidenceInput::host_capability_report(&report)],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("grid neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("grid equal-share constraints should admit");
    let equal_share = constraints
        .equal_share_distribution()
        .expect("grid peers should emit explicit equal-share distribution");

    assert_eq!(
        equal_share.solve_order(),
        UiConstraintEqualShareSolveOrder::AfterSiblingNegotiationBeforeBounds
    );
    assert_eq!(
        equal_share.posture(),
        UiConstraintEqualSharePosture::DeterministicRemainderApplied
    );
    assert_eq!(equal_share.members().len(), 2);
    assert_eq!(
        equal_share
            .members()
            .iter()
            .map(|member| member.remainder_rank())
            .collect::<Vec<_>>(),
        vec![Some(0), Some(1)]
    );

    let equal_share_edges = constraints
        .propagation_edges()
        .iter()
        .filter(|edge| {
            edge.family()
                == crate::evidence::UiConstraintPropagationEdgeFamily::EqualShareDistribution
        })
        .collect::<Vec<_>>();
    assert_eq!(equal_share_edges.len(), 2);
    match equal_share_edges[0].payload() {
        UiConstraintPropagationEdgePayload::EqualShareDistribution {
            group_identity_digest,
            distribution_identity_digest,
            solve_order,
            posture,
            ..
        } => {
            assert_eq!(group_identity_digest, equal_share.group_identity_digest());
            assert_eq!(distribution_identity_digest, equal_share.identity_digest());
            assert_eq!(solve_order, equal_share.solve_order());
            assert_eq!(posture, equal_share.posture());
        }
        other => panic!("unexpected equal-share payload: {other:?}"),
    }
}

#[test]
fn equivalent_peer_reorder_converges_on_the_same_equal_share_result() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-equal-share-reorder");
    let app = peer_app(
        world_profile.clone(),
        "operator:grid",
        &[false, false, false],
    );
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(82);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-equal-share-reorder"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(82),
        &open_policy(),
        &[MeasurementEvidenceInput::host_capability_report(&report)],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("grid neighborhood should admit");
    let reordered = neighborhood
        .with_members_for_graph_test(neighborhood.members().iter().rev().cloned().collect());

    let left = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("canonical equal-share should admit");
    let right = basis
        .admit_allocation_constraint_set(&reordered)
        .expect("reordered equal-share should admit");

    assert_eq!(
        left.equal_share_distribution()
            .expect("left equal-share distribution")
            .identity_digest(),
        right
            .equal_share_distribution()
            .expect("right equal-share distribution")
            .identity_digest()
    );
}

#[test]
fn bounded_split_peers_deny_before_bounded_reconciliation() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-equal-share-denial");
    let app = three_peer_app(
        world_profile.clone(),
        "operator:split",
        &[true, true, true, true],
    );
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let peer_c = graph_node_identity_for_provenance(&app, 3);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b, peer_c]);
    let report = capability_report(83);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-equal-share-denial"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(83),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    831,
                    &report,
                    UiEvidenceAuthorityGeneration::new(83),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_b,
                &host_result_text_intrinsic_size(
                    832,
                    &report,
                    UiEvidenceAuthorityGeneration::new(83),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_c,
                &host_result_text_intrinsic_size(
                    833,
                    &report,
                    UiEvidenceAuthorityGeneration::new(83),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("split neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("bounded split peers should deny before bounds");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint
    );
    assert_eq!(
        denial.family(),
        Some(crate::evidence::UiConstraintPropagationEdgeFamily::SiblingNegotiation)
    );
}

#[test]
fn zero_share_and_single_survivor_resolve_through_typed_posture() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-equal-share-posture");
    let app = peer_app(
        world_profile.clone(),
        "operator:grid",
        &[false, false, false],
    );
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(84);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-equal-share-posture"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(84),
        &open_policy(),
        &[MeasurementEvidenceInput::host_capability_report(&report)],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("grid neighborhood should admit");
    let root_member = neighborhood
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Root
            )
        })
        .expect("grid neighborhood should preserve a root member")
        .clone();
    let first_peer = neighborhood
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Peer
            )
        })
        .expect("grid neighborhood should preserve a peer member")
        .clone();
    let single_survivor = UiAllocationNeighborhood::new_for_graph_test(
        neighborhood.root_graph_node_identity(),
        neighborhood.graph_generation(),
        neighborhood.world_identity_digest(),
        neighborhood.measurement_basis_identity_digest(),
        neighborhood.layout_operator_planning_contract().clone(),
        neighborhood.dependency_map().clone(),
        neighborhood.neighborhood_class(),
        neighborhood.membership_rule(),
        vec![root_member.clone(), first_peer],
        &super::super::UiAllocationNeighborhoodMintAuthority::mint(),
    );
    let zero_share = UiAllocationNeighborhood::new_for_graph_test(
        neighborhood.root_graph_node_identity(),
        neighborhood.graph_generation(),
        neighborhood.world_identity_digest(),
        neighborhood.measurement_basis_identity_digest(),
        neighborhood.layout_operator_planning_contract().clone(),
        neighborhood.dependency_map().clone(),
        neighborhood.neighborhood_class(),
        neighborhood.membership_rule(),
        vec![root_member],
        &super::super::UiAllocationNeighborhoodMintAuthority::mint(),
    );

    let single_constraints = basis
        .admit_allocation_constraint_set(&single_survivor)
        .expect("single-survivor equal-share should admit");
    let zero_constraints = basis
        .admit_allocation_constraint_set(&zero_share)
        .expect("zero-share equal-share should admit");

    assert_eq!(
        single_constraints
            .equal_share_distribution()
            .expect("single survivor should still carry equal-share posture")
            .posture(),
        UiConstraintEqualSharePosture::SingleSurvivingPeer
    );
    assert_eq!(
        zero_constraints
            .equal_share_distribution()
            .expect("zero share should still carry equal-share posture")
            .posture(),
        UiConstraintEqualSharePosture::ZeroShare
    );
}
