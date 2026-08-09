use super::*;

pub(super) fn slotted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

pub(super) fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

pub(super) fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
    .with_structural_token(UiDslStructuralToken::new(
        "mosaic-sizing:workspace.sizing.main",
    ))
}

pub(super) fn page_set_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page_set.shell"),
        UiDslSemanticFamily::PageSet,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("page-set:shell"))
}

pub(super) fn local_composition_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
        UiDslSemanticFamily::LocalComposition,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 4),
    )
    .with_structural_token(UiDslStructuralToken::new("local-composition:inspector"))
}

pub(super) fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostic_surface.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/graph_topology_authority.wui", 5),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}

pub(super) fn extra_root_page_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.authored_root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/graph_topology_root_denial.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
}
