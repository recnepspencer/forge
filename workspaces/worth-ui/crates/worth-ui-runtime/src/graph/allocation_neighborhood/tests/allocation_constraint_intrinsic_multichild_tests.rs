use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context,
    host_result_text_intrinsic_size_with_value, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput, UiConstraintIntrinsicSourcePosture,
    UiConstraintPropagationEdgeFamily,
};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

#[test]
fn sibling_intrinsic_contributors_preserve_distinct_upward_edges() {
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("allocation-constraint-intrinsic-multichild");
    let app = multi_child_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let first_child = graph_node_identity_for_provenance(&app, 1);
    let second_child = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, first_child, second_child]);
    let query_receipt = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("allocation-constraint-intrinsic-multichild"),
        UiEvidenceAuthorityGeneration::new(121),
        &query_intrinsic_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let report = capability_report(121);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-intrinsic-multichild"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(121),
        &query_intrinsic_policy(),
        &[
            MeasurementEvidenceInput::child_query_projection_fact(first_child, &query_receipt),
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                second_child,
                &host_result_text_intrinsic_size_with_value(
                    401,
                    &report,
                    UiEvidenceAuthorityGeneration::new(121),
                    180.0,
                    36.0,
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("multichild neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("multichild intrinsic constraints should admit");

    let intrinsic_edges = constraints
        .propagation_edges()
        .iter()
        .filter(|edge| {
            edge.family() == UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution
        })
        .collect::<Vec<_>>();

    assert_eq!(intrinsic_edges.len(), 2);

    let first = intrinsic_edges
        .iter()
        .find(|edge| {
            edge.payload()
                .child_intrinsic_contribution()
                .is_some_and(|contribution| {
                    contribution.contributor_graph_node_identity() == first_child
                })
        })
        .and_then(|edge| edge.payload().child_intrinsic_contribution())
        .expect("first child query contribution must survive");
    let second = intrinsic_edges
        .iter()
        .find(|edge| {
            edge.payload()
                .child_intrinsic_contribution()
                .is_some_and(|contribution| {
                    contribution.contributor_graph_node_identity() == second_child
                })
        })
        .and_then(|edge| edge.payload().child_intrinsic_contribution())
        .expect("second child host contribution must survive");

    assert_eq!(
        first.source_posture(),
        UiConstraintIntrinsicSourcePosture::QueryOnly
    );
    assert_eq!(first.primary_extent(), 240.0);
    assert_eq!(
        second.source_posture(),
        UiConstraintIntrinsicSourcePosture::HostOnly
    );
    assert_eq!(second.primary_extent(), 180.0);
    assert_ne!(first.identity_digest(), second.identity_digest());
}

fn query_intrinsic_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        None,
        None,
        None,
        vec![UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent],
    )
    .expect("query intrinsic policy should admit")
}

fn multi_child_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.graph.allocation-constraint-intrinsic")
                .with_semantic_artifact_spec(control_spec("workflow_editor.control.parent", 0))
                .with_semantic_artifact_spec(control_spec("workflow_editor.control.left", 1))
                .with_semantic_artifact_spec(control_spec("workflow_editor.control.right", 2)),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec(semantic_key: &str, declaration_index: usize) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(
            "app/allocation_constraint_intrinsic_multichild_tests.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new("operator:row"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
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
            provenance.module_path() == "app/allocation_constraint_intrinsic_multichild_tests.wui"
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
