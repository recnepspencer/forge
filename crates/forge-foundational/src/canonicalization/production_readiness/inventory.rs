use super::vocabulary::{
    CanonicalCertifiedSurface, CanonicalCertifiedSurfaceEvidence, CanonicalCompileFailBoundary,
    CanonicalCostCounterEvidence, CanonicalFixtureManifestEvidence,
    CanonicalGoldenArtifactEvidence, CanonicalHarnessExpansionPoint, CanonicalMilestone2PhaseGate,
    CanonicalPhaseGateEvidence, CanonicalPropertySeed, CanonicalResidualDebt,
    CanonicalRuntimeAssumption, CanonicalRuntimeNonAssumption, CanonicalSyntheticRuntimePressure,
};

pub(super) fn certified_surfaces() -> Vec<CanonicalCertifiedSurface> {
    vec![
        CanonicalCertifiedSurface::BasisGrammar,
        CanonicalCertifiedSurface::MilestoneOneBasisBuilders,
        CanonicalCertifiedSurface::EquivalenceBasis,
        CanonicalCertifiedSurface::MismatchBasis,
        CanonicalCertifiedSurface::ExportBundles,
        CanonicalCertifiedSurface::DigestAlgorithmSlots,
    ]
}

pub(super) fn certified_surface_evidence() -> Vec<CanonicalCertifiedSurfaceEvidence> {
    vec![
        CanonicalCertifiedSurfaceEvidence::new(
            CanonicalCertifiedSurface::BasisGrammar,
            CanonicalSyntheticRuntimePressure::OrderedAuthorityProducer,
            "tests/certification/canonicalization/basis/basis_grammar.rs",
            CanonicalCompileFailBoundary::RawBasisCannotSatisfyReadiness,
            "tests/ui/canonicalization/basis/raw_sequence_cannot_satisfy_ready_basis_api.rs",
            CanonicalCostCounterEvidence::BasisSequenceConstruction,
            Some(CanonicalGoldenArtifactEvidence::BoundaryDigestBases),
        ),
        CanonicalCertifiedSurfaceEvidence::new(
            CanonicalCertifiedSurface::MilestoneOneBasisBuilders,
            CanonicalSyntheticRuntimePressure::ReorderedCompatibilityProducer,
            "tests/certification/canonicalization/digest_preparation/canonical_basis_builders.rs",
            CanonicalCompileFailBoundary::RawBasisCannotSatisfyReadiness,
            "tests/ui/digest_preparation/non_ready_state_cannot_satisfy_digest_basis_api.rs",
            CanonicalCostCounterEvidence::MilestoneOneSurfaceLowering,
            Some(CanonicalGoldenArtifactEvidence::ValueFamilies),
        ),
        CanonicalCertifiedSurfaceEvidence::new(
            CanonicalCertifiedSurface::EquivalenceBasis,
            CanonicalSyntheticRuntimePressure::BlindMismatchConsumer,
            "tests/certification/canonicalization/equivalence/comparison_readiness.rs",
            CanonicalCompileFailBoundary::RawComparisonCannotSatisfyEquivalence,
            "tests/ui/canonicalization/equivalence/raw_ready_basis_cannot_satisfy_comparison_api.rs",
            CanonicalCostCounterEvidence::BasisSequenceConstruction,
            Some(CanonicalGoldenArtifactEvidence::EquivalenceBasis),
        ),
        CanonicalCertifiedSurfaceEvidence::new(
            CanonicalCertifiedSurface::MismatchBasis,
            CanonicalSyntheticRuntimePressure::BlindMismatchConsumer,
            "tests/certification/canonicalization/equivalence/comparison_readiness.rs",
            CanonicalCompileFailBoundary::RawComparisonCannotSatisfyEquivalence,
            "tests/ui/canonicalization/equivalence/raw_ready_basis_cannot_satisfy_comparison_api.rs",
            CanonicalCostCounterEvidence::BasisSequenceConstruction,
            Some(CanonicalGoldenArtifactEvidence::MismatchBasis),
        ),
        CanonicalCertifiedSurfaceEvidence::new(
            CanonicalCertifiedSurface::ExportBundles,
            CanonicalSyntheticRuntimePressure::SupportExportConsumer,
            "tests/certification/canonicalization/export/export_ready_fixtures.rs",
            CanonicalCompileFailBoundary::BoundaryExportRequiresReadmission,
            "tests/ui/canonicalization/export/boundary_bridged_export_cannot_satisfy_current_export_api.rs",
            CanonicalCostCounterEvidence::ExportManifestRows,
            Some(CanonicalGoldenArtifactEvidence::ExportBundleManifest),
        ),
        CanonicalCertifiedSurfaceEvidence::new(
            CanonicalCertifiedSurface::DigestAlgorithmSlots,
            CanonicalSyntheticRuntimePressure::CategoryAdjacentHostileProducer,
            "tests/certification/canonicalization/digest_slots/digest_derivation.rs",
            CanonicalCompileFailBoundary::DigestDerivationRequiresReadyArtifact,
            "tests/ui/canonicalization/digest_slots/raw_bytes_cannot_satisfy_digest_derivation.rs",
            CanonicalCostCounterEvidence::DigestInputMetadata,
            Some(CanonicalGoldenArtifactEvidence::DigestSlotDerivedValue),
        ),
    ]
}

