use std::collections::BTreeSet;
use std::fmt::Debug;
use std::path::Path;
use worth_foundational::{
    boundary_evidence_api::{stronger_lane::readiness, BoundaryEvidencePublicLane},
    FoundationalBoundaryEvidenceCertifiedSurface, FoundationalBoundaryEvidenceCompileFailBoundary,
    FoundationalBoundaryEvidenceGoldenArtifact, FoundationalBoundaryEvidenceHarnessExpansionPoint,
    FoundationalBoundaryEvidenceMilestone7PhaseGate,
    FoundationalBoundaryEvidenceProductionReadinessCertified,
    FoundationalBoundaryEvidenceProductionTestReadyArtifact,
    FoundationalBoundaryEvidencePropertySeed, FoundationalBoundaryEvidenceResidualDebt,
    FoundationalBoundaryEvidenceRuntimeAssumption,
    FoundationalBoundaryEvidenceRuntimeNonAssumption,
    FoundationalBoundaryEvidenceSyntheticRuntimePressure,
};

fn accepts_boundary_evidence_readiness_artifact(
    _: &FoundationalBoundaryEvidenceProductionTestReadyArtifact,
) {
}

fn accepts_boundary_evidence_readiness_proof(
    _: &worth_proof::Proof<
        FoundationalBoundaryEvidenceProductionReadinessCertified,
        worth_foundational::FoundationalBoundaryEvidenceProductionReadinessAuthority,
    >,
) {
}

