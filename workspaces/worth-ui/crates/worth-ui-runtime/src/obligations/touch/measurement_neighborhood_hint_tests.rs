use crate::facade::WorthUiRustAuthoredDeclarationFixture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_font_metrics,
    host_result_viewport_extent, scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput,
};
use crate::facade::WorthUi;
use crate::graph::{UiGraphMeasurementNeighborhoodHint, UiGraphNodeIdentity, UiGraphWorldProfile};
use crate::obligations::touch::UiGraphTouchDenial;

#[test]
fn touch_measurement_neighborhood_hint_only_projects_for_matching_node() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("touch-neighborhood");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("touch-neighborhood");
    let app = touch_neighborhood_app(world_profile.clone());
    let graph_node_identity = graph_node_identity_for_provenance(&app, 0);
    let sibling_graph_node_identity = graph_node_identity_for_provenance(&app, 1);
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let basis = admit_measurement_basis(
        declaration_identity,
        graph_node_identity,
        world_profile.clone(),
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::settled_query_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                21,
                &capability_report,
                generation,
            )),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                22,
                &capability_report,
                generation,
            )),
        ],
    );
    let graph_hint = UiGraphMeasurementNeighborhoodHint::from_basis(&basis);
    let matching_touch = app
        .try_query_touch_for_node(graph_node_identity)
        .expect("matching query touch should admit through the ordinary query-world lane");
    let sibling_touch = app
        .try_query_touch_for_node(sibling_graph_node_identity)
        .expect("sibling query touch should admit through the ordinary query-world lane");

    let matching_projection = matching_touch
        .project_measurement_neighborhood_hint(&graph_hint)
        .expect("matching touch should admit neighborhood metadata");

    assert_eq!(
        matching_projection.touch_identity_digest(),
        matching_touch.identity_digest()
    );
    assert_eq!(
        matching_projection.graph_node_identity(),
        graph_node_identity
    );
    assert_eq!(
        matching_projection.world_identity_digest(),
        graph_hint.world_identity_digest()
    );
    assert_eq!(
        matching_projection.neighborhood_class_hint(),
        graph_hint.neighborhood_class_hint()
    );
    assert_eq!(
        matching_projection.dependency_map(),
        graph_hint.dependency_map()
    );
    assert!(
        sibling_touch
            .project_measurement_neighborhood_hint(&graph_hint)
            .is_none(),
        "touch metadata should stay node-local"
    );
    assert_ne!(
        matching_projection.dependency_map().identity_digest(),
        0,
        "touch projection should preserve a real dependency-map identity"
    );
}

#[test]
fn query_fact_touch_origins_deny_outside_query_world_before_touch_construction() {
    let authoritative_app = touch_neighborhood_app(UiGraphWorldProfile::authoritative());
    let graph_node_identity = graph_node_identity_for_provenance(&authoritative_app, 0);

    assert_eq!(
        authoritative_app
            .try_query_touch_for_node(graph_node_identity)
            .expect_err("authoritative world should deny query-fact touch origins"),
        UiGraphTouchDenial::QueryBindingChangeUnavailableInCurrentWorld
    );
}

fn touch_neighborhood_app(world_profile: UiGraphWorldProfile) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.touch.measurement.neighborhood",
            )
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.primary",
                "app/touch_neighborhood.wui",
                0,
                "control:primary",
                "touch:press",
            ))
            .with_semantic_artifact_spec(control_spec(
                "workflow_editor.control.sibling",
                "app/touch_neighborhood.wui",
                1,
                "control:sibling",
                "touch:press",
            )),
        )
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed")
}

fn control_spec(
    semantic_key: &str,
    module_path: &str,
    declaration_index: usize,
    structural_token: &str,
    posture_token: &str,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(module_path, declaration_index),
    )
    .with_structural_token(UiDslStructuralToken::new(structural_token))
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
            artifact
                .provenance()
                .source_provenance()
                .declaration_index()
                == declaration_index
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
