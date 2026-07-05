use std::sync::Arc;
use worth_ui::facade::admission::{
    UiAdmissionAggregation, UiAdmissionFamily, UiAdmissionTarget, UiAdmissionWorld,
    UiLegalityPosture, UiLegalityReason, UiSupportPosture, UiSupportReason,
};
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;

use worth_ui::facade::graph::{
    resolve_runtime_current_snapshot_basis, snapshot_resolution_report, ForgeQuerySessionLabel,
    ForgeQuerySnapshotIdentity, QueryExternalIdentityToken, SchemaBasisDigest, UiGraphWorldProfile,
};
use worth_ui::facade::inspection::UiInspectionAdmissionPosture;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryBindingSubsystem, WorthUiQueryCausalExplanationLane,
    WorthUiQueryInspectionLane, WorthUiQueryProjectionConsumptionLane,
};

#[test]
fn admission_report_keeps_support_truth_separate_from_legality_truth() {
    let query_world_profile = query_snapshot_world_profile();
    let app = WorthUi::app()
        .with_graph_world_profile(query_world_profile.clone())
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.report")
                .with_semantic_artifact_spec(admitted_control_spec())
                .with_semantic_artifact_spec(advisory_control_spec())
                .with_semantic_artifact_spec(deferred_region_spec())
                .with_semantic_artifact_spec(diagnostic_only_surface_spec()),
        )
        .freeze();
    let foreign_app = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.admission.foreign")
                .with_semantic_artifact_spec(foreign_control_spec()),
        )
        .freeze();
    let boundary = app.admission();
    let admitted = artifact_from_file_provenance(&app, "app/admission_report.wui", 0);
    let advisory = artifact_from_file_provenance(&app, "app/admission_report.wui", 1);
    let deferred = artifact_from_file_provenance(&app, "app/admission_report.wui", 2);
    let diagnostic_only = artifact_from_file_provenance(&app, "app/admission_report.wui", 3);
    let foreign = artifact_from_file_provenance(&foreign_app, "app/admission_foreign.wui", 0);
    let admitted_report = boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity(&app, admitted),
        UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
    ));
    let advisory_report = boundary.report(
        UiAdmissionTarget::graph_node(
            graph_node_identity(&app, advisory),
            UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        )
        .with_query_prerequisites(query_prerequisites(
            &query_world_profile,
            worth_ui::facade::admission::UiAdmissionQueryBasis::GraphAligned,
        )),
    );
    let unsupported_report = boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity(&foreign_app, foreign),
        UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
    ));
    let deferred_report = boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity(&app, deferred),
        UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
    ));
    let diagnostic_only_report = boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity(&app, diagnostic_only),
        UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
    ));
    let observed_world =
        UiAdmissionWorld::from_graph_world_profile(UiGraphWorldProfile::preview_session_label(
            ForgeQuerySessionLabel::scoped_strs("worth-ui", ["preview", "report"])
                .expect("preview label should admit"),
        ));
    let wrong_world_report = boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity(&app, admitted),
        observed_world.clone(),
    ));

    assert_eq!(
        admitted_report.aggregation(),
        UiAdmissionAggregation::Admitted
    );
    assert_eq!(
        admitted_report.support_snapshot().posture(),
        &UiSupportPosture::Supported {
            family: UiAdmissionFamily::TouchMeaning,
            world: UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        }
    );
    assert_eq!(
        admitted_report.inspection_posture(),
        UiInspectionAdmissionPosture::Admitted
    );
    assert_eq!(
        admitted_report
            .legality_decision()
            .expect("admitted report should retain legality truth")
            .posture(),
        UiLegalityPosture::Admitted
    );

    assert_eq!(
        advisory_report.aggregation(),
        UiAdmissionAggregation::AdmittedWithAdvisory
    );
    assert_eq!(
        advisory_report.inspection_posture(),
        UiInspectionAdmissionPosture::AdmittedWithAdvisory
    );
    assert_eq!(
        advisory_report
            .legality_decision()
            .expect("advisory report should retain legality truth")
            .posture(),
        UiLegalityPosture::AdmittedWithAdvisory(
            UiLegalityReason::QueryBindingRequiresLaterRuntimeLane
        )
    );

    assert_eq!(
        unsupported_report.aggregation(),
        UiAdmissionAggregation::Unsupported
    );
    assert_eq!(
        unsupported_report.support_snapshot().posture(),
        &UiSupportPosture::Unsupported {
            family: UiAdmissionFamily::TouchMeaning,
            reason: UiSupportReason::TargetOutsideAdmissionBoundary,
            world: UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        }
    );
    assert_eq!(
        unsupported_report.inspection_posture(),
        UiInspectionAdmissionPosture::Unsupported
    );
    assert!(
        unsupported_report.legality_decision().is_none(),
        "support-blocked report must not carry legality truth"
    );

    assert_eq!(
        deferred_report.aggregation(),
        UiAdmissionAggregation::Deferred
    );
    assert_eq!(
        deferred_report.support_snapshot().posture(),
        &UiSupportPosture::Deferred {
            family: UiAdmissionFamily::TouchMeaning,
            expected_in:
                worth_ui::facade::declaration::UiDeclarationSupportMilestoneExpectation::Milestone32,
            world: UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        }
    );
    assert_eq!(
        deferred_report.inspection_posture(),
        UiInspectionAdmissionPosture::Deferred
    );
    assert!(
        deferred_report.legality_decision().is_none(),
        "deferred report must stop before legality"
    );

    assert_eq!(
        diagnostic_only_report.aggregation(),
        UiAdmissionAggregation::DiagnosticOnly
    );
    assert_eq!(
        diagnostic_only_report.support_snapshot().posture(),
        &UiSupportPosture::DiagnosticOnly {
            family: UiAdmissionFamily::TouchMeaning,
            world: UiAdmissionWorld::from_graph_world_profile(query_world_profile.clone()),
        }
    );
    assert_eq!(
        diagnostic_only_report.inspection_posture(),
        UiInspectionAdmissionPosture::DiagnosticOnly
    );
    assert!(
        diagnostic_only_report.legality_decision().is_none(),
        "diagnostic-only report must stop before legality"
    );

    assert_eq!(
        wrong_world_report.aggregation(),
        UiAdmissionAggregation::WrongWorld
    );
    assert_eq!(
        wrong_world_report.support_snapshot().posture(),
        &UiSupportPosture::WrongWorld {
            family: UiAdmissionFamily::TouchMeaning,
            expected: UiAdmissionWorld::from_graph_world_profile(query_world_profile),
            observed: observed_world,
        }
    );
    assert_eq!(
        admitted_report.support_snapshot().posture().family(),
        wrong_world_report.support_snapshot().posture().family(),
    );
    assert_eq!(
        wrong_world_report.inspection_posture(),
        UiInspectionAdmissionPosture::WrongWorld
    );
    assert!(
        wrong_world_report.legality_decision().is_none(),
        "wrong-world report must stop before legality"
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
        UiDslSourceProvenance::file_authored("app/admission_report.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:plain"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn advisory_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.query"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_report.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("control:query"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn deferred_region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.deferred"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/admission_report.wui", 2),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn diagnostic_only_surface_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.diagnostics.graph"),
        UiDslSemanticFamily::DiagnosticSurface,
        UiDslSourceProvenance::file_authored("app/admission_report.wui", 3),
    )
    .with_structural_token(UiDslStructuralToken::new("diagnostic-surface:graph"))
}

