use std::sync::Arc;

use worth_ui::facade::admission::UiAdmissionQueryBasis;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    admit_runtime_current_snapshot_basis, snapshot_resolution_report, QueryExternalIdentityToken,
    QueryExternalSchemaBasisToken, UiGraphWorldProfile, WorthQuerySnapshotIdentity,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryBindingSubsystem, WorthUiQueryCausalExplanationLane,
    WorthUiQueryInspectionLane, WorthUiQueryProjectionConsumptionLane,
};

pub fn artifact_from_file_provenance<'a>(
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

pub fn graph_node_identity(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should project one graph node")
}

pub fn admitted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.plain"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_denials.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:plain"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

pub fn query_bound_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.query"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_denials.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("control:query"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

pub fn service_bound_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.service"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_denials.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("control:service"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

pub fn query_snapshot_world_profile() -> UiGraphWorldProfile {
    let snapshot_identity = WorthQuerySnapshotIdentity::admit_external_token(
        QueryExternalIdentityToken::new(Arc::<str>::from("snapshot:admission-denials")),
    );
    let basis = admit_runtime_current_snapshot_basis(
        snapshot_identity.evidence_identity(),
        QueryExternalSchemaBasisToken::from_domain_parts(
            &["worth-ui.phase6", "admission", "denials"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        ),
    )
    .expect("runtime current snapshot basis should resolve");

    UiGraphWorldProfile::query_snapshot_basis(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query snapshot basis world should admit")
}

pub fn query_prerequisites(
    world_profile: &UiGraphWorldProfile,
    query_basis: UiAdmissionQueryBasis,
) -> worth_ui::facade::query_binding::WorthUiQueryPrerequisiteEvidence {
    let UiGraphWorldProfile::QuerySnapshotBasis {
        basis,
        resolution_report,
    } = world_profile
    else {
        panic!("query denial proofs require query snapshot worlds");
    };

    WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .assemble(
            basis.clone(),
            resolution_report.clone(),
            match query_basis {
                UiAdmissionQueryBasis::GraphAligned => WorthUiQueryBasisPosture::GraphAligned,
                UiAdmissionQueryBasis::WrongWorldProjection => {
                    WorthUiQueryBasisPosture::WrongWorldProjection
                }
                UiAdmissionQueryBasis::RebindRequired => WorthUiQueryBasisPosture::RebindRequired,
                UiAdmissionQueryBasis::StaleReceipt => WorthUiQueryBasisPosture::StaleReceipt,
                UiAdmissionQueryBasis::AmbiguousSources => {
                    WorthUiQueryBasisPosture::AmbiguousSources
                }
            },
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
        .expect("query prerequisite assembly should admit")
}
