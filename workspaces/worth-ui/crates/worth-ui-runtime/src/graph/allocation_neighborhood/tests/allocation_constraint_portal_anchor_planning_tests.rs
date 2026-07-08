use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_portal_anchor,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiConstraintPropagationDenialReason,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiPortalAnchorPlanningInputPosture,
};
use crate::graph::allocation_constraint_projection_tests::{
    control_app, graph_node_identity_for_provenance,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn portal_anchor_basis_admits_as_typed_planning_input() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-portal");
    let app = control_app(world_profile.clone(), "operator:portal-anchor");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(111);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-portal"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(111),
        &portal_anchor_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_portal_anchor(
                1110,
                &report,
                UiEvidenceAuthorityGeneration::new(111),
            )),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("portal-anchor neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("portal-anchor planning should admit");
    let portal_input = constraints
        .portal_anchor_planning_input()
        .expect("portal anchor should carry planning artifact");

    assert_eq!(portal_input.edge_family(), UiConstraintPropagationEdgeFamily::PortalAnchorInput);
    assert_eq!(
        portal_input.posture(),
        UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly
    );
    assert!(portal_input.is_planning_time_only());
    assert!(portal_input.source_evidence_identity_digest().is_some());
    assert!(constraints.propagation_edges().iter().any(|edge| matches!(
        edge.payload(),
        UiConstraintPropagationEdgePayload::PortalAnchorInput {
            posture: UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly,
            planning_time_only: true,
            ..
        }
    )));
}

#[test]
fn missing_portal_anchor_evidence_denies_through_typed_portal_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-portal-missing");
    let app = control_app(world_profile.clone(), "operator:portal-anchor");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(112);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-portal-missing"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(112),
        &portal_anchor_policy(),
        &[MeasurementEvidenceInput::host_capability_report(&report)],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("portal-anchor neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("missing portal-anchor evidence should deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::MissingRequiredPortalAnchorPlanningInput
    );
    assert_eq!(denial.family(), Some(UiConstraintPropagationEdgeFamily::PortalAnchorInput));
}

#[test]
fn stale_portal_anchor_evidence_denies_as_incompatible_measurement_posture() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-portal-stale");
    let app = control_app(world_profile.clone(), "operator:portal-anchor");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(113);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-portal-stale"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(113),
        &portal_anchor_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_portal_anchor(
                1130,
                &report,
                UiEvidenceAuthorityGeneration::new(112),
            )),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("portal-anchor neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("stale portal-anchor evidence should deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture
    );
    assert_eq!(denial.family(), Some(UiConstraintPropagationEdgeFamily::PortalAnchorInput));
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
