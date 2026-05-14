use forge_foundational::{
    certify_foundational_diagnostic_milestone6_production_test_readiness,
    foundational_diagnostic_milestone6_readiness_report,
    require_foundational_diagnostic_milestone6_production_test_readiness,
    FoundationalDiagnosticAdoptionShapedFollowthrough,
    FoundationalDiagnosticCanonicalGoldenArtifact, FoundationalDiagnosticCertifiedSurface,
    FoundationalDiagnosticCompileFailBoundary, FoundationalDiagnosticForgeProofApi,
    FoundationalDiagnosticForgeProofForbiddenSurface, FoundationalDiagnosticForgeProofSurface,
    FoundationalDiagnosticHarnessExpansionPoint, FoundationalDiagnosticMilestone6PhaseGate,
    FoundationalDiagnosticProductionReadinessCertified,
    FoundationalDiagnosticProductionTestReadyArtifact, FoundationalDiagnosticPropertySeed,
    FoundationalDiagnosticResidualDebt, FoundationalDiagnosticRuntimeAdoptionFailurePressure,
    FoundationalDiagnosticRuntimeAssumption, FoundationalDiagnosticRuntimeNonAssumption,
    FoundationalDiagnosticSyntheticRuntimePressure,
};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::path::Path;

fn accepts_diagnostic_readiness_artifact(_: &FoundationalDiagnosticProductionTestReadyArtifact) {}
fn accepts_diagnostic_readiness_proof(
    _: &forge_proof::Proof<
        FoundationalDiagnosticProductionReadinessCertified,
        forge_foundational::FoundationalDiagnosticProductionReadinessAuthority,
    >,
) {
}

#[test]
fn production_readiness_artifact_carries_complete_machine_checkable_inventory() {
    let readiness = certify_foundational_diagnostic_milestone6_production_test_readiness();
    let report = require_foundational_diagnostic_milestone6_production_test_readiness(&readiness);

    accepts_diagnostic_readiness_artifact(&readiness);
    accepts_diagnostic_readiness_proof(readiness.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness.strong_basis().value().milestone(),
        "forge-foundational.milestone-6"
    );

    assert_exact_inventory(
        "certified surfaces",
        report.certified_surfaces(),
        &[
            FoundationalDiagnosticCertifiedSurface::PrimitiveAndCategoryLaw,
            FoundationalDiagnosticCertifiedSurface::OutcomeSubjectAndRowTopology,
            FoundationalDiagnosticCertifiedSurface::MaterializationSupportAndNamedGapLaw,
            FoundationalDiagnosticCertifiedSurface::CanonicalBasisAndComparisonLaw,
            FoundationalDiagnosticCertifiedSurface::CertifiedBundleAndAttachmentCompatibility,
        ],
    );
    assert_exact_inventory(
        "compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            FoundationalDiagnosticCompileFailBoundary::PrimitiveAndCategoryPreserveNonSubstitution,
            FoundationalDiagnosticCompileFailBoundary::RowTopologyPreservesFamilyAndLocatorLaw,
            FoundationalDiagnosticCompileFailBoundary::MaterializationAndSupportPreserveExplicitSeams,
            FoundationalDiagnosticCompileFailBoundary::BasisAndComparisonPreserveBlindConsumerCanonicalLaw,
            FoundationalDiagnosticCompileFailBoundary::CertifiedBundleAndAttachmentReuseProofLane,
            FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessRequiresCertifiedArtifact,
            FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessAuthorityCannotBeMinted,
        ],
    );
    assert_exact_inventory(
        "canonical golden artifacts",
        report.canonical_golden_artifacts(),
        &[
            FoundationalDiagnosticCanonicalGoldenArtifact::PrimitiveCategoryAndMaterializationMeaning,
            FoundationalDiagnosticCanonicalGoldenArtifact::FamilyDistinctRowTopologyMeaning,
            FoundationalDiagnosticCanonicalGoldenArtifact::MaterializationRichnessAndDebtMeaning,
            FoundationalDiagnosticCanonicalGoldenArtifact::CanonicalBundleAndComparisonMeaning,
            FoundationalDiagnosticCanonicalGoldenArtifact::CertifiedCoverageAndAttachmentMeaning,
        ],
    );
    assert_exact_inventory(
        "property seed inventory",
        report.property_seed_inventory(),
        &[
            FoundationalDiagnosticPropertySeed::PrimitiveOrderingAndTokenCanonicalization,
            FoundationalDiagnosticPropertySeed::RowFamilyOrderingAndSemanticTieBreaks,
            FoundationalDiagnosticPropertySeed::RichnessElisionPreservesTruthUnderPartiality,
            FoundationalDiagnosticPropertySeed::IndependentProducerCanonicalParity,
            FoundationalDiagnosticPropertySeed::CertifiedCoverageNamedGapParity,
        ],
    );
    assert_exact_inventory(
        "harness expansion points",
        report.harness_expansion_points(),
        &[
            FoundationalDiagnosticHarnessExpansionPoint::IndependentProducerDiagnosticParityMatrix,
            FoundationalDiagnosticHarnessExpansionPoint::RichnessAvailabilityAndFallbackReplayMatrix,
            FoundationalDiagnosticHarnessExpansionPoint::BlindConsumerInterpretationReplaySuite,
            FoundationalDiagnosticHarnessExpansionPoint::CertifiedCoverageAttachmentParityMatrix,
        ],
    );
    assert_exact_inventory(
        "runtime adoption failure pressures",
        report.runtime_adoption_failure_pressures(),
        &[
            FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeLoweringMayMisclassifyEvidencePosture,
            FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeMaterializersMayOverclaimDurableOrCertifiedSupport,
            FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeCanonicalRowOrderingMayDriftAcrossStorageLayouts,
            FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeCoverageMatricesMayOmitRequiredFamilies,
            FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeProvenanceReadyRowsMayCollapseIntoExplanationRows,
        ],
    );
    assert_exact_inventory(
        "adoption shaped followthrough",
        report.adoption_shaped_followthrough(),
        &[
            FoundationalDiagnosticAdoptionShapedFollowthrough::ForgeHarnessDiagnosticProducerParityMatrix,
            FoundationalDiagnosticAdoptionShapedFollowthrough::ForgeHarnessRichnessAvailabilityAndFallbackReplaySuite,
            FoundationalDiagnosticAdoptionShapedFollowthrough::AdoptingRuntimeDiagnosticLoweringParityPressure,
            FoundationalDiagnosticAdoptionShapedFollowthrough::AdoptingRuntimeCertifiedCoverageAndAttachmentHostility,
        ],
    );

    for surface in report.certified_surfaces() {
        assert_eq!(
            report
                .certified_surface_evidence()
                .iter()
                .filter(|evidence| evidence.surface() == *surface)
                .count(),
            1,
            "each certified surface must have exactly one evidence row"
        );
    }
}

