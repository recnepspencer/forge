#![cfg(test)]

use std::sync::Arc;

use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiInspectionQuery, UiInspectionScope,
    UiInspectionSupportReport, UiInspectionTarget,
};

use crate::admission::{UiAdmissionTarget, UiAdmissionWorld};
use crate::declaration::{UiDeclarationArtifact, UiDeclarationStructuralRole};
use crate::evidence::{
    admit_measurement_basis, project_measurement_inspection_view, MeasurementEvidenceInput,
};
use crate::graph::{
    UiGraphInstantiationPlan, UiGraphWorldProfile, UiRuntimeDataInstanceKeyToken,
    UiRuntimeInstanceBasisAdmission,
};
use crate::obligations::touch::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchTiming,
};

use super::inspection_bridge::UiMeasurementInspectionEvidenceBundle;
use super::{WorthUi, WorthUiApp, WorthUiDslPackage};

pub(super) fn measurement_query(target: UiInspectionTarget) -> UiInspectionQuery {
    UiInspectionQuery::new(target, UiInspectionScope::Measurement)
        .with_budget(UiEvidenceBudget::ordinary())
        .with_richness(UiEvidenceRichness::materialized_detail())
}

pub(super) fn measurement_detail(
    slice: &crate::evidence::UiEvidenceSlice,
) -> &worth_ui_inspection::UiInspectionMeasurementEvidenceView {
    match slice.materialized_detail() {
        Some(crate::evidence::UiEvidenceMaterializedDetail::Measurement(view)) => view,
        other => panic!("expected materialized measurement detail, got {other:?}"),
    }
}

pub(super) fn host_measurement_app() -> WorthUiApp {
    measurement_app_in_world(
        host_measurement_package(),
        UiGraphWorldProfile::authoritative(),
        None,
    )
}

pub(super) fn query_measurement_app_in_world(
    graph_world_profile: UiGraphWorldProfile,
    evidence: Option<UiMeasurementInspectionEvidenceBundle>,
) -> WorthUiApp {
    measurement_app_in_world(query_measurement_package(), graph_world_profile, evidence)
}

pub(super) fn measurement_app_in_world(
    dsl_package: WorthUiDslPackage,
    graph_world_profile: UiGraphWorldProfile,
    evidence: Option<UiMeasurementInspectionEvidenceBundle>,
) -> WorthUiApp {
    let mut builder = WorthUi::app()
        .with_dsl_package(dsl_package)
        .with_graph_world_profile(graph_world_profile);
    if let Some(bundle) = evidence {
        builder = builder.with_measurement_inspection_evidence(bundle);
    }
    builder.freeze()
}

pub(super) fn graph_node_identity(app: &WorthUiApp) -> crate::graph::UiGraphNodeIdentity {
    let artifact = control_artifact(app);
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()[0]
}

pub(super) fn repeated_instance_app() -> WorthUiApp {
    let baseline = host_measurement_app();
    let root_page_handoff = root_page_artifact(&baseline)
        .graph_handoff()
        .expect("bootstrap root page should lower to graph handoff")
        .clone();
    let control_handoff = control_artifact(&baseline)
        .graph_handoff()
        .expect("control should lower to graph handoff")
        .clone();
    let (capability_snapshot, declaration_artifacts, _, lifecycle) =
        baseline.into_authority_parts();
    let graph_snapshot = UiGraphInstantiationPlan::admit_handoffs(
        &[
            root_page_handoff,
            control_handoff.clone(),
            control_handoff.clone(),
        ],
        &[
            runtime_basis("row:user-7", control_handoff.identity()),
            runtime_basis("row:user-8", control_handoff.identity()),
        ],
    )
    .expect("typed repeated-instance runtime basis should admit internal graph plan")
    .commit_initial_generation(UiGraphWorldProfile::authoritative())
    .expect("typed repeated-instance graph plan should publish authoritative graph")
    .into_committed_snapshot();

    WorthUiApp::from_authority_parts(
        capability_snapshot,
        declaration_artifacts,
        graph_snapshot,
        lifecycle,
    )
}

