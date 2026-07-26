use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken,
};

pub(super) fn inspection_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named("worth-ui.certification.graph-inspection")
                .with_semantic_artifact_spec(control_spec())
                .with_semantic_artifact_spec(consumer_spec())
                .with_semantic_artifact_spec(competing_control_spec())
                .with_semantic_artifact_spec(competing_consumer_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

pub(super) fn graph_node_identity(
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

pub(super) fn published_aspect(
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::declaration::UiAspectName {
    artifact
        .aspect_contract()
        .expect("control declaration should admit aspect contract")
        .published()
        .aspects()
        .first()
        .cloned()
        .expect("control declaration should publish one aspect")
}

pub(super) fn root_page_artifact(
    app: &worth_ui::facade::app::WorthUiApp,
) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| {
                    handoff.role()
                        == worth_ui::facade::declaration::UiDeclarationStructuralRole::Page
                })
                .unwrap_or(false)
        })
        .expect("bootstrap root page artifact should exist")
}

pub(super) fn artifact_from_file_provenance<'a>(
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
            panic!("expected declaration artifact for {module_path}#{declaration_index}")
        })
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.inspectable"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:inspect"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_published_aspect(UiDslAspectName::new("content.text"))
}

fn consumer_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.inspectable_consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:inspect-consumer"))
    .with_consumed_aspect(UiDslAspectName::new("content.text"))
}

fn competing_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.inspectable_icon"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("control:inspect-icon"))
    .with_published_aspect(UiDslAspectName::new("appearance.background"))
}

fn competing_consumer_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.inspectable_icon_consumer"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_inspection_runtime.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("region:inspect-icon-consumer"))
    .with_consumed_aspect(UiDslAspectName::new("appearance.background"))
}
