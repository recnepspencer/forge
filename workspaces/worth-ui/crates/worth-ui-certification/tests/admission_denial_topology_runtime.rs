use std::sync::Arc;
use worth_ui::facade::admission::{
    UiAdmissionAggregation, UiAdmissionHostCapability, UiAdmissionQueryBasis,
    UiAdmissionSelectionBudget, UiAdmissionStaleEvidence, UiAdmissionTarget, UiAdmissionWorld,
    UiLegalityPosture, UiLegalityReason,
};
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    resolve_runtime_current_snapshot_basis, snapshot_resolution_report, ForgeQuerySnapshotIdentity,
    QueryExternalIdentityToken, SchemaBasisDigest, UiGraphWorldProfile,
};
use worth_ui::facade::inspection::UiInspectionAdmissionPosture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_host_contract::{WorthUiHostCapabilityReport, WorthUiHostContract};
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryBindingSubsystem, WorthUiQueryCausalExplanationLane,
    WorthUiQueryInspectionLane, WorthUiQueryProjectionConsumptionLane,
};

#[test]
fn query_basis_and_host_capability_denials_depend_on_attached_runtime_lanes() {
    let query_world_profile = query_snapshot_world_profile();
    let app = WorthUi::app()
        .with_graph_world_profile(query_world_profile.clone())
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.denials")
                .with_semantic_artifact_spec(admitted_control_spec())
                .with_semantic_artifact_spec(query_bound_control_spec())
                .with_semantic_artifact_spec(service_bound_control_spec()),
        )
        .freeze();
    let boundary = app.admission();
    let admitted = artifact_from_file_provenance(&app, "app/admission_denials.wui", 0);
    let query_bound = artifact_from_file_provenance(&app, "app/admission_denials.wui", 1);
    let service_bound = artifact_from_file_provenance(&app, "app/admission_denials.wui", 2);

    let query_basis_denied = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, query_bound),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_query_prerequisites(query_prerequisites(
            &query_world_profile,
            UiAdmissionQueryBasis::WrongWorldProjection,
        )),
    );
    let query_basis_ignored = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, admitted),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_query_prerequisites(query_prerequisites(
            &query_world_profile,
            UiAdmissionQueryBasis::WrongWorldProjection,
        )),
    );
    let host_capability_denied = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, service_bound),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
            WorthUiHostContract::headless(),
        )),
    );
    let host_capability_ignored = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, admitted),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
            WorthUiHostContract::headless(),
        )),
    );

    assert_eq!(
        query_basis_denied.inspection_posture(),
        UiInspectionAdmissionPosture::WrongQueryBasis
    );
    assert_eq!(
        query_basis_denied
            .legality_decision()
            .expect("query-bound declaration should retain denial payload")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::WrongQueryBasis {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: UiAdmissionQueryBasis::WrongWorldProjection,
        })
    );
    assert_eq!(
        query_basis_ignored.aggregation(),
        UiAdmissionAggregation::Admitted
    );
    assert_eq!(
        query_basis_ignored.inspection_posture(),
        UiInspectionAdmissionPosture::Admitted
    );

    assert_eq!(
        host_capability_denied.inspection_posture(),
        UiInspectionAdmissionPosture::WrongHostCapability
    );
    assert_eq!(
        host_capability_denied
            .legality_decision()
            .expect("service-bound declaration should retain denial payload")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::WrongHostCapability {
            required: UiAdmissionHostCapability::Available,
            observed: UiAdmissionHostCapability::Missing,
        })
    );
    assert_eq!(
        host_capability_ignored.aggregation(),
        UiAdmissionAggregation::Admitted
    );
    assert_eq!(
        host_capability_ignored.inspection_posture(),
        UiInspectionAdmissionPosture::Admitted
    );
}

