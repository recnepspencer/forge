use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_scroll_container_viewport,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiConstraintAvailableSpacePosture,
    UiConstraintNormalizationPosture, UiConstraintParentAvailableSpace,
    UiConstraintPropagationDenialReason, UiConstraintPropagationEdgeFamily,
    UiConstraintResizePermissionPosture,
};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

#[test]
fn equivalent_neighborhood_derivation_emits_one_canonical_constraint_edge_set() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-eq");
    let app = control_app(world_profile.clone(), "operator:stack");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let policy = container_policy();
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-eq"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(17),
        &policy,
        &[],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("allocation neighborhood should admit");

    let left = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("left constraint set should admit");
    let right = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("right constraint set should admit");

    assert_eq!(left, right);
    assert_eq!(
        left.propagation_edges(),
        right.propagation_edges(),
        "equivalent neighborhoods must emit one canonical edge set"
    );
}

#[test]
fn undeclared_special_input_observations_do_not_reenter_runtime_authority() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-denial");
    let app = control_app(world_profile.clone(), "operator:stack");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let policy = UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        Some(crate::declaration::UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis),
        vec![],
    )
    .expect("scroll viewport policy should admit");
    let capability_report = capability_report(77);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-denial"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(19),
        &policy,
        &[
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_scroll_container_viewport(
                    99,
                    &capability_report,
                    UiEvidenceAuthorityGeneration::new(19),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("allocation neighborhood should admit");

    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("undeclared special input observations should stay observational");

    assert_eq!(
        constraints.summary().scroll_owner_requirement(),
        crate::evidence::UiConstraintSpecialInputPosture::NotRequired
    );
    assert!(
        constraints
            .propagation_edges()
            .iter()
            .all(|edge| edge.family() != UiConstraintPropagationEdgeFamily::ScrollViewportInput),
        "observed scroll input must not mint ordinary semantic authority when the operator never declared it"
    );
}

#[test]
fn missing_required_downward_constraint_denies_before_other_flows() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-missing");
    let app = control_app(world_profile.clone(), "operator:stack");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let denied_basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-missing"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(22),
        &UiDeclaredMeasurementPolicyPosture::new(
            Some(UiDeclaredMeasurementMode::HugHeight),
            Some(UiDeclaredMeasurementConstraintModifier::Bounded),
            Some(crate::declaration::UiDeclaredMeasurementBasisSource::ScrollViewport),
            None,
            vec![],
        )
        .expect("scroll viewport policy should admit"),
        &[],
    );
    let neighborhood = denied_basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("allocation neighborhood should admit");

    let denial = denied_basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("denied basis must not mint downward constraints");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::MissingRequiredDownwardConstraint
    );
}

#[test]
fn operator_specific_contracts_emit_distinct_production_edge_sets() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-operator");
    let stack_app = control_app(world_profile.clone(), "operator:stack");
    let grid_app = control_app(world_profile.clone(), "operator:grid");
    let stack_root = graph_node_identity_for_provenance(&stack_app, 0);
    let grid_root = graph_node_identity_for_provenance(&grid_app, 0);
    let stack_snapshot = snapshot_with_admitted_layout(&stack_app, &[stack_root]);
    let grid_snapshot = snapshot_with_admitted_layout(&grid_app, &[grid_root]);
    let stack_basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-operator"),
        stack_root,
        world_profile.clone(),
        UiEvidenceAuthorityGeneration::new(23),
        &container_policy(),
        &[],
    );
    let grid_basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-operator"),
        grid_root,
        world_profile,
        UiEvidenceAuthorityGeneration::new(23),
        &container_policy(),
        &[],
    );
    let stack_neighborhood = stack_basis
        .admit_allocation_neighborhood_from_graph(&stack_snapshot)
        .expect("stack neighborhood should admit");
    let grid_neighborhood = grid_basis
        .admit_allocation_neighborhood_from_graph(&grid_snapshot)
        .expect("grid neighborhood should admit");
    let stack_constraints = stack_basis
        .admit_allocation_constraint_set(&stack_neighborhood)
        .expect("stack constraints should admit");
    let grid_constraints = grid_basis
        .admit_allocation_constraint_set(&grid_neighborhood)
        .expect("grid constraints should admit");

    assert_ne!(
        stack_neighborhood
            .layout_operator_planning_contract()
            .semantics()
            .child_participation_rule(),
        grid_neighborhood
            .layout_operator_planning_contract()
            .semantics()
            .child_participation_rule()
    );
    assert_ne!(
        stack_constraints.identity().identity_digest(),
        grid_constraints.identity().identity_digest()
    );
    assert_eq!(
        stack_constraints.summary().equal_share_group(),
        crate::evidence::UiConstraintEqualShareGroup::None
    );
    assert_ne!(
        stack_constraints.summary().equal_share_group(),
        grid_constraints.summary().equal_share_group()
    );
}

#[test]
fn split_declared_durable_resize_support_stays_latent_without_runtime_witness_on_raw_constraint_lane(
) {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-resize");
    let app = control_app(world_profile.clone(), "operator:split");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-resize"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(29),
        &container_policy(),
        &[],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("split neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("split constraints should admit with declared durable support");

    assert_eq!(
        constraints.summary().resize_permission_posture(),
        UiConstraintResizePermissionPosture::DurableAuthorityLane
    );
    assert!(
        constraints
            .propagation_edges()
            .iter()
            .all(|edge| edge.family() != UiConstraintPropagationEdgeFamily::DurableResizeInput),
        "declared durable posture alone must not mint ordinary durable resize authority on the raw constraint lane"
    );
}

#[test]
fn parent_available_space_edges_stay_child_facing_and_typed() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-downward");
    let app = control_app(world_profile.clone(), "operator:stack");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-downward"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(35),
        &container_policy(),
        &[],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("stack neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("stack constraints should admit");

    let downward_edges = constraints
        .propagation_edges()
        .iter()
        .filter(|edge| edge.family() == UiConstraintPropagationEdgeFamily::ParentAvailableSpace)
        .collect::<Vec<_>>();

    assert_eq!(downward_edges.len(), 1);
    let root_member = neighborhood
        .members()
        .iter()
        .find(|member| member.graph_node_identity() == root_node)
        .expect("root member should be present");
    let peer_member = neighborhood
        .members()
        .iter()
        .find(|member| member.graph_node_identity() == peer_node)
        .expect("peer member should be present");
    assert_eq!(
        downward_edges[0].source_member_identity_digest(),
        root_member.identity_digest()
    );
    assert_eq!(
        downward_edges[0].target_member_identity_digest(),
        peer_member.identity_digest()
    );
    assert_eq!(
        downward_edges[0]
            .payload()
            .parent_available_space()
            .expect("parent available-space edge must preserve typed downward witness"),
        UiConstraintParentAvailableSpace::new(
            crate::evidence::UiConstraintAxisScope::Primary,
            UiConstraintAvailableSpacePosture::DeclaredExtentUnknown,
            crate::evidence::UiConstraintBoundedMinMaxRequirement::PrimaryAxis,
            UiConstraintNormalizationPosture::deferred(),
        )
    );
}

fn container_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("container policy should admit")
}

pub(crate) fn control_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.graph.allocation-constraint",
            )
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.primary",
                0,
                operator_token,
            ))
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.sibling",
                1,
                operator_token,
            )),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec(
    semantic_key: &str,
    declaration_index: usize,
    operator_token: &str,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(
            "app/allocation_constraint_projection_tests.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new(operator_token))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
}

pub(crate) fn graph_node_identity_for_provenance(
    app: &crate::facade::WorthUiApp,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_constraint_projection_tests.wui"
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
