use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_text_intrinsic_size,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiAllocationNeighborhood,
    UiConstraintPropagationDenialReason,
};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::UiGraphNodeIdentity;

#[test]
fn sibling_negotiation_is_explicit_and_records_solve_order() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-sibling");
    let app = peer_app(
        world_profile.clone(),
        "operator:grid",
        &[false, false, false],
    );
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(41);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-sibling"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(41),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    501,
                    &report,
                    UiEvidenceAuthorityGeneration::new(41),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_b,
                &host_result_text_intrinsic_size(
                    777,
                    &report,
                    UiEvidenceAuthorityGeneration::new(41),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("grid neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("sibling negotiation requires an explicit fixed-point witness");

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
fn equivalent_peer_reorder_converges_on_the_same_negotiation_result() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-sibling-eq");
    let app = peer_app(
        world_profile.clone(),
        "operator:row",
        &[false, false, false],
    );
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(52);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-sibling-eq"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(52),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    411,
                    &report,
                    UiEvidenceAuthorityGeneration::new(52),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_b,
                &host_result_text_intrinsic_size(
                    412,
                    &report,
                    UiEvidenceAuthorityGeneration::new(52),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("row neighborhood should admit");
    let reordered_neighborhood = UiAllocationNeighborhood::new_with_authority(
        neighborhood.root_graph_node_identity(),
        neighborhood.graph_generation(),
        neighborhood.world_identity_digest(),
        neighborhood.measurement_basis_identity_digest(),
        neighborhood.layout_operator_planning_contract().clone(),
        neighborhood.dependency_map().clone(),
        neighborhood.neighborhood_class(),
        neighborhood.membership_rule(),
        neighborhood.members().iter().rev().cloned().collect(),
    );

    let left = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("canonical row constraints should admit");
    let right = basis
        .admit_allocation_constraint_set(&reordered_neighborhood)
        .expect("reordered row constraints should admit");

    assert_eq!(
        left.sibling_negotiation()
            .expect("left sibling negotiation")
            .identity_digest(),
        right
            .sibling_negotiation()
            .expect("right sibling negotiation")
            .identity_digest()
    );
}

#[test]
fn contradictory_bounded_peer_requirements_deny_before_equal_share_and_bounds() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-sibling-contradiction");
    let app = peer_app(world_profile.clone(), "operator:row", &[true, true, true]);
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(61);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-sibling-contradiction"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(61),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    620,
                    &report,
                    UiEvidenceAuthorityGeneration::new(61),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("row neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("bounded peer missing intrinsic evidence must deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::ContradictorySiblingRequirements
    );
    assert_eq!(
        denial.family(),
        Some(crate::evidence::UiConstraintPropagationEdgeFamily::SiblingNegotiation)
    );
}

#[test]
fn primary_axis_fixed_point_denies_without_runtime_durable_resize_witness() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-sibling-fixed-point");
    let app = peer_app(world_profile.clone(), "operator:split", &[true, true, true]);
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(62);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-sibling-fixed-point"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(62),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    621,
                    &report,
                    UiEvidenceAuthorityGeneration::new(62),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_b,
                &host_result_text_intrinsic_size(
                    622,
                    &report,
                    UiEvidenceAuthorityGeneration::new(62),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("split neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("split peers must deny fixed-point admission until the runtime durable resize witness is staged");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint
    );
    assert_eq!(
        denial.family(),
        Some(crate::evidence::UiConstraintPropagationEdgeFamily::SiblingNegotiation)
    );
}

fn intrinsic_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("intrinsic policy should admit")
}

fn peer_app(
    world_profile: crate::graph::UiGraphWorldProfile,
    operator_token: &str,
    bounded_flags: &[bool; 3],
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.graph.allocation-constraint-sibling")
                .with_semantic_artifact_spec(control_spec(
                    "workflow_editor.root",
                    0,
                    operator_token,
                    bounded_flags[0],
                ))
                .with_semantic_artifact_spec(control_spec(
                    "workflow_editor.peer.a",
                    1,
                    operator_token,
                    bounded_flags[1],
                ))
                .with_semantic_artifact_spec(control_spec(
                    "workflow_editor.peer.b",
                    2,
                    operator_token,
                    bounded_flags[2],
                )),
        )
        .freeze()
}

fn control_spec(
    semantic_key: &str,
    declaration_index: usize,
    operator_token: &str,
    bounded: bool,
) -> UiDslSemanticArtifactSpec {
    let spec = UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(
            "app/allocation_constraint_sibling_tests.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new(operator_token))
    .with_posture_token(UiDslPostureToken::new("touch:press"));
    if bounded {
        spec.with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
    } else {
        spec
    }
}

fn graph_node_identity_for_provenance(
    app: &crate::facade::WorthUiApp,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_constraint_sibling_tests.wui"
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
