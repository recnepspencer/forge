use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::projection_fact_test_support::{
    capability_report, display_field_projection_context, host_result_portal_anchor,
    host_result_scroll_container_viewport, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiBoundReconciliationPosture,
    UiConstraintPropagationEdgeFamily,
};
use crate::graph::allocation_constraint_projection_tests::{
    control_app, graph_node_identity_for_provenance,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn declared_bounded_clamp_is_explicit_on_ordinary_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-clamp");
    let app = control_app(world_profile.clone(), "operator:scroll");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let report = capability_report(91);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-bounds-clamp"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(91),
        &scroll_bound_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport(
                910,
                &report,
                UiEvidenceAuthorityGeneration::new(91),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("stack neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("bounded stack should admit explicit bound reconciliation");
    let bound = constraints
        .bound_reconciliation()
        .expect("bounded stack should mint a bound reconciliation artifact");

    assert_eq!(
        bound.posture(),
        UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp
    );
    assert!(
        constraints.propagation_edges().iter().any(|edge| {
            matches!(
                edge.payload(),
                crate::evidence::UiConstraintPropagationEdgePayload::BoundedReconciliation {
                    reconciliation_identity_digest,
                    posture: UiBoundReconciliationPosture::SatisfiedWithDeclaredClamp,
                    ..
                } if reconciliation_identity_digest == bound.identity_digest()
            )
        }),
        "bounded reconciliation edges must carry the solved clamp posture instead of a bare family tag"
    );
}

#[test]
fn stale_bound_inputs_remain_typed_on_ordinary_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-stale");
    let app = control_app(world_profile.clone(), "operator:scroll");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let report = capability_report(92);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-bounds-stale"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(92),
        &scroll_bound_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport(
                920,
                &report,
                UiEvidenceAuthorityGeneration::new(91),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("stack neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("stale bounded inputs should stay typed on the ordinary lane");

    assert_eq!(
        constraints
            .bound_reconciliation()
            .expect("bounded stack should still materialize typed stale posture")
            .posture(),
        UiBoundReconciliationPosture::StaleInput
    );
}

#[test]
fn portal_anchor_bounds_stay_typed_as_unsupported_special_input() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-portal");
    let app = control_app(world_profile.clone(), "operator:portal-anchor");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let report = capability_report(93);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-bounds-portal"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(93),
        &portal_anchor_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_portal_anchor(
                930,
                &report,
                UiEvidenceAuthorityGeneration::new(93),
            )),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("portal neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("portal bounded inputs should stay typed through bound reconciliation");

    assert_eq!(
        constraints
            .bound_reconciliation()
            .expect("portal anchor should mint a bound reconciliation artifact")
            .posture(),
        UiBoundReconciliationPosture::UnsupportedSpecialInput
    );
}

#[test]
fn equivalent_bounded_inputs_converge_on_same_bound_result() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-bounds-equivalence");
    let app = control_app(world_profile.clone(), "operator:scroll");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let report = capability_report(94);
    let build_constraints = || {
        let basis = admit_measurement_basis(
            synthetic_declaration_identity("allocation-constraint-bounds-equivalence"),
            root_node,
            world_profile.clone(),
            UiEvidenceAuthorityGeneration::new(94),
            &scroll_bound_policy(),
            &[
                MeasurementEvidenceInput::host_capability_report(&report),
                MeasurementEvidenceInput::host_measurement_result(
                    &host_result_scroll_container_viewport(
                    940,
                    &report,
                    UiEvidenceAuthorityGeneration::new(94),
                    ),
                ),
            ],
        );
        let neighborhood = basis
            .admit_allocation_neighborhood_from_graph(&snapshot)
            .expect("stack neighborhood should admit");
        basis
            .admit_allocation_constraint_set(&neighborhood)
            .expect("equivalent bounded inputs should admit")
    };

    let left = build_constraints();
    let right = build_constraints();

    assert_eq!(
        left.bound_reconciliation()
            .expect("left should carry bound reconciliation")
            .identity_digest(),
        right
            .bound_reconciliation()
            .expect("right should carry bound reconciliation")
            .identity_digest()
    );
    assert_eq!(
        left.propagation_edges()
            .iter()
            .filter(|edge| edge.family() == UiConstraintPropagationEdgeFamily::BoundedReconciliation)
            .collect::<Vec<_>>(),
        right
            .propagation_edges()
            .iter()
            .filter(|edge| edge.family() == UiConstraintPropagationEdgeFamily::BoundedReconciliation)
            .collect::<Vec<_>>()
    );
}

fn portal_anchor_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
        Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired),
        vec![],
    )
    .expect("portal anchor policy should admit")
}

fn scroll_bound_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![],
    )
    .expect("scroll bound policy should admit")
}