fn foreign_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.foreign"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/admission_foreign.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:foreign"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn query_snapshot_world_profile() -> UiGraphWorldProfile {
    let snapshot_identity = ForgeQuerySnapshotIdentity::admit_external_token(
        QueryExternalIdentityToken::new(Arc::<str>::from("snapshot:admission-report")),
    );
    let basis = resolve_runtime_current_snapshot_basis(
        snapshot_identity.evidence_identity(),
        SchemaBasisDigest::from_domain_parts(
            &["worth-ui.phase5", "admission", "report"]
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
    query_basis: worth_ui::facade::admission::UiAdmissionQueryBasis,
) -> worth_ui::facade::query_binding::WorthUiQueryPrerequisiteEvidence {
    let UiGraphWorldProfile::QuerySnapshotBasis {
        basis,
        resolution_report,
    } = world_profile
    else {
        panic!("query report proofs require query snapshot worlds");
    };

    WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .assemble(
            basis.clone(),
            resolution_report.clone(),
            match query_basis {
                worth_ui::facade::admission::UiAdmissionQueryBasis::GraphAligned => {
                    WorthUiQueryBasisPosture::GraphAligned
                }
                worth_ui::facade::admission::UiAdmissionQueryBasis::WrongWorldProjection => {
                    WorthUiQueryBasisPosture::WrongWorldProjection
                }
                worth_ui::facade::admission::UiAdmissionQueryBasis::RebindRequired => {
                    WorthUiQueryBasisPosture::RebindRequired
                }
                worth_ui::facade::admission::UiAdmissionQueryBasis::StaleReceipt => {
                    WorthUiQueryBasisPosture::StaleReceipt
                }
                worth_ui::facade::admission::UiAdmissionQueryBasis::AmbiguousSources => {
                    WorthUiQueryBasisPosture::AmbiguousSources
                }
            },
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
        .expect("query prerequisite assembly should admit")
}
