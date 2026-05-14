use forge_foundational::{
    canonical_milestone2_production_readiness_report,
    certify_canonical_milestone2_production_readiness, require_canonical_production_test_readiness,
    CanonicalCertifiedSurface, CanonicalCompileFailBoundary, CanonicalCostCounterEvidence,
    CanonicalGoldenArtifactEvidence, CanonicalHarnessExpansionPoint, CanonicalMilestone2PhaseGate,
    CanonicalProductionReadinessCertified, CanonicalProductionTestReadyArtifact,
    CanonicalPropertySeed, CanonicalResidualDebt, CanonicalRuntimeAssumption,
    CanonicalRuntimeNonAssumption, CanonicalSyntheticRuntimePressure,
};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::path::Path;

fn accepts_production_readiness_artifact(_: &CanonicalProductionTestReadyArtifact) {}
fn accepts_production_readiness_proof(
    _: &forge_proof::Proof<
        CanonicalProductionReadinessCertified,
        forge_foundational::CanonicalProductionReadinessAuthority,
    >,
) {
}

#[test]
fn production_readiness_artifact_carries_complete_machine_checkable_inventory() {
    let readiness = certify_canonical_milestone2_production_readiness();
    let report = require_canonical_production_test_readiness(&readiness);

    accepts_production_readiness_artifact(&readiness);
    accepts_production_readiness_proof(readiness.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness.strong_basis().value().milestone(),
        "forge-foundational.milestone-2"
    );

    assert_exact_inventory(
        "certified surfaces",
        report.certified_surfaces(),
        &[
            CanonicalCertifiedSurface::BasisGrammar,
            CanonicalCertifiedSurface::MilestoneOneBasisBuilders,
            CanonicalCertifiedSurface::EquivalenceBasis,
            CanonicalCertifiedSurface::MismatchBasis,
            CanonicalCertifiedSurface::ExportBundles,
            CanonicalCertifiedSurface::DigestAlgorithmSlots,
        ],
    );
    assert_exact_inventory(
        "compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            CanonicalCompileFailBoundary::RawBasisCannotSatisfyReadiness,
            CanonicalCompileFailBoundary::RawComparisonCannotSatisfyEquivalence,
            CanonicalCompileFailBoundary::BoundaryExportRequiresReadmission,
            CanonicalCompileFailBoundary::DigestDerivationRequiresReadyArtifact,
            CanonicalCompileFailBoundary::ProductionReadinessRequiresCertifiedArtifact,
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

    for surface in [
        CanonicalCertifiedSurface::BasisGrammar,
        CanonicalCertifiedSurface::MilestoneOneBasisBuilders,
        CanonicalCertifiedSurface::EquivalenceBasis,
        CanonicalCertifiedSurface::MismatchBasis,
        CanonicalCertifiedSurface::ExportBundles,
        CanonicalCertifiedSurface::DigestAlgorithmSlots,
    ] {
        assert!(report.certified_surfaces().contains(&surface));
        assert!(report
            .certified_surface_evidence()
            .iter()
            .any(|evidence| evidence.surface() == surface));
    }

    assert!(report
        .residual_debt()
        .contains(&CanonicalResidualDebt::FinalCryptographicPolicyDeferred));
    assert!(report
        .non_assumptions()
        .contains(&CanonicalRuntimeNonAssumption::RealRuntimeLoweringCorrect));
}

#[test]
fn production_readiness_surface_evidence_binds_surfaces_to_real_hostile_and_blind_tests() {
    let report = canonical_milestone2_production_readiness_report();

    for evidence in report.certified_surface_evidence() {
        assert!(report.certified_surfaces().contains(&evidence.surface()));
        assert!(report
            .synthetic_pressures()
            .contains(&evidence.hostile_pressure()));
        assert!(report
            .compile_fail_boundaries()
            .contains(&evidence.compile_fail_boundary()));
        assert!(report
            .cost_counter_evidence()
            .contains(&evidence.cost_counter_evidence()));
        assert!(crate_root_path(evidence.blind_consumer_evidence_path()).is_file());
        assert!(crate_root_path(evidence.compile_fail_evidence_path()).is_file());
        assert!(
            evidence
                .compile_fail_evidence_path()
                .starts_with("tests/ui/"),
            "compile-fail evidence must be a trybuild fixture"
        );
        if let Some(artifact) = evidence.golden_artifact() {
            assert!(report.golden_artifacts().contains(&artifact));
            assert!(report
                .fixture_manifest()
                .iter()
                .any(|fixture| fixture.artifact() == artifact));
        }
    }
}

#[test]
fn production_readiness_report_names_hostile_pressures_and_property_seeds() {
    let report = canonical_milestone2_production_readiness_report();

    assert_exact_inventory(
        "synthetic pressures",
        report.synthetic_pressures(),
        &[
            CanonicalSyntheticRuntimePressure::OrderedAuthorityProducer,
            CanonicalSyntheticRuntimePressure::ReorderedCompatibilityProducer,
            CanonicalSyntheticRuntimePressure::SupportExportConsumer,
            CanonicalSyntheticRuntimePressure::CategoryAdjacentHostileProducer,
            CanonicalSyntheticRuntimePressure::BlindMismatchConsumer,
        ],
    );
    assert_exact_inventory(
        "property seeds",
        report.property_seeds(),
        &[
            CanonicalPropertySeed::OrderingIndependence,
            CanonicalPropertySeed::CategoryAdjacency,
            CanonicalPropertySeed::CompatibilityLoweringParity,
            CanonicalPropertySeed::EquivalenceScope,
            CanonicalPropertySeed::MismatchLocus,
            CanonicalPropertySeed::DigestSlotHostility,
        ],
    );
    assert_exact_inventory(
        "cost counter evidence",
        report.cost_counter_evidence(),
        &[
            CanonicalCostCounterEvidence::BasisSequenceConstruction,
            CanonicalCostCounterEvidence::MilestoneOneSurfaceLowering,
            CanonicalCostCounterEvidence::ExportManifestRows,
            CanonicalCostCounterEvidence::DigestInputMetadata,
        ],
    );

    assert!(report
        .assumptions()
        .contains(&CanonicalRuntimeAssumption::DigestDerivationGatedByReadiness));
    assert!(report
        .non_assumptions()
        .contains(&CanonicalRuntimeNonAssumption::DigestEqualityAuthorizesSemanticEquivalence));
    let evidenced_seeds: Vec<_> = report
        .property_seed_evidence()
        .iter()
        .map(|evidence| evidence.seed())
        .collect();
    assert_exact_inventory(
        "property seed evidence",
        &evidenced_seeds,
        report.property_seeds(),
    );
    assert!(report.property_seed_evidence().iter().all(|evidence| {
        !evidence.hostile_dimension().trim().is_empty()
            && canonicalization_test_path(evidence.owning_test()).is_file()
            && report
                .harness_expansion_points()
                .contains(&evidence.harness_lane())
    }));
}

#[test]
fn production_readiness_fixture_manifest_maps_exact_golden_artifacts_to_harness_lanes() {
    let report = canonical_milestone2_production_readiness_report();

    let manifest_artifacts: Vec<_> = report
        .fixture_manifest()
        .iter()
        .map(|fixture| fixture.artifact())
        .collect();
    assert_exact_inventory(
        "fixture manifest artifacts",
        &manifest_artifacts,
        report.golden_artifacts(),
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::AspectContractBasis,
        "golden_artifacts/boundary_digest_bases.rs",
        CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::AspectMaskBasis,
        "golden_artifacts/boundary_digest_bases.rs",
        CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::AuthoritativeStateBasis,
        "golden_artifacts/boundary_digest_bases.rs",
        CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::AuthoritativePatchBasis,
        "golden_artifacts/boundary_digest_bases.rs",
        CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::CompatibilityLoweredStateBasis,
        "golden_artifacts/boundary_digest_bases.rs",
        CanonicalHarnessExpansionPoint::RuntimeParityRunMatrix,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::ExportBundleManifest,
        "export/export_ready_fixtures.rs",
        CanonicalHarnessExpansionPoint::ExportFixtureReplayLane,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::EquivalenceBasis,
        "golden_artifacts/equivalence_and_mismatch.rs",
        CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::MismatchBasis,
        "golden_artifacts/equivalence_and_mismatch.rs",
        CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
    );
    assert_fixture_manifest_row(
        &report,
        CanonicalGoldenArtifactEvidence::DigestSlotDerivedValue,
        "golden_artifacts/digest_slots.rs",
        CanonicalHarnessExpansionPoint::DigestSlotHostilityLane,
    );
}

#[test]
fn production_readiness_phase_gates_are_linear_and_evidence_backed() {
    let report = canonical_milestone2_production_readiness_report();
    let gates: Vec<_> = report
        .phase_gates()
        .iter()
        .map(|evidence| evidence.gate())
        .collect();

    assert_eq!(
        gates,
        vec![
            CanonicalMilestone2PhaseGate::BasisGrammar,
            CanonicalMilestone2PhaseGate::MilestoneOneBasisBuilders,
            CanonicalMilestone2PhaseGate::EquivalenceAndMismatch,
            CanonicalMilestone2PhaseGate::ExportFixtures,
            CanonicalMilestone2PhaseGate::DigestSlots,
            CanonicalMilestone2PhaseGate::ProductionReadiness,
        ]
    );
    assert!(report
        .phase_gates()
        .iter()
        .all(|evidence| evidence.evidence_path().starts_with("tests/certification/")));
    assert!(report
        .phase_gates()
        .iter()
        .all(|evidence| crate_root_path(evidence.evidence_path()).exists()));
}

#[test]
fn production_readiness_fixture_manifest_points_at_real_owning_tests() {
    let report = canonical_milestone2_production_readiness_report();

    assert!(report
        .fixture_manifest()
        .iter()
        .all(|fixture| { canonicalization_test_path(fixture.owning_test()).is_file() }));
}

#[test]
fn production_readiness_topology_keeps_responsibility_homes_distinct() {
    let report = canonical_milestone2_production_readiness_report();
    let evidence_paths: Vec<_> = report
        .phase_gates()
        .iter()
        .map(|evidence| evidence.evidence_path())
        .collect();

    assert!(evidence_paths.contains(&"tests/certification/canonicalization/basis"));
    assert!(evidence_paths.contains(&"tests/certification/canonicalization/equivalence"));
    assert!(evidence_paths.contains(&"tests/certification/canonicalization/export"));
    assert!(evidence_paths.contains(&"tests/certification/canonicalization/digest_slots"));
    assert!(evidence_paths.contains(&"tests/certification/canonicalization/production_readiness"));
}

fn crate_root_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn canonicalization_test_path(relative: &str) -> std::path::PathBuf {
    crate_root_path("tests/certification/canonicalization").join(relative)
}

fn assert_fixture_manifest_row(
    report: &forge_foundational::CanonicalProductionReadinessReport,
    artifact: CanonicalGoldenArtifactEvidence,
    owning_test: &'static str,
    harness_lane: CanonicalHarnessExpansionPoint,
) {
    assert!(report.fixture_manifest().iter().any(|fixture| {
        fixture.artifact() == artifact
            && fixture.owning_test() == owning_test
            && fixture.harness_lane() == harness_lane
    }));
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
