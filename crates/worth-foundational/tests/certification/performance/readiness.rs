use std::collections::BTreeSet;
use std::fmt::Debug;
use std::path::Path;
use worth_foundational::{
    performance_api::{
        stronger_lane::readiness, FoundationalPerformancePublicLane,
        FoundationalPerformancePublicSurfaceEntry,
    },
    FoundationalPerformanceCertifiedSurface, FoundationalPerformanceCompileFailBoundary,
    FoundationalPerformanceHarnessExpansionPoint, FoundationalPerformanceMilestone8PhaseGate,
    FoundationalPerformanceProductionReadinessCertified,
    FoundationalPerformanceProductionTestReadyArtifact,
    FoundationalPerformancePublicSurfaceDocumentationCoverage,
    FoundationalPerformanceRuntimeAdoptionPressure, FoundationalPerformanceRuntimeAssumption,
    FoundationalPerformanceRuntimeNonAssumption, FoundationalPerformanceSyntheticRuntimePressure,
};

fn accepts_performance_readiness_artifact(_: &FoundationalPerformanceProductionTestReadyArtifact) {}

fn accepts_performance_readiness_proof(
    _: &worth_proof::Proof<
        FoundationalPerformanceProductionReadinessCertified,
        worth_foundational::FoundationalPerformanceProductionReadinessAuthority,
    >,
) {
}

#[test]
fn production_readiness_artifact_carries_complete_machine_checkable_inventory() {
    let readiness_artifact =
        readiness::certify_foundational_performance_milestone8_production_test_readiness();
    let report = readiness::require_foundational_performance_milestone8_production_test_readiness(
        &readiness_artifact,
    );

    accepts_performance_readiness_artifact(&readiness_artifact);
    accepts_performance_readiness_proof(readiness_artifact.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness_artifact.strong_basis().value().milestone(),
        "worth-foundational.milestone-8"
    );

    assert_exact_inventory(
        "certified surfaces",
        report.certified_surfaces(),
        &[
            FoundationalPerformanceCertifiedSurface::PrimitiveAndCategoryLaw,
            FoundationalPerformanceCertifiedSurface::ClaimBoundaryAndEvidenceStrengthLaw,
            FoundationalPerformanceCertifiedSurface::LayoutIntentAndRepresentationFreedom,
            FoundationalPerformanceCertifiedSurface::PolicyAdmissionAndBudgetLaw,
            FoundationalPerformanceCertifiedSurface::CanonicalBundleAndCounterReceiptLaw,
            FoundationalPerformanceCertifiedSurface::ReportAttachmentAndMaterializationLaw,
            FoundationalPerformanceCertifiedSurface::CertifiedBundleAndReadmissionLaw,
        ],
    );
    assert_exact_inventory(
        "compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            FoundationalPerformanceCompileFailBoundary::PrimitiveFamiliesAndCommonPathBoundaries,
            FoundationalPerformanceCompileFailBoundary::ClaimLaneBoundaries,
            FoundationalPerformanceCompileFailBoundary::LayoutAttachmentBoundaries,
            FoundationalPerformanceCompileFailBoundary::PolicyPreExecutionBoundaries,
            FoundationalPerformanceCompileFailBoundary::BundleAndCounterReceiptLoweringBoundaries,
            FoundationalPerformanceCompileFailBoundary::ReportMaterializationBoundaries,
            FoundationalPerformanceCompileFailBoundary::CertifiedBundleAndReadmissionProofLane,
            FoundationalPerformanceCompileFailBoundary::PerformanceReadinessRequiresCertifiedArtifact,
            FoundationalPerformanceCompileFailBoundary::PerformanceReadinessAuthorityCannotBeMinted,
            FoundationalPerformanceCompileFailBoundary::GroupedStrongerLaneRequiresCertifiedReadiness,
        ],
    );
    assert_exact_inventory(
        "harness expansion points",
        report.harness_expansion_points(),
        &[FoundationalPerformanceHarnessExpansionPoint::PolicyUnavailableSectionMatrix],
    );
    assert_exact_inventory(
        "runtime adoption pressures",
        report.runtime_adoption_pressures(),
        &[
            FoundationalPerformanceRuntimeAdoptionPressure::CrossCrateMeaningParityMatrix,
            FoundationalPerformanceRuntimeAdoptionPressure::CertifiedBundleSourceCompatibilityMatrix,
        ],
    );
}

