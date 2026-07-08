use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_scroll_container_viewport,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiConstraintPropagationDenialReason,
    UiConstraintPropagationEdgeFamily, UiConstraintPropagationEdgePayload,
    UiScrollOwnerPlanningInputPosture,
};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

#[test]
fn scroll_owner_basis_admits_as_typed_planning_input() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-scroll");
    let app = scroll_owner_app(world_profile.clone());
    let root_node = graph_node_identity_for_scroll_owner(&app);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(101);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-scroll"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(101),
        &scroll_owner_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_scroll_container_viewport(
                1010,
                &report,
                UiEvidenceAuthorityGeneration::new(101),
            )),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("scroll-owner neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("scroll-owner planning should admit");
    let scroll_input = constraints
        .scroll_owner_planning_input()
        .expect("scroll owner should carry scroll planning artifact");

    assert_eq!(scroll_input.edge_family(), UiConstraintPropagationEdgeFamily::ScrollViewportInput);
    assert_eq!(
        scroll_input.posture(),
        UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly
    );
    assert!(scroll_input.is_planning_time_only());
    assert!(scroll_input.source_evidence_identity_digest().is_some());
    assert!(constraints.propagation_edges().iter().any(|edge| matches!(
        edge.payload(),
        UiConstraintPropagationEdgePayload::ScrollViewportInput {
            posture: UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly,
            planning_time_only: true,
            ..
        }
    )));
}

#[test]
fn missing_scroll_owner_basis_denies_through_typed_scroll_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-scroll-missing");
    let app = scroll_owner_app(world_profile.clone());
    let root_node = graph_node_identity_for_scroll_owner(&app);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(102);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-scroll-missing"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(102),
        &scroll_owner_policy(),
        &[MeasurementEvidenceInput::host_capability_report(&report)],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("scroll-owner neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("missing scroll-owner basis should deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::MissingRequiredScrollOwnerPlanningInput
    );
    assert_eq!(denial.family(), Some(UiConstraintPropagationEdgeFamily::ScrollViewportInput));
}

#[test]
fn stale_scroll_owner_basis_denies_as_incompatible_measurement_posture() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-scroll-stale");
    let app = scroll_owner_app(world_profile.clone());
    let root_node = graph_node_identity_for_scroll_owner(&app);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let report = capability_report(103);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-scroll-stale"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(103),
        &scroll_owner_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_scroll_container_viewport(
                1030,
                &report,
                UiEvidenceAuthorityGeneration::new(102),
            )),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("scroll-owner neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("stale scroll-owner basis should deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture
    );
    assert_eq!(denial.family(), Some(UiConstraintPropagationEdgeFamily::ScrollViewportInput));
}

fn scroll_owner_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![],
    )
    .expect("scroll-owner policy should admit")
}

fn scroll_owner_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.graph.allocation-constraint-scroll")
                .with_semantic_artifact_spec(scroll_owner_spec()),
        )
        .freeze()
}

fn scroll_owner_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.scroll.owner"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/allocation_constraint_scroll_owner_tests.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new("slot:body"))
    .with_structural_token(UiDslStructuralToken::new("operator:scroll"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
}

fn graph_node_identity_for_scroll_owner(app: &crate::facade::WorthUiApp) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_constraint_scroll_owner_tests.wui"
                && provenance.declaration_index() == 0
        })
        .expect("scroll-owner artifact should exist");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("scroll-owner artifact should project one graph node")
}