#[test]
fn production_readiness_report_names_forge_proof_dependency_boundary_and_runtime_handoff() {
    let report = foundational_diagnostic_milestone6_readiness_report();

    assert_exact_inventory(
        "forge-proof required surfaces",
        report.forge_proof_required_surfaces(),
        &[
            FoundationalDiagnosticForgeProofSurface::CertifiedDiagnosticAttachmentAuthority,
            FoundationalDiagnosticForgeProofSurface::ProofBearingCertifiedDiagnosticBundle,
            FoundationalDiagnosticForgeProofSurface::CertifiedBundleBoundaryBridge,
            FoundationalDiagnosticForgeProofSurface::CertifiedBundleReadmitWithAuthority,
            FoundationalDiagnosticForgeProofSurface::ProductionReadinessCertificationArtifact,
        ],
    );
    assert_exact_inventory(
        "forge-proof api appendix",
        report.forge_proof_api_appendix(),
        &[
            FoundationalDiagnosticForgeProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalDiagnosticForgeProofApi::ProofFromAuthorityWitness,
            FoundationalDiagnosticForgeProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalDiagnosticForgeProofApi::ArtifactBridgeTrustBoundary,
            FoundationalDiagnosticForgeProofApi::ArtifactReadmitWithAuthority,
        ],
    );
    assert_exact_inventory(
        "forge-proof forbidden surfaces",
        report.forge_proof_forbidden_surfaces(),
        &[
            FoundationalDiagnosticForgeProofForbiddenSurface::PlainDiagnosticPrimitives,
            FoundationalDiagnosticForgeProofForbiddenSurface::PlainDiagnosticRowsAndBundles,
            FoundationalDiagnosticForgeProofForbiddenSurface::PlainMaterializationVocabulary,
            FoundationalDiagnosticForgeProofForbiddenSurface::PlainCanonicalComparisonVocabulary,
        ],
    );
    assert!(report.assumptions().contains(
        &FoundationalDiagnosticRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForDiagnosticBasis
    ));
    assert!(report.assumptions().contains(
        &FoundationalDiagnosticRuntimeAssumption::CertifiedDiagnosticBundlesReuseForgeProofLane
    ));
    assert!(report.non_assumptions().contains(
        &FoundationalDiagnosticRuntimeNonAssumption::BoundaryCrossingPreservesCertifiedCurrentBasisWithoutReadmission
    ));
    assert_eq!(
        report.residual_debt(),
        &[
            FoundationalDiagnosticResidualDebt::AdoptingRuntimeParityDeferred,
            FoundationalDiagnosticResidualDebt::Milestone7ProvenanceAndReceiptDeepeningDeferred,
            FoundationalDiagnosticResidualDebt::RuntimeSpecificSupportTaxonomiesDeferred,
        ]
    );
}