pub(super) fn synthetic_pressures() -> Vec<CanonicalSyntheticRuntimePressure> {
    vec![
        CanonicalSyntheticRuntimePressure::OrderedAuthorityProducer,
        CanonicalSyntheticRuntimePressure::ReorderedCompatibilityProducer,
        CanonicalSyntheticRuntimePressure::SupportExportConsumer,
        CanonicalSyntheticRuntimePressure::CategoryAdjacentHostileProducer,
        CanonicalSyntheticRuntimePressure::BlindMismatchConsumer,
    ]
}

pub(super) fn compile_fail_boundaries() -> Vec<CanonicalCompileFailBoundary> {
    vec![
        CanonicalCompileFailBoundary::RawBasisCannotSatisfyReadiness,
        CanonicalCompileFailBoundary::RawComparisonCannotSatisfyEquivalence,
        CanonicalCompileFailBoundary::BoundaryExportRequiresReadmission,
        CanonicalCompileFailBoundary::DigestDerivationRequiresReadyArtifact,
        CanonicalCompileFailBoundary::ProductionReadinessRequiresCertifiedArtifact,
    ]
}

pub(super) fn golden_artifacts() -> Vec<CanonicalGoldenArtifactEvidence> {
    vec![
        CanonicalGoldenArtifactEvidence::ValueFamilies,
        CanonicalGoldenArtifactEvidence::BoundaryDigestBases,
        CanonicalGoldenArtifactEvidence::IdentityAndLocator,
        CanonicalGoldenArtifactEvidence::EquivalenceBasis,
        CanonicalGoldenArtifactEvidence::MismatchBasis,
        CanonicalGoldenArtifactEvidence::ExportBundleManifest,
        CanonicalGoldenArtifactEvidence::DigestSlotDerivedValue,
    ]
}

pub(super) fn property_seeds() -> Vec<CanonicalPropertySeed> {
    vec![
        CanonicalPropertySeed::OrderingIndependence,
        CanonicalPropertySeed::CategoryAdjacency,
        CanonicalPropertySeed::CompatibilityLoweringParity,
        CanonicalPropertySeed::EquivalenceScope,
        CanonicalPropertySeed::MismatchLocus,
        CanonicalPropertySeed::DigestSlotHostility,
    ]
}

pub(super) fn cost_counter_evidence() -> Vec<CanonicalCostCounterEvidence> {
    vec![
        CanonicalCostCounterEvidence::BasisSequenceConstruction,
        CanonicalCostCounterEvidence::MilestoneOneSurfaceLowering,
        CanonicalCostCounterEvidence::ExportManifestRows,
        CanonicalCostCounterEvidence::DigestInputMetadata,
    ]
}

pub(super) fn harness_expansion_points() -> Vec<CanonicalHarnessExpansionPoint> {
    vec![
        CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
        CanonicalHarnessExpansionPoint::ExportFixtureReplayLane,
        CanonicalHarnessExpansionPoint::DigestSlotHostilityLane,
        CanonicalHarnessExpansionPoint::RuntimeParityRunMatrix,
    ]
}

