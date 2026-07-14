use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_host_contract::WorthUiHostCapabilityReport;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_font_metrics,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiConstraintAvailableSpacePosture,
    UiConstraintBoundedMinMaxRequirement, UiConstraintNormalizationPosture,
    UiConstraintParentAvailableSpace, UiConstraintPropagationDenialReason,
    UiConstraintPropagationEdgeFamily, UiMeasurementResult,
};
use crate::facade::WorthUi;
use crate::graph::allocation_constraint_projection_tests::{
    control_app, graph_node_identity_for_provenance,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::UiGraphWorldProfile;

#[test]
fn contradictory_downward_normalization_denies_before_other_flows() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-contradict");
    let app = control_app(world_profile.clone(), "operator:scroll");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let policy = downward_contradiction_policy();
    let capability_report = capability_report(88);
    let font_metrics = host_result_font_metrics(
        400,
        &capability_report,
        UiEvidenceAuthorityGeneration::new(41),
    );
    let scroll_viewport = contradictory_scroll_container_viewport(
        401,
        &capability_report,
        UiEvidenceAuthorityGeneration::new(41),
    );
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-contradict"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(41),
        &policy,
        &[
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&scroll_viewport),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("allocation neighborhood should admit");

    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("mismatched downward normalization must deny before later flows run");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture
    );
}

#[test]
fn equivalent_child_declaration_posture_converges_on_same_downward_payload() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-child-eq");
    let left = child_posture_constraints(world_profile.clone(), true, 51);
    let right = child_posture_constraints(world_profile, true, 51);

    assert_eq!(
        downward_payloads(&left),
        downward_payloads(&right),
        "equivalent parent basis and child declaration posture must converge"
    );
}

#[test]
fn changed_child_declaration_posture_changes_downward_payload_without_override() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-child-divergence");
    let bounded = child_posture_constraints(world_profile.clone(), true, 61);
    let unbounded = child_posture_constraints(world_profile, false, 61);

    assert_eq!(
        downward_payloads(&bounded),
        vec![UiConstraintParentAvailableSpace::new(
            crate::evidence::UiConstraintAxisScope::Primary,
            UiConstraintAvailableSpacePosture::DeclaredExtentUnknown,
            UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
            UiConstraintNormalizationPosture::deferred(),
        )]
    );
    assert_eq!(
        downward_payloads(&unbounded),
        vec![UiConstraintParentAvailableSpace::new(
            crate::evidence::UiConstraintAxisScope::Primary,
            UiConstraintAvailableSpacePosture::DeclaredExtentUnknown,
            UiConstraintBoundedMinMaxRequirement::None,
            UiConstraintNormalizationPosture::deferred(),
        )]
    );
    assert_ne!(bounded.identity(), unbounded.identity());
    assert_eq!(bounded_bounded_edges(&bounded), 1);
    assert_eq!(bounded_bounded_edges(&unbounded), 0);
}

fn downward_contradiction_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics],
    )
    .expect("downward contradiction policy should admit")
}

fn contradictory_scroll_container_viewport(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    crate::host::tests::measurement_fixture::collected_scroll_container_viewport_for_test(
        request_seed,
        report,
        generation,
    )
}

fn child_posture_constraints(
    world_profile: UiGraphWorldProfile,
    child_bounded: bool,
    authority_seed: u64,
) -> crate::evidence::UiAllocationConstraintSet {
    let app = child_posture_app(world_profile.clone(), child_bounded);
    let root_node = graph_node_identity_for_child_posture(&app, 0);
    let child_node = graph_node_identity_for_child_posture(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, child_node]);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-child-posture"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(authority_seed),
        &UiDeclaredMeasurementPolicyPosture::new(
            Some(UiDeclaredMeasurementMode::HugHeight),
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            None,
            None,
            vec![],
        )
        .expect("container policy should admit"),
        &[],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("allocation neighborhood should admit");

    basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("allocation constraints should admit")
}

fn child_posture_app(
    world_profile: UiGraphWorldProfile,
    child_bounded: bool,
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.graph.allocation-constraint.child-posture")
                .with_semantic_artifact_spec(child_posture_spec(
                    "workflow_editor.control.parent",
                    0,
                    true,
                ))
                .with_semantic_artifact_spec(child_posture_spec(
                    "workflow_editor.control.child",
                    1,
                    child_bounded,
                )),
        )
        .freeze()
}

fn child_posture_spec(
    semantic_key: &str,
    declaration_index: usize,
    bounded: bool,
) -> UiDslSemanticArtifactSpec {
    let mut spec = UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(
            "app/allocation_constraint_downward_tests.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new("operator:stack"))
    .with_posture_token(UiDslPostureToken::new("touch:press"));
    if bounded {
        spec = spec.with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"));
    }
    spec
}

fn graph_node_identity_for_child_posture(
    app: &crate::facade::WorthUiApp,
    declaration_index: usize,
) -> crate::graph::UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_constraint_downward_tests.wui"
                && provenance.declaration_index() == declaration_index
        })
        .expect("expected declaration artifact for requested provenance row");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should project one graph node")
}

fn downward_payloads(
    constraints: &crate::evidence::UiAllocationConstraintSet,
) -> Vec<UiConstraintParentAvailableSpace> {
    constraints
        .propagation_edges()
        .iter()
        .filter(|edge| edge.family() == UiConstraintPropagationEdgeFamily::ParentAvailableSpace)
        .map(|edge| {
            edge.payload()
                .parent_available_space()
                .expect("parent available-space edge must preserve typed payload")
        })
        .collect()
}

fn bounded_bounded_edges(constraints: &crate::evidence::UiAllocationConstraintSet) -> usize {
    constraints
        .propagation_edges()
        .iter()
        .filter(|edge| edge.family() == UiConstraintPropagationEdgeFamily::BoundedReconciliation)
        .count()
}