#[test]
fn production_readiness_artifact_carries_complete_machine_checkable_inventory() {
    let readiness_artifact =
        readiness::certify_foundational_boundary_evidence_milestone7_production_test_readiness();
    let report =
        readiness::require_foundational_boundary_evidence_milestone7_production_test_readiness(
            &readiness_artifact,
        );

    accepts_boundary_evidence_readiness_artifact(&readiness_artifact);
    accepts_boundary_evidence_readiness_proof(readiness_artifact.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness_artifact.strong_basis().value().milestone(),
        "worth-foundational.milestone-7"
    );

    assert_exact_inventory(
        "certified surfaces",
        report.certified_surfaces(),
        &[
            FoundationalBoundaryEvidenceCertifiedSurface::PrimitiveCategoryAndRoleLaw,
            FoundationalBoundaryEvidenceCertifiedSurface::ProvenanceLayeringAndFreshnessLaw,
            FoundationalBoundaryEvidenceCertifiedSurface::ReceiptFamilyAndCloseoutTruth,
            FoundationalBoundaryEvidenceCertifiedSurface::LineageContinuityAndDivergence,
            FoundationalBoundaryEvidenceCertifiedSurface::SupportTruthRecoveryAndDebt,
            FoundationalBoundaryEvidenceCertifiedSurface::AttachmentMaterializationAndReadmission,
        ],
    );
    assert_exact_inventory(
        "compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            FoundationalBoundaryEvidenceCompileFailBoundary::PrimitiveNonSubstitution,
            FoundationalBoundaryEvidenceCompileFailBoundary::ProvenanceFreshnessAndArtifactBoundaries,
            FoundationalBoundaryEvidenceCompileFailBoundary::ReceiptPlanningVersusCompletedBoundarySeparation,
            FoundationalBoundaryEvidenceCompileFailBoundary::ReplayAndHistoryRecordsCannotMasquerade,
            FoundationalBoundaryEvidenceCompileFailBoundary::LineageContinuityStrengthBoundaries,
            FoundationalBoundaryEvidenceCompileFailBoundary::SupportGradeAndBasisDisclosureBoundaries,
            FoundationalBoundaryEvidenceCompileFailBoundary::AttachmentScopeAndReadmissionBoundaries,
            FoundationalBoundaryEvidenceCompileFailBoundary::BoundaryEvidenceReadinessRequiresCertifiedArtifact,
            FoundationalBoundaryEvidenceCompileFailBoundary::BoundaryEvidenceReadinessAuthorityCannotBeMinted,
            FoundationalBoundaryEvidenceCompileFailBoundary::GroupedStrongerLaneRequiresCertifiedReadiness,
        ],
    );
    assert_exact_inventory(
        "golden artifacts",
        report.golden_artifacts(),
        &[
            FoundationalBoundaryEvidenceGoldenArtifact::PrimitiveCategoryRoleAndLocalityMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::ProvenanceLayeringAndFreshnessMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::ReceiptExecutionAndCloseoutMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::LineageContinuityPromotionAndPartialityMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::SupportTruthRecoveryAndResidualDebtMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::AttachmentCanonicalDigestAndReadmissionMeaning,
        ],
    );
    assert_exact_inventory(
        "property seeds",
        report.property_seeds(),
        &[
            FoundationalBoundaryEvidencePropertySeed::PrimitiveDefinitionOrdering,
            FoundationalBoundaryEvidencePropertySeed::ProvenanceLayerAndSupportContextOrdering,
            FoundationalBoundaryEvidencePropertySeed::PlanningExecutedAndCloseoutStrength,
            FoundationalBoundaryEvidencePropertySeed::ReplayHistoryAndPromotionStrength,
            FoundationalBoundaryEvidencePropertySeed::MixedAttachmentCanonicalAndDigestParity,
        ],
    );
    assert_exact_inventory(
        "harness expansion points",
        report.harness_expansion_points(),
        &[
            FoundationalBoundaryEvidenceHarnessExpansionPoint::ReplayHistoryMasqueradeMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::RecoveryAndDegradedOperationMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::MixedAttachmentCanonicalDigestParityMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::TrustBoundaryReadmissionParityMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::GroupedPublicSurfaceLane,
        ],
    );
    assert_exact_inventory(
        "documentation surface inventory",
        report.documentation_surface_inventory(),
        &[
            "docs/README.md",
            "docs/lineage-provenance-receipts-and-support-truth/README.md",
            "docs/lineage-provenance-receipts-and-support-truth/primitive-categories-locality-and-role-postures.md",
            "docs/lineage-provenance-receipts-and-support-truth/provenance-layering-and-freshness.md",
            "docs/lineage-provenance-receipts-and-support-truth/receipts-and-closeout-truth.md",
            "docs/lineage-provenance-receipts-and-support-truth/lineage-continuity-divergence-and-promotion.md",
            "docs/lineage-provenance-receipts-and-support-truth/support-truth-recovery-and-degraded-operation.md",
            "docs/lineage-provenance-receipts-and-support-truth/attachment-materialization-canonical-participation-and-readmission.md",
            "docs/lineage-provenance-receipts-and-support-truth/grouped-public-lanes-and-stronger-readiness.md",
            "docs/lineage-provenance-receipts-and-support-truth/boundary-evidence-production-readiness.md",
        ],
    );
    for path in report.documentation_surface_inventory() {
        assert!(crate_root_path(path).is_file());
    }
}

#[test]
fn production_readiness_binds_surfaces_to_real_tests_and_exact_public_surface_inventory() {
    let report = readiness::foundational_boundary_evidence_milestone7_readiness_report();

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
        .map(|entry| entry.path())
        .collect();
    let stronger_lane_count = report
        .public_surface_inventory()
        .iter()
        .filter(|entry| entry.lane() == BoundaryEvidencePublicLane::StrongerLane)
        .count();

    assert_eq!(report.public_surface_inventory().len(), 10);
    assert_eq!(stronger_lane_count, 3);
    assert!(paths.contains("worth_foundational::boundary_evidence_api::common_path"));
    assert!(paths.contains("worth_foundational::boundary_evidence_api::stronger_lane::readmission"));
    assert!(paths.contains("worth_foundational::boundary_evidence_api::stronger_lane::readiness"));
    assert!(crate_root_path(report.public_surface_evidence_path()).is_file());
    assert!(crate_root_path(report.public_surface_compile_fail_path()).is_file());
    let crate_root_docs =
        std::fs::read_to_string(crate_root_path("src/lib.rs")).expect("crate root docs");
    assert!(crate_root_docs.contains("docs/README.md"));
    assert!(
        crate_root_docs.contains("docs/lineage-provenance-receipts-and-support-truth/README.md")
    );
}

