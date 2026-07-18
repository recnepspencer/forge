use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_inspection::UiInspectionAdmissionPosture;

use crate::admission::{
    UiAdmissionAggregation, UiAdmissionFamily, UiAdmissionQueryBasis, UiAdmissionStaleEvidence,
    UiAdmissionTarget, UiAdmissionWorld, UiLegalityPosture, UiLegalityReason, UiSupportPosture,
};
use crate::declaration::UiDeclarationArtifact;
use crate::facade::WorthUi;

use super::UiAdmissionBoundary;

#[test]
fn owner_boundary_can_prove_missing_declaration_artifact_denial() {
    let app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.admission.denied")
                .with_semantic_artifact_spec(admitted_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let admitted = artifact_from_file_provenance(&app, "app/admission_denied.wui", 0);
    let support_artifacts = app.declaration_artifacts().to_vec();
    let legality_artifacts = app
        .declaration_artifacts()
        .iter()
        .filter(|artifact| artifact.identity() != admitted.identity())
        .cloned()
        .collect::<Vec<_>>();
    let boundary = UiAdmissionBoundary::from_authority_parts(
        &support_artifacts,
        &legality_artifacts,
        app.graph_snapshot(),
    );

    let denied_report = boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity(&app, admitted),
        UiAdmissionWorld::authoritative(),
    ));

    assert_eq!(denied_report.aggregation(), UiAdmissionAggregation::Denied);
    assert_eq!(
        denied_report.support_snapshot().posture(),
        &UiSupportPosture::Supported {
            family: UiAdmissionFamily::TouchMeaning,
            world: UiAdmissionWorld::authoritative(),
        }
    );
    assert_eq!(
        denied_report.inspection_posture(),
        UiInspectionAdmissionPosture::Stale
    );
    assert_eq!(
        denied_report
            .legality_decision()
            .expect("denied report should retain legality truth")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::Stale {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: UiAdmissionQueryBasis::StaleReceipt,
            evidence: UiAdmissionStaleEvidence::DeclarationArtifactMissing,
        })
    );
}

fn artifact_from_file_provenance<'a>(
    app: &'a crate::facade::WorthUiApp,
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

fn graph_node_identity(
    app: &crate::facade::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> crate::graph::UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should project one graph node")
}

fn admitted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.plain"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_denied.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:plain"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}
