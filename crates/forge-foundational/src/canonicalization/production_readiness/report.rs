use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries, cost_counter_evidence,
    fixture_manifest, golden_artifacts, harness_expansion_points, phase_gates,
    property_seed_evidence, property_seeds, public_surface_compile_fail_path,
    public_surface_evidence_path, public_surface_inventory, residual_debt, runtime_assumptions,
    runtime_non_assumptions, synthetic_pressures,
};
use super::vocabulary::{
    CanonicalCertifiedSurface, CanonicalCertifiedSurfaceEvidence, CanonicalCompileFailBoundary,
    CanonicalCostCounterEvidence, CanonicalFixtureManifestEvidence,
    CanonicalGoldenArtifactEvidence, CanonicalHarnessExpansionPoint, CanonicalMilestone2PhaseGate,
    CanonicalPhaseGateEvidence, CanonicalPropertySeed, CanonicalPropertySeedEvidence,
    CanonicalResidualDebt, CanonicalRuntimeAssumption, CanonicalRuntimeNonAssumption,
    CanonicalSyntheticRuntimePressure,
};
use crate::canonicalization_api::{CanonicalPublicLane, CanonicalPublicSurfaceEntry};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalProductionReadinessReport {
    certified_surfaces: Vec<CanonicalCertifiedSurface>,
    certified_surface_evidence: Vec<CanonicalCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<CanonicalSyntheticRuntimePressure>,
    compile_fail_boundaries: Vec<CanonicalCompileFailBoundary>,
    golden_artifacts: Vec<CanonicalGoldenArtifactEvidence>,
    property_seeds: Vec<CanonicalPropertySeed>,
    property_seed_evidence: Vec<CanonicalPropertySeedEvidence>,
    cost_counter_evidence: Vec<CanonicalCostCounterEvidence>,
    harness_expansion_points: Vec<CanonicalHarnessExpansionPoint>,
    assumptions: Vec<CanonicalRuntimeAssumption>,
    non_assumptions: Vec<CanonicalRuntimeNonAssumption>,
    residual_debt: Vec<CanonicalResidualDebt>,
    phase_gates: Vec<CanonicalPhaseGateEvidence>,
    fixture_manifest: Vec<CanonicalFixtureManifestEvidence>,
    public_surface_inventory: Vec<CanonicalPublicSurfaceEntry>,
    public_surface_evidence_path: &'static str,
    public_surface_compile_fail_path: &'static str,
}

impl CanonicalProductionReadinessReport {
    pub(super) fn new() -> Self {
        Self {
            certified_surfaces: certified_surfaces(),
            certified_surface_evidence: certified_surface_evidence(),
            synthetic_pressures: synthetic_pressures(),
            compile_fail_boundaries: compile_fail_boundaries(),
            golden_artifacts: golden_artifacts(),
            property_seeds: property_seeds(),
            property_seed_evidence: property_seed_evidence(),
            cost_counter_evidence: cost_counter_evidence(),
            harness_expansion_points: harness_expansion_points(),
            assumptions: runtime_assumptions(),
            non_assumptions: runtime_non_assumptions(),
            residual_debt: residual_debt(),
            phase_gates: phase_gates(),
            fixture_manifest: fixture_manifest(),
            public_surface_inventory: public_surface_inventory(),
            public_surface_evidence_path: public_surface_evidence_path(),
            public_surface_compile_fail_path: public_surface_compile_fail_path(),
        }
    }

    pub fn certified_surfaces(&self) -> &[CanonicalCertifiedSurface] {
        &self.certified_surfaces
    }

    pub fn certified_surface_evidence(&self) -> &[CanonicalCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }

    pub fn synthetic_pressures(&self) -> &[CanonicalSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }

    pub fn compile_fail_boundaries(&self) -> &[CanonicalCompileFailBoundary] {
        &self.compile_fail_boundaries
    }

    pub fn golden_artifacts(&self) -> &[CanonicalGoldenArtifactEvidence] {
        &self.golden_artifacts
    }

    pub fn property_seeds(&self) -> &[CanonicalPropertySeed] {
        &self.property_seeds
    }

    pub fn property_seed_evidence(&self) -> &[CanonicalPropertySeedEvidence] {
        &self.property_seed_evidence
    }

    pub fn cost_counter_evidence(&self) -> &[CanonicalCostCounterEvidence] {
        &self.cost_counter_evidence
    }

    pub fn harness_expansion_points(&self) -> &[CanonicalHarnessExpansionPoint] {
        &self.harness_expansion_points
    }

    pub fn assumptions(&self) -> &[CanonicalRuntimeAssumption] {
        &self.assumptions
    }

    pub fn non_assumptions(&self) -> &[CanonicalRuntimeNonAssumption] {
        &self.non_assumptions
    }

    pub fn residual_debt(&self) -> &[CanonicalResidualDebt] {
        &self.residual_debt
    }

    pub fn phase_gates(&self) -> &[CanonicalPhaseGateEvidence] {
        &self.phase_gates
    }

    pub fn fixture_manifest(&self) -> &[CanonicalFixtureManifestEvidence] {
        &self.fixture_manifest
    }

