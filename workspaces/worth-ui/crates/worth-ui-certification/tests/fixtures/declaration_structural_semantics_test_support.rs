use worth_ui::facade::declaration::{
    UiDeclarationArtifact, UiDeclarationContainmentIntent, UiDeclarationFamilyKind,
    UiDeclarationStructuralRole,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, UiDslSupportToken,
};

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

pub(crate) fn assert_structural_projection(
    artifact: &UiDeclarationArtifact,
    expected_family: UiDeclarationFamilyKind,
    expected_role: UiDeclarationStructuralRole,
    expected_containment: &UiDeclarationContainmentIntent,
    expected_claim_name: Option<&str>,
) {
    let structural = artifact
        .structural_semantics()
        .expect("structural family should admit structural semantics");
    let handoff = artifact
        .graph_handoff()
        .expect("structural family should derive structural handoff");

    assert_eq!(structural.family(), expected_family);
    assert_eq!(structural.role(), expected_role);
    assert_eq!(structural.containment_intent(), expected_containment);
    assert_eq!(
        structural.containment_intent().claim_name(),
        expected_claim_name
    );
    assert!(structural.slot_participation_intent().is_none());
    assert_eq!(handoff.family_kind(), expected_family);
    assert_eq!(handoff.role(), expected_role);
    assert_eq!(handoff.containment_intent(), expected_containment);
    assert_eq!(
        handoff.containment_intent().claim_name(),
        expected_claim_name
    );
    assert!(handoff.slot_participation_intent().is_none());
}

pub(crate) fn slotted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/structural_semantics.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new("operator:stack"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

pub(crate) fn slotted_control_with_noise_spec() -> UiDslSemanticArtifactSpec {
    slotted_control_spec()
        .with_published_aspect(UiDslAspectName::new("content.text"))
        .with_support_token(UiDslSupportToken::new("support:preview-only"))
}

pub(crate) fn page_set_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page_set.shell"),
        UiDslSemanticFamily::PageSet,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page-set:shell"))
}

pub(crate) fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

pub(crate) fn mosaic_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.mosaic.workspace"),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("mosaic:workspace"))
}

pub(crate) fn local_composition_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.local_composition.inspector"),
        UiDslSemanticFamily::LocalComposition,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("local-composition:inspector"))
}

pub(crate) fn diagnostic_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostics.lint"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/structural_families.wui", 4),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:lint"))
}

pub(crate) fn unsupported_structural_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.unsupported"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/structural_denials.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("repeat:many"))
}

pub(crate) fn page_with_slot_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.page.invalid_slot"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::file_authored("app/structural_invalid_slot.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
}

pub(crate) fn standalone_query_binding_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.query.selection"),
        UiDslSemanticFamily::QueryBinding,
        UiDslSourceProvenance::file_authored("app/structural_non_structural.wui", 0),
    )
    .with_posture_token(UiDslPostureToken::new("query-binding:standalone"))
}