#[test]
fn denial_reports_retain_full_payloads_for_query_staleness_and_budget() {
    let query_world_profile = query_snapshot_world_profile();
    let app = WorthUi::app()
        .with_graph_world_profile(query_world_profile.clone())
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.denials.payloads")
                .with_semantic_artifact_spec(admitted_control_spec())
                .with_semantic_artifact_spec(query_bound_control_spec()),
        )
        .freeze();
    let boundary = app.admission();
    let admitted = artifact_from_file_provenance(&app, "app/admission_denials.wui", 0);
    let query_bound = artifact_from_file_provenance(&app, "app/admission_denials.wui", 1);

    let rebind_required_report = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, query_bound),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_query_prerequisites(query_prerequisites(
            &query_world_profile,
            UiAdmissionQueryBasis::RebindRequired,
        )),
    );
    let stale_report = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, query_bound),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_query_prerequisites(query_prerequisites(
            &query_world_profile,
            UiAdmissionQueryBasis::StaleReceipt,
        )),
    );
    let ambiguous_report = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, query_bound),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_query_prerequisites(query_prerequisites(
            &query_world_profile,
            UiAdmissionQueryBasis::AmbiguousSources,
        )),
    );
    let budget_exceeded_report = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, admitted),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_selection_budget(UiAdmissionSelectionBudget::ordinary_lane_budget(0)),
    );

    assert_eq!(
        rebind_required_report.inspection_posture(),
        UiInspectionAdmissionPosture::RebindRequired
    );
    assert_eq!(
        rebind_required_report
            .legality_decision()
            .expect("rebind-required report should retain legality truth")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::RebindRequired {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: UiAdmissionQueryBasis::RebindRequired,
        })
    );

    assert_eq!(
        stale_report.inspection_posture(),
        UiInspectionAdmissionPosture::Stale
    );
    assert_eq!(
        stale_report
            .legality_decision()
            .expect("stale report should retain legality truth")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::Stale {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: UiAdmissionQueryBasis::StaleReceipt,
            evidence: UiAdmissionStaleEvidence::QueryReceiptExpired,
        })
    );

    assert_eq!(
        ambiguous_report.inspection_posture(),
        UiInspectionAdmissionPosture::Ambiguous
    );
    assert_eq!(
        ambiguous_report
            .legality_decision()
            .expect("ambiguous report should retain legality truth")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::Ambiguous {
            required_query_basis: Some(UiAdmissionQueryBasis::GraphAligned),
            observed_query_basis: Some(UiAdmissionQueryBasis::AmbiguousSources),
            required_host_capability: None,
            observed_host_capability: None,
        })
    );

    assert_eq!(
        budget_exceeded_report.inspection_posture(),
        UiInspectionAdmissionPosture::BudgetExceeded
    );
    assert_eq!(
        budget_exceeded_report
            .legality_decision()
            .expect("budget-exceeded report should retain legality truth")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::BudgetExceeded {
            budget: UiAdmissionSelectionBudget::ordinary_lane_budget(0),
            attempted_lane_cost: 1,
        })
    );
}

#[test]
fn attached_runtime_lanes_fail_closed_without_owner_bound_prerequisite_evidence() {
    let query_app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.denials.missing-query")
                .with_semantic_artifact_spec(query_bound_control_spec()),
        )
        .freeze();
    let query_artifact = artifact_from_file_provenance(&query_app, "app/admission_denials.wui", 1);
    let query_report = query_app.admission().report(UiAdmissionTarget::graph_node(
        graph_node_identity(&query_app, query_artifact),
        UiAdmissionWorld::authoritative(),
    ));

    assert_eq!(query_report.aggregation(), UiAdmissionAggregation::Denied);
    assert_eq!(
        query_report.inspection_posture(),
        UiInspectionAdmissionPosture::Unsupported
    );
    assert_eq!(
        query_report
            .legality_decision()
            .expect("query-bound report should retain legality truth")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::MissingQueryPrerequisiteEvidence)
    );

    let service_app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.denials.missing-host")
                .with_semantic_artifact_spec(service_bound_control_spec()),
        )
        .freeze();
    let service_artifact =
        artifact_from_file_provenance(&service_app, "app/admission_denials.wui", 2);
    let service_report = service_app
        .admission()
        .report(UiAdmissionTarget::graph_node(
            graph_node_identity(&service_app, service_artifact),
            UiAdmissionWorld::authoritative(),
        ));

    assert_eq!(service_report.aggregation(), UiAdmissionAggregation::Denied);
    assert_eq!(
        service_report.inspection_posture(),
        UiInspectionAdmissionPosture::Unsupported
    );
    assert_eq!(
        service_report
            .legality_decision()
            .expect("service-bound report should retain legality truth")
            .posture(),
        UiLegalityPosture::Denied(UiLegalityReason::MissingHostCapabilityReport)
    );
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
            panic!("expected declaration artifact for {module_path}#{declaration_index}")
        })
}

fn graph_node_identity(
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

fn admitted_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.plain"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_denials.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:plain"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn query_bound_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.query"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_denials.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("control:query"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn service_bound_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.service"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_denials.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("control:service"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

fn query_snapshot_world_profile() -> UiGraphWorldProfile {
    let snapshot_identity = ForgeQuerySnapshotIdentity::admit_external_token(
        QueryExternalIdentityToken::new(Arc::<str>::from("snapshot:admission-denials")),
    );
    let basis = resolve_runtime_current_snapshot_basis(
        snapshot_identity.evidence_identity(),
        SchemaBasisDigest::from_domain_parts(
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

fn query_prerequisites(
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
