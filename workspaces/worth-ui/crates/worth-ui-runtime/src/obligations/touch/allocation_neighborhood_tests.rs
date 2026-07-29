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
use crate::evidence::{
    admit_measurement_basis,
    measurement::projection::fact_test_support::{
        display_field_projection_context, synthetic_declaration_identity,
    },
};
use crate::facade::WorthUi;
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

#[test]
fn touch_allocation_neighborhood_only_projects_for_matching_node() {
    let (_, _, world_profile) = display_field_projection_context("touch-allocation-neighborhood");
    let app = touch_neighborhood_app(world_profile.clone());
    let graph_node_identity = graph_node_identity_for_provenance(&app, 0);
    let sibling_graph_node_identity = graph_node_identity_for_provenance(&app, 1);
    let touch = app
        .try_query_touch_for_node(graph_node_identity)
        .expect("matching query touch should admit");
    let sibling_touch = app
        .try_query_touch_for_node(sibling_graph_node_identity)
        .expect("sibling query touch should admit");
    let admitted_snapshot = snapshot_with_admitted_layout(&app, &[graph_node_identity]);
    let selected = app.admission().select_obligations(&touch);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("touch-allocation-neighborhood"),
        graph_node_identity,
        world_profile,
        UiEvidenceAuthorityGeneration::new(17),
        &container_policy(),
        &[],
    );
    let neighborhood = selected
        .admit_allocation_neighborhood(&admitted_snapshot, &basis)
        .expect("allocation neighborhood should admit");

    let touch_projection = touch
        .project_allocation_neighborhood(&neighborhood)
        .expect("matching touch should project neighborhood");

    assert_eq!(
        touch_projection.touch_identity_digest(),
        touch.identity_digest()
    );
    assert_eq!(touch_projection.graph_node_identity(), graph_node_identity);
    assert_eq!(
        touch_projection.neighborhood().identity(),
        neighborhood.identity()
    );
    assert!(
        sibling_touch
            .project_allocation_neighborhood(&neighborhood)
            .is_none(),
        "touch-scoped neighborhood projection must stay node-local"
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
    .expect("container measurement policy should admit")
}

fn touch_neighborhood_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.touch.allocation-neighborhood",
            )
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.primary",
                0,
                "control:primary",
                "touch:press",
            ))
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.sibling",
                1,
                "control:sibling",
                "touch:press",
            )),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec(
    semantic_key: &str,
    declaration_index: usize,
    structural_token: &str,
    posture_token: &str,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(
            "app/touch_allocation_neighborhood.wui",
            declaration_index,
        ),
    )
    .with_structural_token(UiDslStructuralToken::new(structural_token))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new(posture_token))
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
            provenance.module_path() == "app/touch_allocation_neighborhood.wui"
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
