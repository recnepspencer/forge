use worth_foundational::facade::{
    certify_foundational_boundary_artifact_milestone4_production_test_readiness,
    certify_foundational_boundary_evidence_milestone7_production_test_readiness,
    certify_foundational_performance_milestone8_production_test_readiness,
    FoundationalBoundaryArtifactCompileFailBoundary,
    FoundationalBoundaryEvidenceCompileFailBoundary, FoundationalPerformanceCompileFailBoundary,
};

use super::{
    milestone_nine_five_forbidden_fallback_closeout_report,
    MilestoneNineFiveForbiddenFallbackNeedle, MilestoneNineFiveForbiddenFallbackSurface,
};

#[test]
fn forbidden_fallback_closeout_report_keeps_ordinary_runtime_backed_read_surface_at_exact_zero() {
    let report = milestone_nine_five_forbidden_fallback_closeout_report();

    assert_eq!(report.total_occurrence_count(), 0);
    assert_eq!(report.rows().len(), 6);
    for surface in [
        MilestoneNineFiveForbiddenFallbackSurface::OrdinaryRuntimeBackedReadBootstrap,
        MilestoneNineFiveForbiddenFallbackSurface::OrdinaryRuntimeBackedReadBootstrapSupport,
    ] {
        for forbidden_needle in [
            MilestoneNineFiveForbiddenFallbackNeedle::ReadLiveArtifactBinding,
            MilestoneNineFiveForbiddenFallbackNeedle::ReadLiveArtifactBundle,
            MilestoneNineFiveForbiddenFallbackNeedle::BridgeBackedRuntimeBuilder,
        ] {
            let row = report
                .rows()
                .iter()
                .find(|row| row.surface() == surface && row.forbidden_needle() == forbidden_needle)
                .expect("closeout row should exist for every surface/forbidden-needle pair");
            assert_eq!(row.occurrence_count(), 0);
            assert!(!row.row_digest().is_empty());
        }
    }
    assert!(!report.report_digest().is_empty());
}

#[test]
fn forbidden_fallback_closeout_report_carries_shared_foundational_boundaries() {
    let report = milestone_nine_five_forbidden_fallback_closeout_report();

    assert!(report.boundary_artifact_compile_fail_boundaries().contains(
        &FoundationalBoundaryArtifactCompileFailBoundary::BoundaryArtifactReadinessRequiresCertifiedArtifact
    ));
    assert!(report.boundary_evidence_compile_fail_boundaries().contains(
        &FoundationalBoundaryEvidenceCompileFailBoundary::BoundaryEvidenceReadinessRequiresCertifiedArtifact
    ));
    assert!(report.performance_compile_fail_boundaries().contains(
        &FoundationalPerformanceCompileFailBoundary::PerformanceReadinessRequiresCertifiedArtifact
    ));
}

#[test]
fn forbidden_fallback_closeout_foundational_ready_artifacts_remain_certifiable() {
    let boundary_artifact =
        certify_foundational_boundary_artifact_milestone4_production_test_readiness();
    let boundary_evidence =
        certify_foundational_boundary_evidence_milestone7_production_test_readiness();
    let performance = certify_foundational_performance_milestone8_production_test_readiness();

    assert!(boundary_artifact.payload().passes_readiness_checklist());
    assert!(boundary_evidence.payload().passes_readiness_checklist());
    assert!(performance.payload().passes_readiness_checklist());
}
