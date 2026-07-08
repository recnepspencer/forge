use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_scroll_container_viewport,
    host_result_scroll_container_viewport_with_value, host_result_text_intrinsic_size,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiBoundReconciliationPosture,
    UiConstraintPropagationEdgePayload,
};
use crate::graph::allocation_constraint_bound_reconciliation_test_support::{
    bounded_policy, graph_node_identity_for_provenance, peer_app, scroll_bound_policy,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn root_only_bounded_neighborhood_stays_underconstrained_on_ordinary_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-underconstrained");
    let app = peer_app(world_profile.clone(), "operator:stack", &[true, true, false]);
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a]);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-bounds-underconstrained"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(101),
        &bounded_policy(),
        &[],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("bounded neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("bounded neighborhood should preserve typed underconstrained posture");

    assert_eq!(
        constraints
            .bound_reconciliation()
            .expect("bounded reconciliation artifact should still materialize")
            .posture(),
        UiBoundReconciliationPosture::Underconstrained
    );
}

#[test]
fn mixed_bounded_peer_participation_stays_contradictory_on_ordinary_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-contradiction");
    let app = peer_app(world_profile.clone(), "operator:scroll", &[true, true, false]);
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(102);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-bounds-contradiction"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(102),
        &scroll_bound_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport(
                    1020,
                    &report,
                    UiEvidenceAuthorityGeneration::new(102),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("mixed bounded neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("mixed bounded neighborhood should preserve contradiction as typed posture");
    let bound = constraints
        .bound_reconciliation()
        .expect("mixed bounded neighborhood should mint a bound reconciliation artifact");

    assert_eq!(bound.posture(), UiBoundReconciliationPosture::ContradictoryMinMax);
    assert!(constraints.propagation_edges().iter().any(|edge| {
        matches!(
            edge.payload(),
            UiConstraintPropagationEdgePayload::BoundedReconciliation {
                reconciliation_identity_digest,
                posture: UiBoundReconciliationPosture::ContradictoryMinMax,
                ..
            } if reconciliation_identity_digest == bound.identity_digest()
        )
    }));
}

#[test]
fn zero_space_bounded_neighborhood_stays_overconstrained_on_ordinary_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-overconstrained");
    let app = peer_app(world_profile.clone(), "operator:scroll", &[true, true, true]);
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(103);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-bounds-overconstrained"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(103),
        &scroll_bound_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport_with_value(
                    1030,
                    &report,
                    UiEvidenceAuthorityGeneration::new(103),
                    120.0,
                    0.0,
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("zero-space bounded neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("zero-space bounded neighborhood should stay typed on the ordinary lane");

    assert_eq!(
        constraints
            .bound_reconciliation()
            .expect("zero-space bounded neighborhood should mint a bound reconciliation artifact")
            .posture(),
        UiBoundReconciliationPosture::Overconstrained
    );
}

#[test]
fn final_bound_artifact_reclassifies_to_cyclic_on_ordinary_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-cyclic");
    let app = peer_app(world_profile.clone(), "operator:stack", &[true, true, false]);
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a]);
    let report = capability_report(104);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-bounds-cyclic"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(104),
        &bounded_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    1041,
                    &report,
                    UiEvidenceAuthorityGeneration::new(104),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("cyclic bounded neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("cyclic bounded neighborhood should admit with typed cycle posture");
    let bound = constraints
        .bound_reconciliation()
        .expect("cyclic bounded neighborhood should still carry a bound reconciliation artifact");

    assert_eq!(bound.posture(), UiBoundReconciliationPosture::Cyclic);
    assert!(constraints.propagation_edges().iter().any(|edge| {
        matches!(
            edge.payload(),
            UiConstraintPropagationEdgePayload::BoundedReconciliation {
                reconciliation_identity_digest,
                posture: UiBoundReconciliationPosture::Cyclic,
                ..
            } if reconciliation_identity_digest == bound.identity_digest()
        )
    }));
}
