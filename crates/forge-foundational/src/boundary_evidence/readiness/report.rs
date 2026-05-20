use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries,
    documentation_surface_inventory, golden_artifacts, harness_expansion_points, phase_gates,
    property_seed_evidence, property_seeds, public_surface_compile_fail_path,
    public_surface_evidence_path, public_surface_inventory, residual_debt, runtime_assumptions,
    runtime_non_assumptions, synthetic_pressures,
};
use super::vocabulary::{
    FoundationalBoundaryEvidenceCertifiedSurface,
    FoundationalBoundaryEvidenceCertifiedSurfaceEvidence,
    FoundationalBoundaryEvidenceCompileFailBoundary, FoundationalBoundaryEvidenceGoldenArtifact,
    FoundationalBoundaryEvidenceHarnessExpansionPoint,
    FoundationalBoundaryEvidencePhaseGateEvidence, FoundationalBoundaryEvidencePropertySeed,
    FoundationalBoundaryEvidencePropertySeedEvidence, FoundationalBoundaryEvidenceResidualDebt,
    FoundationalBoundaryEvidenceRuntimeAssumption,
    FoundationalBoundaryEvidenceRuntimeNonAssumption,
    FoundationalBoundaryEvidenceSyntheticRuntimePressure,
};
use crate::boundary_evidence_api::{
    BoundaryEvidencePublicLane, BoundaryEvidencePublicSurfaceEntry,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceProductionReadinessReport {
    certified_surfaces: Vec<FoundationalBoundaryEvidenceCertifiedSurface>,
    certified_surface_evidence: Vec<FoundationalBoundaryEvidenceCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<FoundationalBoundaryEvidenceSyntheticRuntimePressure>,
    compile_fail_boundaries: Vec<FoundationalBoundaryEvidenceCompileFailBoundary>,
    golden_artifacts: Vec<FoundationalBoundaryEvidenceGoldenArtifact>,
    property_seeds: Vec<FoundationalBoundaryEvidencePropertySeed>,
    property_seed_evidence: Vec<FoundationalBoundaryEvidencePropertySeedEvidence>,
    harness_expansion_points: Vec<FoundationalBoundaryEvidenceHarnessExpansionPoint>,
    assumptions: Vec<FoundationalBoundaryEvidenceRuntimeAssumption>,
    non_assumptions: Vec<FoundationalBoundaryEvidenceRuntimeNonAssumption>,
    residual_debt: Vec<FoundationalBoundaryEvidenceResidualDebt>,
    phase_gates: Vec<FoundationalBoundaryEvidencePhaseGateEvidence>,
    public_surface_inventory: Vec<BoundaryEvidencePublicSurfaceEntry>,
    documentation_surface_inventory: Vec<&'static str>,
    public_surface_evidence_path: &'static str,
    public_surface_compile_fail_path: &'static str,
}

impl FoundationalBoundaryEvidenceProductionReadinessReport {
    pub(super) fn new() -> Self {
        Self {
            certified_surfaces: certified_surfaces(),
            certified_surface_evidence: certified_surface_evidence(),
            synthetic_pressures: synthetic_pressures(),
            compile_fail_boundaries: compile_fail_boundaries(),
            golden_artifacts: golden_artifacts(),
            property_seeds: property_seeds(),
            property_seed_evidence: property_seed_evidence(),
            harness_expansion_points: harness_expansion_points(),
            assumptions: runtime_assumptions(),
            non_assumptions: runtime_non_assumptions(),
            residual_debt: residual_debt(),
            phase_gates: phase_gates(),
            public_surface_inventory: public_surface_inventory(),
            documentation_surface_inventory: documentation_surface_inventory(),
            public_surface_evidence_path: public_surface_evidence_path(),
            public_surface_compile_fail_path: public_surface_compile_fail_path(),
        }
    }

    pub fn certified_surfaces(&self) -> &[FoundationalBoundaryEvidenceCertifiedSurface] {
        &self.certified_surfaces
    }
    pub fn certified_surface_evidence(
        &self,
    ) -> &[FoundationalBoundaryEvidenceCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }
    pub fn synthetic_pressures(&self) -> &[FoundationalBoundaryEvidenceSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }
    pub fn compile_fail_boundaries(&self) -> &[FoundationalBoundaryEvidenceCompileFailBoundary] {
        &self.compile_fail_boundaries
    }
    pub fn golden_artifacts(&self) -> &[FoundationalBoundaryEvidenceGoldenArtifact] {
        &self.golden_artifacts
    }
    pub fn property_seeds(&self) -> &[FoundationalBoundaryEvidencePropertySeed] {
        &self.property_seeds
    }
    pub fn property_seed_evidence(&self) -> &[FoundationalBoundaryEvidencePropertySeedEvidence] {
        &self.property_seed_evidence
    }
    pub fn harness_expansion_points(&self) -> &[FoundationalBoundaryEvidenceHarnessExpansionPoint] {
        &self.harness_expansion_points
    }
    pub fn assumptions(&self) -> &[FoundationalBoundaryEvidenceRuntimeAssumption] {
        &self.assumptions
    }
    pub fn non_assumptions(&self) -> &[FoundationalBoundaryEvidenceRuntimeNonAssumption] {
        &self.non_assumptions
    }
    pub fn residual_debt(&self) -> &[FoundationalBoundaryEvidenceResidualDebt] {
        &self.residual_debt
    }
    pub fn phase_gates(&self) -> &[FoundationalBoundaryEvidencePhaseGateEvidence] {
        &self.phase_gates
    }
    pub fn public_surface_inventory(&self) -> &[BoundaryEvidencePublicSurfaceEntry] {
        &self.public_surface_inventory
    }
    pub fn documentation_surface_inventory(&self) -> &[&'static str] {
        &self.documentation_surface_inventory
    }
    pub fn public_surface_evidence_path(&self) -> &'static str {
        self.public_surface_evidence_path
    }
    pub fn public_surface_compile_fail_path(&self) -> &'static str {
        self.public_surface_compile_fail_path
    }

    pub fn passes_readiness_checklist(&self) -> bool {
        self.has_exact_surface_coverage()
            && self.has_all_synthetic_pressures()
            && self.has_all_compile_fail_boundaries()
            && self.has_all_golden_artifacts()
            && self.has_all_property_seeds()
            && self.has_all_harness_expansion_points()
            && self.has_runtime_assumption_boundary()
            && self.has_named_residual_debt()
            && self.has_all_phase_gates()
            && self.has_exact_public_surface_inventory()
            && self.has_exact_documentation_surface_inventory()
    }

    fn has_exact_surface_coverage(&self) -> bool {
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
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::PrimitiveAdjacencyHostility,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::FreshnessDisclosureHostility,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::PlannedVersusExecutedSeparation,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::ReplayVersusHistoryMasqueradeRejection,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::SupportGradeOverclaimRejection,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::AttachmentScopeAndOrderingHostility,
            FoundationalBoundaryEvidenceSyntheticRuntimePressure::TrustBoundaryReadmissionForgery,
        ]
        .iter()
        .all(|pressure| self.synthetic_pressures.contains(pressure))
    }

    fn has_all_compile_fail_boundaries(&self) -> bool {
        [
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
        ]
        .iter()
        .all(|boundary| self.compile_fail_boundaries.contains(boundary))
    }

    fn has_all_golden_artifacts(&self) -> bool {
        [
            FoundationalBoundaryEvidenceGoldenArtifact::PrimitiveCategoryRoleAndLocalityMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::ProvenanceLayeringAndFreshnessMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::ReceiptExecutionAndCloseoutMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::LineageContinuityPromotionAndPartialityMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::SupportTruthRecoveryAndResidualDebtMeaning,
            FoundationalBoundaryEvidenceGoldenArtifact::AttachmentCanonicalDigestAndReadmissionMeaning,
        ]
        .iter()
        .all(|artifact| self.golden_artifacts.contains(artifact))
    }

    fn has_all_property_seeds(&self) -> bool {
        let seeded: BTreeSet<_> = self.property_seeds.iter().copied().collect();
        let evidenced: BTreeSet<_> = self
            .property_seed_evidence
            .iter()
            .map(|evidence| evidence.seed())
            .collect();

        seeded
            == BTreeSet::from([
                FoundationalBoundaryEvidencePropertySeed::PrimitiveDefinitionOrdering,
                FoundationalBoundaryEvidencePropertySeed::ProvenanceLayerAndSupportContextOrdering,
                FoundationalBoundaryEvidencePropertySeed::PlanningExecutedAndCloseoutStrength,
                FoundationalBoundaryEvidencePropertySeed::ReplayHistoryAndPromotionStrength,
                FoundationalBoundaryEvidencePropertySeed::MixedAttachmentCanonicalAndDigestParity,
            ])
            && seeded == evidenced
            && self.property_seed_evidence.iter().all(|evidence| {
                !evidence.hostile_dimension().trim().is_empty()
                    && self
                        .harness_expansion_points
                        .contains(&evidence.harness_lane())
            })
    }

    fn has_all_harness_expansion_points(&self) -> bool {
        [
            FoundationalBoundaryEvidenceHarnessExpansionPoint::ReplayHistoryMasqueradeMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::RecoveryAndDegradedOperationMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::MixedAttachmentCanonicalDigestParityMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::TrustBoundaryReadmissionParityMatrix,
            FoundationalBoundaryEvidenceHarnessExpansionPoint::GroupedPublicSurfaceLane,
        ]
        .iter()
        .all(|point| self.harness_expansion_points.contains(point))
    }

    fn has_runtime_assumption_boundary(&self) -> bool {
        self.assumptions.contains(
            &FoundationalBoundaryEvidenceRuntimeAssumption::ForgeProofAuthorityLaneRemainsAvailable,
        ) && self.assumptions.contains(
            &FoundationalBoundaryEvidenceRuntimeAssumption::BoundaryArtifactAndDiagnosticMeaningRemainCertifiedDependencies,
        ) && self.non_assumptions.contains(
            &FoundationalBoundaryEvidenceRuntimeNonAssumption::SupportTruthUpgradesToAuthorityWithoutBridge,
        ) && self.non_assumptions.contains(
            &FoundationalBoundaryEvidenceRuntimeNonAssumption::CrossBoundaryAttachmentBundlesRemainCurrentWithoutReadmission,
        )
    }

    fn has_named_residual_debt(&self) -> bool {
        [
            FoundationalBoundaryEvidenceResidualDebt::AdoptingRuntimeParityDeferred,
            FoundationalBoundaryEvidenceResidualDebt::RuntimeSpecificHistoryAndJournalTaxonomiesDeferred,
            FoundationalBoundaryEvidenceResidualDebt::RealRuntimeSupportBundlePersistenceDeferred,
        ]
        .iter()
        .all(|debt| self.residual_debt.contains(debt))
    }

    fn has_all_phase_gates(&self) -> bool {
        self.phase_gates.len() == 9
            && self
                .phase_gates
                .iter()
                .all(|gate| !gate.evidence_path().trim().is_empty())
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
            .filter(|entry| entry.lane() == BoundaryEvidencePublicLane::CommonPath)
            .count();
        let stronger_lane_count = self
            .public_surface_inventory
            .iter()
            .filter(|entry| entry.lane() == BoundaryEvidencePublicLane::StrongerLane)
            .count();

        paths
            == BTreeSet::from([
                "forge_foundational::boundary_evidence_api::common_path",
                "forge_foundational::boundary_evidence_api::lower_lane::primitives",
                "forge_foundational::boundary_evidence_api::lower_lane::provenance",
                "forge_foundational::boundary_evidence_api::lower_lane::receipts",
                "forge_foundational::boundary_evidence_api::lower_lane::lineage",
                "forge_foundational::boundary_evidence_api::lower_lane::support",
                "forge_foundational::boundary_evidence_api::lower_lane::attachments",
                "forge_foundational::boundary_evidence_api::stronger_lane",
                "forge_foundational::boundary_evidence_api::stronger_lane::readmission",
                "forge_foundational::boundary_evidence_api::stronger_lane::readiness",
            ])
            && self.public_surface_inventory.len() == paths.len()
            && common_path_count == 1
            && stronger_lane_count == 3
            && self.public_surface_inventory.iter().all(|entry| {
                !entry.teaches().trim().is_empty() && !entry.does_not_hide().trim().is_empty()
            })
    }

    fn has_exact_documentation_surface_inventory(&self) -> bool {
        let docs: BTreeSet<_> = self
            .documentation_surface_inventory
            .iter()
            .copied()
            .collect();
        docs
            == BTreeSet::from([
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
            ])
            && docs.len() == self.documentation_surface_inventory.len()
    }
}