pub(super) fn runtime_assumptions() -> Vec<CanonicalRuntimeAssumption> {
    vec![
        CanonicalRuntimeAssumption::FoundationalBasisLawCertified,
        CanonicalRuntimeAssumption::CanonicalOrderingStable,
        CanonicalRuntimeAssumption::ComparisonAndMismatchEvidenceSelfDescribing,
        CanonicalRuntimeAssumption::DigestDerivationGatedByReadiness,
    ]
}

pub(super) fn runtime_non_assumptions() -> Vec<CanonicalRuntimeNonAssumption> {
    vec![
        CanonicalRuntimeNonAssumption::RealRuntimeLoweringCorrect,
        CanonicalRuntimeNonAssumption::FinalCryptographicPolicySelected,
        CanonicalRuntimeNonAssumption::ProfilesReceiptsDiagnosticsOrBranchOntologyExists,
        CanonicalRuntimeNonAssumption::DigestEqualityAuthorizesSemanticEquivalence,
    ]
}

pub(super) fn residual_debt() -> Vec<CanonicalResidualDebt> {
    vec![
        CanonicalResidualDebt::FinalCryptographicPolicyDeferred,
        CanonicalResidualDebt::RealRuntimeAdoptionParityDeferred,
        CanonicalResidualDebt::LaterMilestoneOntologyDeferred,
    ]
}

pub(super) fn phase_gates() -> Vec<CanonicalPhaseGateEvidence> {
    vec![
        CanonicalPhaseGateEvidence::new(
            CanonicalMilestone2PhaseGate::BasisGrammar,
            "tests/certification/canonicalization/basis",
        ),
        CanonicalPhaseGateEvidence::new(
            CanonicalMilestone2PhaseGate::MilestoneOneBasisBuilders,
            "tests/certification/canonicalization/digest_preparation",
        ),
        CanonicalPhaseGateEvidence::new(
            CanonicalMilestone2PhaseGate::EquivalenceAndMismatch,
            "tests/certification/canonicalization/equivalence",
        ),
        CanonicalPhaseGateEvidence::new(
            CanonicalMilestone2PhaseGate::ExportFixtures,
            "tests/certification/canonicalization/export",
        ),
        CanonicalPhaseGateEvidence::new(
            CanonicalMilestone2PhaseGate::DigestSlots,
            "tests/certification/canonicalization/digest_slots",
        ),
        CanonicalPhaseGateEvidence::new(
            CanonicalMilestone2PhaseGate::ProductionReadiness,
            "tests/certification/canonicalization/production_readiness",
        ),
    ]
}

pub(super) fn fixture_manifest() -> Vec<CanonicalFixtureManifestEvidence> {
    vec![
        CanonicalFixtureManifestEvidence::new(
            CanonicalGoldenArtifactEvidence::ValueFamilies,
            "golden_artifacts/value_families.rs",
            CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
        ),
        CanonicalFixtureManifestEvidence::new(
            CanonicalGoldenArtifactEvidence::BoundaryDigestBases,
            "golden_artifacts/boundary_digest_bases.rs",
            CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
        ),
        CanonicalFixtureManifestEvidence::new(
            CanonicalGoldenArtifactEvidence::IdentityAndLocator,
            "golden_artifacts/identity_and_locator.rs",
            CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
        ),
        CanonicalFixtureManifestEvidence::new(
            CanonicalGoldenArtifactEvidence::EquivalenceBasis,
            "golden_artifacts/equivalence_and_mismatch.rs",
            CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
        ),
        CanonicalFixtureManifestEvidence::new(
            CanonicalGoldenArtifactEvidence::MismatchBasis,
            "golden_artifacts/equivalence_and_mismatch.rs",
            CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
        ),
        CanonicalFixtureManifestEvidence::new(
            CanonicalGoldenArtifactEvidence::ExportBundleManifest,
            "export/export_ready_fixtures.rs",
            CanonicalHarnessExpansionPoint::ExportFixtureReplayLane,
        ),
        CanonicalFixtureManifestEvidence::new(
            CanonicalGoldenArtifactEvidence::DigestSlotDerivedValue,
            "golden_artifacts/digest_slots.rs",
            CanonicalHarnessExpansionPoint::DigestSlotHostilityLane,
        ),
    ]
}
