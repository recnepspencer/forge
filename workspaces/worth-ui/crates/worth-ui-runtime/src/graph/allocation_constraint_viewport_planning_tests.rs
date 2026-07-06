use worth_ui_inspection::UiEvidenceAuthorityGeneration;
use crate::evidence::projection_fact_test_support::{
    capability_report, display_field_projection_context, host_result_viewport_extent,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiConstraintPropagationDenialReason,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiViewportPlanningInputPosture,
};
use crate::facade::WorthUi;
use crate::graph::allocation_constraint_equal_share_test_support::viewport_basis_policy;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

#[test]
fn page_root_admits_viewport_as_typed_planning_input() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-viewport");
    let app = page_root_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(91);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-viewport"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(91),
        &viewport_basis_policy(false),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                910,
                &report,
                UiEvidenceAuthorityGeneration::new(91),
            )),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("page-root neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("page-root viewport planning should admit");
    let viewport_input = constraints
        .viewport_planning_input()
        .expect("page root should carry viewport planning artifact");

    assert_eq!(neighborhood.members().len(), 1);
    assert_eq!(viewport_input.edge_family(), UiConstraintPropagationEdgeFamily::ViewportInput);
    assert_eq!(
        viewport_input.posture(),
        UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly
    );
    assert!(viewport_input.is_planning_time_only());
    assert!(viewport_input.source_evidence_identity_digest().is_some());
    assert!(
        constraints.propagation_edges().iter().any(|edge| matches!(
            edge.payload(),
            UiConstraintPropagationEdgePayload::ViewportInput {
                posture: UiViewportPlanningInputPosture::AdmittedPlanningTimeOnly,
                planning_time_only: true,
                ..
            }
        ))
    );
}

#[test]
fn missing_viewport_evidence_denies_through_typed_viewport_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-viewport-missing");
    let app = page_root_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(92);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-viewport-missing"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(92),
        &viewport_basis_policy(false),
        &[MeasurementEvidenceInput::host_capability_report(&report)],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("page-root neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("missing viewport evidence should deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::MissingRequiredViewportPlanningInput
    );
    assert_eq!(denial.family(), Some(UiConstraintPropagationEdgeFamily::ViewportInput));
}

#[test]
fn stale_viewport_evidence_denies_as_incompatible_measurement_posture() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-viewport-stale");
    let app = page_root_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(93);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-viewport-stale"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(93),
        &viewport_basis_policy(false),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                930,
                &report,
                UiEvidenceAuthorityGeneration::new(92),
            )),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("page-root neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("stale viewport evidence should deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture
    );
    assert_eq!(denial.family(), Some(UiConstraintPropagationEdgeFamily::ViewportInput));
}

fn page_root_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .freeze()
}

fn graph_node_identity_for_provenance(app: &crate::facade::WorthUiApp) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| {
                    handoff.role() == crate::declaration::UiDeclarationStructuralRole::Page
                })
                .unwrap_or(false)
        })
        .expect("bootstrap root page artifact should exist");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("page declaration should project one graph node")
}