    pub fn public_surface_inventory(&self) -> &[CanonicalPublicSurfaceEntry] {
        &self.public_surface_inventory
    }

    pub fn public_surface_evidence_path(&self) -> &'static str {
        self.public_surface_evidence_path
    }

    pub fn public_surface_compile_fail_path(&self) -> &'static str {
        self.public_surface_compile_fail_path
    }

    pub fn passes_readiness_checklist(&self) -> bool {
        self.has_all_certified_surfaces()
            && self.has_certified_surface_evidence()
            && self.has_all_synthetic_pressures()
            && self.has_all_phase_gates()
            && self.has_compile_fail_evidence()
            && self.has_all_golden_artifacts()
            && self.has_all_property_seeds()
            && self.has_property_seed_evidence()
            && self.has_all_cost_counter_evidence()
            && self.has_fixture_manifest_coverage()
            && self.has_named_residual_debt()
            && self.has_harness_expansion_points()
            && self.has_runtime_assumption_boundary()
            && self.has_exact_public_surface_inventory()
    }

    fn has_all_certified_surfaces(&self) -> bool {
        [
            CanonicalCertifiedSurface::BasisGrammar,
            CanonicalCertifiedSurface::MilestoneOneBasisBuilders,
            CanonicalCertifiedSurface::EquivalenceBasis,
            CanonicalCertifiedSurface::MismatchBasis,
            CanonicalCertifiedSurface::ExportBundles,
            CanonicalCertifiedSurface::DigestAlgorithmSlots,
        ]
        .iter()
        .all(|surface| self.certified_surfaces.contains(surface))
    }

    fn has_certified_surface_evidence(&self) -> bool {
        self.surface_evidence_has_exact_coverage()
            && self.certified_surface_evidence.iter().all(|evidence| {
                self.synthetic_pressures
                    .contains(&evidence.hostile_pressure())
                    && self
                        .compile_fail_boundaries
                        .contains(&evidence.compile_fail_boundary())
                    && self
                        .cost_counter_evidence
                        .contains(&evidence.cost_counter_evidence())
                    && evidence
                        .golden_artifact()
                        .is_none_or(|artifact| self.golden_artifacts.contains(&artifact))
            })
    }

    fn surface_evidence_has_exact_coverage(&self) -> bool {
        let certified: BTreeSet<_> = self.certified_surfaces.iter().copied().collect();
        let evidenced: BTreeSet<_> = self
            .certified_surface_evidence
            .iter()
            .map(|evidence| evidence.surface())
            .collect();

        certified == evidenced
            && self.certified_surfaces.len() == self.certified_surface_evidence.len()
    }

    fn has_all_synthetic_pressures(&self) -> bool {
        [
            CanonicalSyntheticRuntimePressure::OrderedAuthorityProducer,
            CanonicalSyntheticRuntimePressure::ReorderedCompatibilityProducer,
            CanonicalSyntheticRuntimePressure::SupportExportConsumer,
            CanonicalSyntheticRuntimePressure::CategoryAdjacentHostileProducer,
            CanonicalSyntheticRuntimePressure::BlindMismatchConsumer,
        ]
        .iter()
        .all(|pressure| self.synthetic_pressures.contains(pressure))
    }

    fn has_all_phase_gates(&self) -> bool {
        [
            CanonicalMilestone2PhaseGate::BasisGrammar,
            CanonicalMilestone2PhaseGate::MilestoneOneBasisBuilders,
            CanonicalMilestone2PhaseGate::EquivalenceAndMismatch,
            CanonicalMilestone2PhaseGate::ExportFixtures,
            CanonicalMilestone2PhaseGate::DigestSlots,
            CanonicalMilestone2PhaseGate::ProductionReadiness,
        ]
        .iter()
        .all(|gate| {
            self.phase_gates
                .iter()
                .any(|evidence| evidence.gate() == *gate)
        })
    }

    fn has_compile_fail_evidence(&self) -> bool {
        [
            CanonicalCompileFailBoundary::RawBasisCannotSatisfyReadiness,
            CanonicalCompileFailBoundary::RawComparisonCannotSatisfyEquivalence,
            CanonicalCompileFailBoundary::BoundaryExportRequiresReadmission,
            CanonicalCompileFailBoundary::DigestDerivationRequiresReadyArtifact,
            CanonicalCompileFailBoundary::ProductionReadinessRequiresCertifiedArtifact,
        ]
        .iter()
        .all(|boundary| self.compile_fail_boundaries.contains(boundary))
    }

    fn has_all_golden_artifacts(&self) -> bool {
        [
            CanonicalGoldenArtifactEvidence::ValueFamilies,
            CanonicalGoldenArtifactEvidence::AspectContractBasis,
            CanonicalGoldenArtifactEvidence::AspectMaskBasis,
            CanonicalGoldenArtifactEvidence::AuthoritativeStateBasis,
            CanonicalGoldenArtifactEvidence::AuthoritativePatchBasis,
            CanonicalGoldenArtifactEvidence::CompatibilityLoweredStateBasis,
            CanonicalGoldenArtifactEvidence::IdentityAndLocator,
            CanonicalGoldenArtifactEvidence::EquivalenceBasis,
            CanonicalGoldenArtifactEvidence::MismatchBasis,
            CanonicalGoldenArtifactEvidence::ExportBundleManifest,
            CanonicalGoldenArtifactEvidence::DigestSlotDerivedValue,
        ]
        .iter()
        .all(|artifact| self.golden_artifacts.contains(artifact))
    }

    fn has_all_property_seeds(&self) -> bool {
        [
            CanonicalPropertySeed::OrderingIndependence,
            CanonicalPropertySeed::CategoryAdjacency,
            CanonicalPropertySeed::CompatibilityLoweringParity,
            CanonicalPropertySeed::EquivalenceScope,
            CanonicalPropertySeed::MismatchLocus,
            CanonicalPropertySeed::DigestSlotHostility,
        ]
        .iter()
        .all(|seed| self.property_seeds.contains(seed))
    }

    fn has_property_seed_evidence(&self) -> bool {
        let seeds: BTreeSet<_> = self.property_seeds.iter().copied().collect();
        let evidenced: BTreeSet<_> = self
            .property_seed_evidence
            .iter()
            .map(|evidence| evidence.seed())
            .collect();

        seeds == evidenced
            && self.property_seeds.len() == self.property_seed_evidence.len()
            && self.property_seed_evidence.iter().all(|evidence| {
                !evidence.hostile_dimension().trim().is_empty()
                    && self
                        .harness_expansion_points
                        .contains(&evidence.harness_lane())
            })
    }

    fn has_all_cost_counter_evidence(&self) -> bool {
        [
            CanonicalCostCounterEvidence::BasisSequenceConstruction,
            CanonicalCostCounterEvidence::MilestoneOneSurfaceLowering,
            CanonicalCostCounterEvidence::ExportManifestRows,
            CanonicalCostCounterEvidence::DigestInputMetadata,
        ]
        .iter()
        .all(|evidence| self.cost_counter_evidence.contains(evidence))
    }

    fn has_fixture_manifest_coverage(&self) -> bool {
        self.golden_artifacts.iter().all(|artifact| {
            self.fixture_manifest
                .iter()
                .any(|fixture| fixture.artifact() == *artifact)
        })
    }

    fn has_named_residual_debt(&self) -> bool {
        [
            CanonicalResidualDebt::FinalCryptographicPolicyDeferred,
            CanonicalResidualDebt::RealRuntimeAdoptionParityDeferred,
            CanonicalResidualDebt::LaterMilestoneOntologyDeferred,
        ]
        .iter()
        .all(|debt| self.residual_debt.contains(debt))
    }

    fn has_harness_expansion_points(&self) -> bool {
        [
            CanonicalHarnessExpansionPoint::CanonicalBasisReplayLane,
            CanonicalHarnessExpansionPoint::ExportFixtureReplayLane,
            CanonicalHarnessExpansionPoint::DigestSlotHostilityLane,
            CanonicalHarnessExpansionPoint::RuntimeParityRunMatrix,
            CanonicalHarnessExpansionPoint::GroupedPublicSurfaceLane,
        ]
        .iter()
        .all(|point| self.harness_expansion_points.contains(point))
    }

    fn has_runtime_assumption_boundary(&self) -> bool {
        self.assumptions
            .contains(&CanonicalRuntimeAssumption::FoundationalBasisLawCertified)
            && self
                .assumptions
                .contains(&CanonicalRuntimeAssumption::DigestDerivationGatedByReadiness)
            && self
                .non_assumptions
                .contains(&CanonicalRuntimeNonAssumption::RealRuntimeLoweringCorrect)
            && self.non_assumptions.contains(
                &CanonicalRuntimeNonAssumption::DigestEqualityAuthorizesSemanticEquivalence,
            )
    }

    fn has_exact_public_surface_inventory(&self) -> bool {
        let paths: BTreeSet<_> = self
            .public_surface_inventory
            .iter()
            .map(|entry| entry.path())
            .collect();
        let common_path_count = self
            .public_surface_inventory
            .iter()
            .filter(|entry| entry.lane() == CanonicalPublicLane::CommonPath)
            .count();
        let stronger_lane_count = self
            .public_surface_inventory
            .iter()
            .filter(|entry| entry.lane() == CanonicalPublicLane::StrongerLane)
            .count();

        paths
            == BTreeSet::from([
                "forge_foundational::canonicalization_api::common_path",
                "forge_foundational::canonicalization_api::lower_lane::basis",
                "forge_foundational::canonicalization_api::lower_lane::comparison",
                "forge_foundational::canonicalization_api::lower_lane::export",
                "forge_foundational::canonicalization_api::lower_lane::digest",
                "forge_foundational::canonicalization_api::stronger_lane",
                "forge_foundational::canonicalization_api::stronger_lane::readiness",
            ])
            && self.public_surface_inventory.len() == paths.len()
            && common_path_count == 1
            && stronger_lane_count == 2
            && self.public_surface_inventory.iter().all(|entry| {
                !entry.teaches().trim().is_empty() && !entry.does_not_hide().trim().is_empty()
            })
    }
}