#[test]
fn production_readiness_binds_surfaces_to_real_tests_and_exact_public_surface_inventory() {
    let report = readiness::foundational_performance_milestone8_readiness_report();

    for evidence in report.certified_surface_evidence() {
        assert!(report.certified_surfaces().contains(&evidence.surface()));
        assert!(report
            .synthetic_pressures()
            .contains(&evidence.hostile_pressure()));
        assert!(report
            .compile_fail_boundaries()
            .contains(&evidence.compile_fail_boundary()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
        assert!(crate_root_path(evidence.compile_fail_evidence_path()).is_file());
    }

    let paths: BTreeSet<_> = report
        .public_surface_inventory()
        .iter()
        .map(FoundationalPerformancePublicSurfaceEntry::path)
        .collect();
    let doc_paths: BTreeSet<_> = report
        .documentation_surface_inventory()
        .iter()
        .copied()
        .collect();
    let coverage_surface_paths: BTreeSet<_> = report
        .public_surface_documentation_coverage()
        .iter()
        .map(FoundationalPerformancePublicSurfaceDocumentationCoverage::public_surface_path)
        .collect();
    let exact_doc_coverage: BTreeSet<_> = report
        .public_surface_documentation_coverage()
        .iter()
        .map(|row| (row.public_surface_path(), row.primary_documentation_path()))
        .collect();
    let stronger_lane_count = report
        .public_surface_inventory()
        .iter()
        .filter(|entry| entry.lane() == FoundationalPerformancePublicLane::StrongerLane)
        .count();

    assert_eq!(report.public_surface_inventory().len(), 9);
    assert_eq!(stronger_lane_count, 3);
    assert!(paths.contains("worth_foundational::performance_api::common_path"));
    assert!(paths.contains("worth_foundational::performance_api::stronger_lane::certified"));
    assert!(paths.contains("worth_foundational::performance_api::stronger_lane::readiness"));
    assert!(doc_paths.contains("docs/performance/README.md"));
    assert!(doc_paths.contains("docs/performance/canonical-bundles-and-comparison.md"));
    assert!(doc_paths.contains("docs/performance/counter-backed-performance-receipts.md"));
    assert!(
        doc_paths.contains("docs/performance/performance-report-planning-and-materialization.md")
    );
    assert!(doc_paths.contains("docs/performance/certified-and-readmitted-performance-bundles.md"));
    assert_eq!(paths, coverage_surface_paths);
    assert_eq!(
        exact_doc_coverage,
        BTreeSet::from([
            (
                "worth_foundational::performance_api::common_path",
                "docs/performance/common-performance-claims-and-layout-intent.md",
            ),
            (
                "worth_foundational::performance_api::lower_lane::basis",
                "docs/performance/canonical-bundles-and-comparison.md",
            ),
            (
                "worth_foundational::performance_api::lower_lane::policy",
                "docs/performance/policy-admission-receipts.md",
            ),
            (
                "worth_foundational::performance_api::lower_lane::receipts",
                "docs/performance/counter-backed-performance-receipts.md",
            ),
            (
                "worth_foundational::performance_api::lower_lane::reports",
                "docs/performance/performance-report-planning-and-materialization.md",
            ),
            (
                "worth_foundational::performance_api::lower_lane",
                "docs/performance/grouped-public-lanes-and-stronger-readiness.md",
            ),
            (
                "worth_foundational::performance_api::stronger_lane",
                "docs/performance/grouped-public-lanes-and-stronger-readiness.md",
            ),
            (
                "worth_foundational::performance_api::stronger_lane::certified",
                "docs/performance/certified-and-readmitted-performance-bundles.md",
            ),
            (
                "worth_foundational::performance_api::stronger_lane::readiness",
                "docs/performance/performance-production-readiness.md",
            ),
        ]),
        "public surface documentation coverage drifted"
    );
    assert!(crate_root_path(report.public_surface_evidence_path()).is_file());
    assert!(crate_root_path(report.public_surface_compile_fail_path()).is_file());
    for path in report.documentation_surface_inventory() {
        assert!(crate_root_path(path).is_file());
    }
    for row in report.public_surface_documentation_coverage() {
        assert!(crate_root_path(row.primary_documentation_path()).is_file());
    }
}

#[test]
fn production_readiness_names_runtime_boundary_and_phase_order_exactly() {
    let report = readiness::foundational_performance_milestone8_readiness_report();

    assert_exact_inventory(
        "runtime assumptions",
        report.assumptions(),
        &[
            FoundationalPerformanceRuntimeAssumption::WORTHProofAuthorityLaneRemainsAvailable,
            FoundationalPerformanceRuntimeAssumption::ProfileLawRemainsAuthorityForReportElision,
            FoundationalPerformanceRuntimeAssumption::PhaseEvidencePathsRemainOwnedWithinFoundational,
        ],
    );
    assert_exact_inventory(
        "runtime non-assumptions",
        report.non_assumptions(),
        &[FoundationalPerformanceRuntimeNonAssumption::WorkspaceWideTelemetryEngineIsOwnedHere],
    );
    assert_exact_inventory("residual debt", report.residual_debt(), &[]);
    assert_eq!(
        report
            .phase_gates()
            .iter()
            .map(|evidence| evidence.gate())
            .collect::<Vec<_>>(),
        vec![
            FoundationalPerformanceMilestone8PhaseGate::PrimitiveAndCategoryLaw,
            FoundationalPerformanceMilestone8PhaseGate::ClaimBoundaryAndEvidenceStrengthLaw,
            FoundationalPerformanceMilestone8PhaseGate::LayoutIntentAccessAndAllocationLaw,
            FoundationalPerformanceMilestone8PhaseGate::RuntimePolicyBudgetAndFallbackLaw,
            FoundationalPerformanceMilestone8PhaseGate::CanonicalBasisCounterAndComparisonLaw,
            FoundationalPerformanceMilestone8PhaseGate::AttachmentMaterializationAndBundleLaw,
            FoundationalPerformanceMilestone8PhaseGate::ProductionReadiness,
            FoundationalPerformanceMilestone8PhaseGate::FeatureDocsCrateDocIntegrationAndPublicationClosure,
        ]
    );
    for evidence in report.phase_gates() {
        assert!(crate_root_path(evidence.evidence_path()).exists());
    }
}

#[test]
fn production_readiness_hostile_pressures_inventory_stays_exact() {
    let report = readiness::foundational_performance_milestone8_readiness_report();

    assert_exact_inventory(
        "synthetic pressures",
        report.synthetic_pressures(),
        &[
            FoundationalPerformanceSyntheticRuntimePressure::PrimitiveFamilyNonSubstitution,
            FoundationalPerformanceSyntheticRuntimePressure::ClaimStrengthAndLaneCollapseRejection,
            FoundationalPerformanceSyntheticRuntimePressure::RepresentationEquivalenceOverclaimRejection,
            FoundationalPerformanceSyntheticRuntimePressure::PreExecutionMasqueradeRejection,
            FoundationalPerformanceSyntheticRuntimePressure::CanonicalCounterLoweringRejection,
            FoundationalPerformanceSyntheticRuntimePressure::HiddenSupportExpansionRejection,
            FoundationalPerformanceSyntheticRuntimePressure::CertifiedProofLaneBoundary,
            FoundationalPerformanceSyntheticRuntimePressure::GroupedStrongerLaneBoundary,
        ],
    );
}

#[test]
fn production_readiness_names_runtime_adoption_proof_paths_exactly() {
    let report = readiness::foundational_performance_milestone8_readiness_report();

    let exact_runtime_adoption_coverage: BTreeSet<_> = report
        .runtime_adoption_pressure_evidence()
        .iter()
        .map(|row| (row.pressure(), row.evidence_path()))
        .collect();

    assert_eq!(
        exact_runtime_adoption_coverage,
        BTreeSet::from([
            (
                FoundationalPerformanceRuntimeAdoptionPressure::CrossCrateMeaningParityMatrix,
                "tests/certification/performance/runtime_parity.rs",
            ),
            (
                FoundationalPerformanceRuntimeAdoptionPressure::CertifiedBundleSourceCompatibilityMatrix,
                "tests/certification/performance/runtime_parity.rs",
            ),
        ]),
        "runtime adoption pressure coverage drifted"
    );
    for row in report.runtime_adoption_pressure_evidence() {
        assert!(crate_root_path(row.evidence_path()).is_file());
    }
}

fn crate_root_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn assert_exact_inventory<T>(name: &str, actual: &[T], expected: &[T])
where
    T: Copy + Debug + Ord,
{
    let actual_set: BTreeSet<_> = actual.iter().copied().collect();
    let expected_set: BTreeSet<_> = expected.iter().copied().collect();

    assert_eq!(
        actual.len(),
        expected.len(),
        "{name} contains duplicate rows"
    );
    assert_eq!(
        actual_set, expected_set,
        "{name} changed without updating readiness certification"
    );
}