#[test]
fn production_readiness_surface_evidence_binds_surfaces_to_real_hostile_compile_fail_and_blind_consumer_tests(
) {
    let report = foundational_diagnostic_milestone6_readiness_report();

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
        assert!(crate_root_path(evidence.blind_consumer_evidence_path()).is_file());
        assert!(
            evidence
                .compile_fail_evidence_path()
                .starts_with("tests/ui/"),
            "compile-fail evidence must point at a trybuild fixture"
        );
    }
}

#[test]
fn production_readiness_hostile_pressures_and_compile_fail_boundaries_have_real_evidence_paths() {
    let report = foundational_diagnostic_milestone6_readiness_report();

    for evidence in report.synthetic_pressure_evidence() {
        assert!(report.synthetic_pressures().contains(&evidence.pressure()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
    }

    for evidence in report.canonical_golden_artifact_evidence() {
        assert!(report
            .canonical_golden_artifacts()
            .contains(&evidence.artifact()));
        assert!(crate_root_path(evidence.evidence_path()).is_file());
    }

    for evidence in report.property_seed_evidence() {
        assert!(report.property_seed_inventory().contains(&evidence.seed()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
        assert!(!evidence.hostile_dimension().trim().is_empty());
    }

    for evidence in report.harness_expansion_evidence() {
        assert!(report
            .harness_expansion_points()
            .contains(&evidence.point()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
    }

    for evidence in report.compile_fail_evidence() {
        assert!(report
            .compile_fail_boundaries()
            .contains(&evidence.boundary()));
        assert!(crate_root_path(evidence.evidence_path()).is_file());
        assert!(
            evidence.evidence_path().starts_with("tests/ui/"),
            "compile-fail evidence must point at a trybuild fixture"
        );
    }
}

#[test]
fn production_readiness_forge_proof_appendix_is_bound_to_real_diagnostics_source_lanes() {
    let report = foundational_diagnostic_milestone6_readiness_report();

    for evidence in report.forge_proof_api_evidence() {
        assert!(report.forge_proof_api_appendix().contains(&evidence.api()));
        let source = std::fs::read_to_string(crate_root_path(evidence.source_path()))
            .expect("forge-proof api evidence source must be readable");
        assert!(
            source.contains(evidence.source_snippet()),
            "forge-proof api evidence for {:?} drifted from the named source lane",
            evidence.api()
        );
    }
}

#[test]
fn production_readiness_phase_gates_are_linear_and_evidence_backed() {
    let report = foundational_diagnostic_milestone6_readiness_report();
    let gates: Vec<_> = report
        .phase_gates()
        .iter()
        .map(|evidence| evidence.gate())
        .collect();

    assert_eq!(
        gates,
        vec![
            FoundationalDiagnosticMilestone6PhaseGate::PrimitiveAndCategoryLaw,
            FoundationalDiagnosticMilestone6PhaseGate::OutcomeSubjectAndRowTopology,
            FoundationalDiagnosticMilestone6PhaseGate::MaterializationSupportAndNamedGapLaw,
            FoundationalDiagnosticMilestone6PhaseGate::CanonicalBasisAndComparisonLaw,
            FoundationalDiagnosticMilestone6PhaseGate::CertifiedBundleAndAttachmentCompatibility,
            FoundationalDiagnosticMilestone6PhaseGate::ProductionReadiness,
        ]
    );
    assert!(report
        .phase_gates()
        .iter()
        .all(|evidence| crate_root_path(evidence.evidence_path()).exists()));
}

#[test]
fn production_readiness_hostile_pressure_inventory_stays_exact() {
    let report = foundational_diagnostic_milestone6_readiness_report();

    assert_exact_inventory(
        "synthetic pressures",
        report.synthetic_pressures(),
        &[
            FoundationalDiagnosticSyntheticRuntimePressure::PrimitiveNonSubstitution,
            FoundationalDiagnosticSyntheticRuntimePressure::GenericRowCollapseRejection,
            FoundationalDiagnosticSyntheticRuntimePressure::HiddenRediscoveryDebtRejection,
            FoundationalDiagnosticSyntheticRuntimePressure::ThinOrEmptySupportOverclaimRejection,
            FoundationalDiagnosticSyntheticRuntimePressure::BlindConsumerCanonicalParity,
            FoundationalDiagnosticSyntheticRuntimePressure::HiddenSourceDigestOrCoverageForgery,
            FoundationalDiagnosticSyntheticRuntimePressure::ExplanationProvenanceBoundaryPreservation,
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
