use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationStatus,
    UiGraphWorldProfile,
};
use worth_ui_certification::scenario::installed_query_world;
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_test_support::WorthUiApplicationBuilderCertificationExt;

pub fn touch_app(world_profile: UiGraphWorldProfile) -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(world_profile)
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.graph-touch")
                .with_semantic_artifact_spec(control_spec())
                .with_semantic_artifact_spec(region_spec())
                .with_semantic_artifact_spec(mosaic_spec()),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
}

pub fn control_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_runtime.wui", 0)
}

pub fn region_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_runtime.wui", 1)
}

pub fn mosaic_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    artifact_from_file_provenance(app, "app/graph_touch_runtime.wui", 2)
}

pub fn graph_node_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

pub fn mount_eligibility_transition(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphMountEligibilityTransition {
    let graph = app.graph();
    let graph_node_identity = graph_node_identity(graph, artifact);
    let control_node = graph
        .lookup()
        .graph_node(graph_node_identity)
        .expect("graph should resolve control node")
        .value();

    graph
        .mount_eligibility_transition_for_node(
            graph_node_identity,
            control_node
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted),
            UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
        )
        .expect("mounted admission should yield one graph-owned transition")
}

pub fn query_snapshot_world_profile(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let binding = schema_basis_parts.join(".").replace('-', "_");
    installed_query_world::settled_query_world_profile(
        worth_ui::facade::declaration::ViewBindingId::new(binding.clone()).unwrap(),
        format!("{binding}.{snapshot_label}").replace('-', "_"),
    )
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_touch_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_touch_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/graph_touch_runtime.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
}

fn artifact_from_file_provenance<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        })
        .unwrap_or_else(|| {
            panic!(
                "expected declaration artifact for {module_path}#{declaration_index} on freeze path"
            )
        })
}