pub(super) fn direct_measurement_view_for_graph_node(
    app: &WorthUiApp,
    bundle: &UiMeasurementInspectionEvidenceBundle,
    graph_node_identity: crate::graph::UiGraphNodeIdentity,
) -> worth_ui_inspection::UiInspectionMeasurementEvidenceView {
    let artifact = control_artifact(app);
    let policy = artifact
        .support_snapshot()
        .expect("control artifact should preserve support snapshot")
        .row(crate::declaration::UiDeclarationSupportRowSchemaKind::MeasurementPolicy)
        .and_then(|row| row.declared_measurement_policy_posture())
        .cloned()
        .expect("control artifact should preserve measurement policy posture");
    let origin = app
        .graph()
        .touches()
        .declaration_change_receipt(artifact)
        .expect("success parity path should derive declaration change touch");
    let touch = app
        .graph()
        .touches()
        .from_node(
            origin,
            UiGraphTouchTiming::PostMutation,
            graph_node_identity,
            UiGraphTouchAspects::new().measurement(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("success parity path should derive measurement touch");

    let mut admission_target = UiAdmissionTarget::graph_node(
        graph_node_identity,
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    );
    if let Some(report) = bundle.host_capability_report() {
        admission_target = admission_target.with_host_capability_report(report.clone());
    }
    let authority = bundle
        .query_authority()
        .expect("success parity path should carry Query authority");
    admission_target = admission_target
        .with_query_prerequisites_from_query_authority(authority)
        .expect("query projection consumption should bind prerequisites");

    let selected = app
        .admission()
        .select_obligations_for_target(&touch, admission_target);
    let measurement_admission = app
        .admission()
        .admit_measurement_requirement(&selected)
        .expect("success parity path should admit measurement requirement");
    let query_eligibility = app
        .admission()
        .admit_query_measurement_eligibility_from_query_authority(
            &selected,
            &measurement_admission,
            authority.clone(),
        )
        .expect("success parity path should admit query measurement eligibility");
    let projection_receipt = query_eligibility
        .projection_fact_receipt()
        .cloned()
        .expect("success parity path should yield query projection receipt");

    let mut evidence_inputs = vec![MeasurementEvidenceInput::query_projection_fact(
        &projection_receipt,
    )];
    if let Some(report) = bundle.host_capability_report() {
        evidence_inputs.push(MeasurementEvidenceInput::host_capability_report(report));
    }
    evidence_inputs.extend(
        bundle
            .host_measurement_results()
            .iter()
            .map(MeasurementEvidenceInput::host_measurement_result),
    );

    let basis = admit_measurement_basis(
        artifact.identity().clone(),
        graph_node_identity,
        app.graph_snapshot().world_profile().clone(),
        measurement_admission.selected_support_authority_generation(),
        &policy,
        &evidence_inputs,
    );

    project_measurement_inspection_view(measurement_support_report(artifact), Some(&basis))
}

fn host_measurement_package() -> WorthUiDslPackage {
    WorthUiDslPackage::named("worth-ui.phase11.measurement-inspection").with_semantic_artifact_spec(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("workflow_editor.control.measurement"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/measurement_inspection.wui", 0),
        )
        .with_structural_token(UiDslStructuralToken::new("control:save"))
        .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
        .with_posture_token(UiDslPostureToken::new(
            "measurement:evidence:font-metrics-required",
        )),
    )
}

pub(super) fn query_measurement_package() -> WorthUiDslPackage {
    WorthUiDslPackage::named("worth-ui.phase11.measurement-inspection.query")
        .with_semantic_artifact_spec(
            UiDslSemanticArtifactSpec::new(
                UiDslSemanticKey::new("workflow_editor.control.measurement"),
                UiDslSemanticFamily::Control,
                UiDslSourceProvenance::file_authored("app/measurement_inspection.wui", 0),
            )
            .with_structural_token(UiDslStructuralToken::new("control:save"))
            .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
            .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
            .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
            .with_posture_token(UiDslPostureToken::new(
                "measurement:evidence:font-metrics-required",
            )),
        )
}

fn runtime_basis(
    runtime_key: &str,
    declaration_identity: &crate::declaration::UiDeclarationIdentity,
) -> UiRuntimeInstanceBasisAdmission {
    UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
        declaration_identity,
        UiRuntimeDataInstanceKeyToken::new(Arc::<str>::from(runtime_key)),
    )
    .expect("typed runtime basis key should admit")
}

fn control_artifact(app: &WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/measurement_inspection.wui"
        })
        .expect("control artifact should exist")
}

fn root_page_artifact(app: &WorthUiApp) -> &UiDeclarationArtifact {
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

fn measurement_support_report(artifact: &UiDeclarationArtifact) -> UiInspectionSupportReport {
    let rows = artifact
        .support_snapshot()
        .expect("measurement inspection parity should keep declaration support snapshot")
        .inspection_rows(UiInspectionScope::Measurement);
    UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Measurement, rows.as_ref())
}
