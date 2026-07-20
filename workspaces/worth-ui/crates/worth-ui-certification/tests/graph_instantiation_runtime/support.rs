use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclarationStructuralRole};
use worth_ui::facade::graph::{
    UiGraphNodeInstantiationEntry, UiGraphParticipationAxis, UiGraphParticipationEvidenceHandle,
    UiGraphParticipationReasonCode, UiGraphParticipationReasonSource, UiGraphParticipationStatus,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};

pub(super) fn artifact_from_file_provenance<'a>(
    app: &'a WorthUiApp,
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

pub(super) fn root_page_artifact(app: &WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact
                .graph_handoff()
                .map(|handoff| handoff.role() == UiDeclarationStructuralRole::Page)
                .unwrap_or(false)
        })
        .expect("bootstrap root page artifact should exist")
}

pub(super) fn control_graph_input_spec() -> UiDslSemanticArtifactSpec {
    graph_input_with_non_graph_obligations()
}

pub(super) fn graph_input_without_non_graph_obligations() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_instantiation.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

pub(super) fn graph_input_with_non_graph_obligations() -> UiDslSemanticArtifactSpec {
    graph_input_without_non_graph_obligations()
        .with_posture_token(UiDslPostureToken::new("touch:press"))
        .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
}

pub(super) fn invalid_graph_input_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.invalid"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_instantiation_invalid.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("service:unknown"))
}

pub(super) fn assert_participation_seed_axis(
    entry: &UiGraphNodeInstantiationEntry,
    axis: UiGraphParticipationAxis,
    status: UiGraphParticipationStatus,
    source: UiGraphParticipationReasonSource,
    reason: UiGraphParticipationReasonCode,
    evidence: UiGraphParticipationEvidenceHandle,
) {
    let participation = entry.participation_seed().axis(axis);

    assert_eq!(participation.status(), status);
    assert_eq!(participation.source(), source);
    assert_eq!(participation.reason(), reason);
    assert_eq!(participation.evidence_handle(), evidence);
}
