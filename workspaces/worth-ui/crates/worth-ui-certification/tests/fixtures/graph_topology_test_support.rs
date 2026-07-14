use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::UiGraphNodeIdentity;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

pub(crate) fn graph_node_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    artifact: &UiDeclarationArtifact,
) -> UiGraphNodeIdentity {
    graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

pub(crate) fn root_page_artifact(
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

pub(crate) fn artifact_from_file_provenance<'a>(
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
            panic!("expected declaration artifact for {module_path}#{declaration_index} on freeze path")
        })
}

pub(crate) fn slotted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_topology.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new("operator:stack"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

pub(crate) fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_topology.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

pub(crate) fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/graph_topology.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
    .with_structural_token(UiDslStructuralToken::new(
        "mosaic-sizing:workspace.sizing.main",
    ))
}

pub(crate) fn page_set_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page_set.shell"),
        UiDslSemanticFamily::PageSet,
        UiDslSourceProvenance::file_authored("app/graph_topology_claims.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page-set:shell"))
}

pub(crate) fn local_composition_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
        UiDslSemanticFamily::LocalComposition,
        UiDslSourceProvenance::file_authored("app/graph_topology_claims.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("local-composition:inspector"))
}

pub(crate) fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/graph_topology_claims.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}

pub(crate) fn extra_root_page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.authored_root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/graph_topology_root_denial.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
}

pub(crate) fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_string(),
            Err(_) => "<non-string panic payload>".to_string(),
        },
    }
}