#[test]
fn production_readiness_names_runtime_boundary_and_phase_order_exactly() {
    let report = readiness::foundational_boundary_evidence_milestone7_readiness_report();

    assert_exact_inventory(
        "runtime assumptions",
        report.assumptions(),
        &[
            FoundationalBoundaryEvidenceRuntimeAssumption::WORTHProofAuthorityLaneRemainsAvailable,
            FoundationalBoundaryEvidenceRuntimeAssumption::BoundaryArtifactAndDiagnosticMeaningRemainCertifiedDependencies,
            FoundationalBoundaryEvidenceRuntimeAssumption::CanonicalizationLawRemainsAuthorityForAttachmentParticipation,
            FoundationalBoundaryEvidenceRuntimeAssumption::ReadmissionRemainsExplicitAcrossTrustBoundaries,
        ],
    );
    assert_exact_inventory(
        "runtime non-assumptions",
        report.non_assumptions(),
        &[
            FoundationalBoundaryEvidenceRuntimeNonAssumption::RuntimeSpecificHistoryStoreLayoutOwnedHere,
            FoundationalBoundaryEvidenceRuntimeNonAssumption::ReplayDerivationUpgradesToAttestedContinuity,
            FoundationalBoundaryEvidenceRuntimeNonAssumption::SupportTruthUpgradesToAuthorityWithoutBridge,
            FoundationalBoundaryEvidenceRuntimeNonAssumption::CrossBoundaryAttachmentBundlesRemainCurrentWithoutReadmission,
        ],
    );
    assert_exact_inventory(
        "residual debt",
        report.residual_debt(),
        &[
            FoundationalBoundaryEvidenceResidualDebt::AdoptingRuntimeParityDeferred,
            FoundationalBoundaryEvidenceResidualDebt::RuntimeSpecificHistoryAndJournalTaxonomiesDeferred,
            FoundationalBoundaryEvidenceResidualDebt::RealRuntimeSupportBundlePersistenceDeferred,
        ],
    );
    assert_eq!(
        report
            .phase_gates()
            .iter()
            .map(|evidence| evidence.gate())
            .collect::<Vec<_>>(),
        vec![
            FoundationalBoundaryEvidenceMilestone7PhaseGate::PrimitiveCategoryAndRoleLaw,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::ProvenanceLayeringAndFreshnessLaw,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::ReceiptFamilyAndCloseoutTruth,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::LineageContinuityAndDivergence,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::SupportTruthRecoveryAndDegradedOperation,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::AttachmentMaterializationAndReadmission,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::ProductionReadiness,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::FeatureDocsAndCrateDocIntegration,
            FoundationalBoundaryEvidenceMilestone7PhaseGate::FeatureDocWriterCloseoutAndRegistration,
        ]
    );
    for evidence in report.phase_gates() {
        assert!(crate_root_path(evidence.evidence_path()).exists());
    }
}

#[test]
fn production_readiness_hostile_pressures_inventory_stays_exact() {
    let report = readiness::foundational_boundary_evidence_milestone7_readiness_report();

    assert_exact_inventory(
        "synthetic pressures",
        report.synthetic_pressures(),
        &[
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::PrimitiveAdjacencyHostility,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::FreshnessDisclosureHostility,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::PlannedVersusExecutedSeparation,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::ReplayVersusHistoryMasqueradeRejection,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::SupportGradeOverclaimRejection,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::AttachmentScopeAndOrderingHostility,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::TrustBoundaryReadmissionWORTHry,
        ],
    );
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
