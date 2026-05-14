use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries, compile_fail_evidence,
    forge_proof_api_appendix, forge_proof_api_evidence, forge_proof_forbidden_surfaces,
    forge_proof_required_surfaces, phase_gates, residual_debt, runtime_assumptions,
    runtime_non_assumptions, synthetic_pressure_evidence, synthetic_pressures,
};
use super::production_test_contract::{
    FoundationalDiagnosticAdoptionShapedFollowthrough,
    FoundationalDiagnosticCanonicalGoldenArtifact,
    FoundationalDiagnosticCanonicalGoldenArtifactEvidence,
    FoundationalDiagnosticHarnessExpansionEvidence, FoundationalDiagnosticHarnessExpansionPoint,
    FoundationalDiagnosticPropertySeed, FoundationalDiagnosticPropertySeedEvidence,
    FoundationalDiagnosticRuntimeAdoptionFailurePressure,
};
use super::production_test_handoff::{
    adoption_shaped_followthrough, canonical_golden_artifact_evidence, canonical_golden_artifacts,
    harness_expansion_evidence, harness_expansion_points, property_seed_evidence,
    property_seed_inventory, runtime_adoption_failure_pressures,
};
use super::vocabulary::{
    FoundationalDiagnosticCertifiedSurface, FoundationalDiagnosticCertifiedSurfaceEvidence,
    FoundationalDiagnosticCompileFailBoundary, FoundationalDiagnosticCompileFailEvidence,
    FoundationalDiagnosticForgeProofApi, FoundationalDiagnosticForgeProofApiEvidence,
    FoundationalDiagnosticForgeProofForbiddenSurface, FoundationalDiagnosticForgeProofSurface,
    FoundationalDiagnosticPhaseGateEvidence, FoundationalDiagnosticResidualDebt,
    FoundationalDiagnosticRuntimeAssumption, FoundationalDiagnosticRuntimeNonAssumption,
    FoundationalDiagnosticSyntheticPressureEvidence,
    FoundationalDiagnosticSyntheticRuntimePressure,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticProductionReadinessReport {
    pub(super) certified_surfaces: Vec<FoundationalDiagnosticCertifiedSurface>,
    pub(super) certified_surface_evidence: Vec<FoundationalDiagnosticCertifiedSurfaceEvidence>,
    pub(super) synthetic_pressures: Vec<FoundationalDiagnosticSyntheticRuntimePressure>,
    pub(super) synthetic_pressure_evidence: Vec<FoundationalDiagnosticSyntheticPressureEvidence>,
    pub(super) compile_fail_boundaries: Vec<FoundationalDiagnosticCompileFailBoundary>,
    pub(super) compile_fail_evidence: Vec<FoundationalDiagnosticCompileFailEvidence>,
    pub(super) canonical_golden_artifacts: Vec<FoundationalDiagnosticCanonicalGoldenArtifact>,
    pub(super) canonical_golden_artifact_evidence:
        Vec<FoundationalDiagnosticCanonicalGoldenArtifactEvidence>,
    pub(super) property_seed_inventory: Vec<FoundationalDiagnosticPropertySeed>,
    pub(super) property_seed_evidence: Vec<FoundationalDiagnosticPropertySeedEvidence>,
    pub(super) harness_expansion_points: Vec<FoundationalDiagnosticHarnessExpansionPoint>,
    pub(super) harness_expansion_evidence: Vec<FoundationalDiagnosticHarnessExpansionEvidence>,
    pub(super) forge_proof_required_surfaces: Vec<FoundationalDiagnosticForgeProofSurface>,
    pub(super) forge_proof_api_appendix: Vec<FoundationalDiagnosticForgeProofApi>,
    pub(super) forge_proof_api_evidence: Vec<FoundationalDiagnosticForgeProofApiEvidence>,
    pub(super) forge_proof_forbidden_surfaces:
        Vec<FoundationalDiagnosticForgeProofForbiddenSurface>,
    pub(super) assumptions: Vec<FoundationalDiagnosticRuntimeAssumption>,
    pub(super) non_assumptions: Vec<FoundationalDiagnosticRuntimeNonAssumption>,
    pub(super) runtime_adoption_failure_pressures:
        Vec<FoundationalDiagnosticRuntimeAdoptionFailurePressure>,
    pub(super) residual_debt: Vec<FoundationalDiagnosticResidualDebt>,
    pub(super) adoption_shaped_followthrough:
        Vec<FoundationalDiagnosticAdoptionShapedFollowthrough>,
    pub(super) phase_gates: Vec<FoundationalDiagnosticPhaseGateEvidence>,
}

impl FoundationalDiagnosticProductionReadinessReport {
    pub(super) fn new() -> Self {
        Self {
            certified_surfaces: certified_surfaces(),
            certified_surface_evidence: certified_surface_evidence(),
            synthetic_pressures: synthetic_pressures(),
            synthetic_pressure_evidence: synthetic_pressure_evidence(),
            compile_fail_boundaries: compile_fail_boundaries(),
            compile_fail_evidence: compile_fail_evidence(),
            canonical_golden_artifacts: canonical_golden_artifacts(),
            canonical_golden_artifact_evidence: canonical_golden_artifact_evidence(),
            property_seed_inventory: property_seed_inventory(),
            property_seed_evidence: property_seed_evidence(),
            harness_expansion_points: harness_expansion_points(),
            harness_expansion_evidence: harness_expansion_evidence(),
            forge_proof_required_surfaces: forge_proof_required_surfaces(),
            forge_proof_api_appendix: forge_proof_api_appendix(),
            forge_proof_api_evidence: forge_proof_api_evidence(),
            forge_proof_forbidden_surfaces: forge_proof_forbidden_surfaces(),
            assumptions: runtime_assumptions(),
            non_assumptions: runtime_non_assumptions(),
            runtime_adoption_failure_pressures: runtime_adoption_failure_pressures(),
            residual_debt: residual_debt(),
            adoption_shaped_followthrough: adoption_shaped_followthrough(),
            phase_gates: phase_gates(),
        }
    }

    pub fn certified_surfaces(&self) -> &[FoundationalDiagnosticCertifiedSurface] {
        &self.certified_surfaces
    }

    pub fn certified_surface_evidence(&self) -> &[FoundationalDiagnosticCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }

    pub fn synthetic_pressures(&self) -> &[FoundationalDiagnosticSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }

    pub fn synthetic_pressure_evidence(
        &self,
    ) -> &[FoundationalDiagnosticSyntheticPressureEvidence] {
        &self.synthetic_pressure_evidence
    }

    pub fn compile_fail_boundaries(&self) -> &[FoundationalDiagnosticCompileFailBoundary] {
        &self.compile_fail_boundaries
    }

    pub fn compile_fail_evidence(&self) -> &[FoundationalDiagnosticCompileFailEvidence] {
        &self.compile_fail_evidence
    }

    pub fn canonical_golden_artifacts(&self) -> &[FoundationalDiagnosticCanonicalGoldenArtifact] {
        &self.canonical_golden_artifacts
    }

    pub fn canonical_golden_artifact_evidence(
        &self,
    ) -> &[FoundationalDiagnosticCanonicalGoldenArtifactEvidence] {
        &self.canonical_golden_artifact_evidence
    }

    pub fn property_seed_inventory(&self) -> &[FoundationalDiagnosticPropertySeed] {
        &self.property_seed_inventory
    }

    pub fn property_seed_evidence(&self) -> &[FoundationalDiagnosticPropertySeedEvidence] {
        &self.property_seed_evidence
    }

    pub fn harness_expansion_points(&self) -> &[FoundationalDiagnosticHarnessExpansionPoint] {
        &self.harness_expansion_points
    }

    pub fn harness_expansion_evidence(&self) -> &[FoundationalDiagnosticHarnessExpansionEvidence] {
        &self.harness_expansion_evidence
    }

    pub fn forge_proof_required_surfaces(&self) -> &[FoundationalDiagnosticForgeProofSurface] {
        &self.forge_proof_required_surfaces
    }

    pub fn forge_proof_api_appendix(&self) -> &[FoundationalDiagnosticForgeProofApi] {
        &self.forge_proof_api_appendix
    }

    pub fn forge_proof_api_evidence(&self) -> &[FoundationalDiagnosticForgeProofApiEvidence] {
        &self.forge_proof_api_evidence
    }

    pub fn forge_proof_forbidden_surfaces(
        &self,
    ) -> &[FoundationalDiagnosticForgeProofForbiddenSurface] {
        &self.forge_proof_forbidden_surfaces
    }

    pub fn assumptions(&self) -> &[FoundationalDiagnosticRuntimeAssumption] {
        &self.assumptions
    }

    pub fn non_assumptions(&self) -> &[FoundationalDiagnosticRuntimeNonAssumption] {
        &self.non_assumptions
    }

    pub fn runtime_adoption_failure_pressures(
        &self,
    ) -> &[FoundationalDiagnosticRuntimeAdoptionFailurePressure] {
        &self.runtime_adoption_failure_pressures
    }

    pub fn residual_debt(&self) -> &[FoundationalDiagnosticResidualDebt] {
        &self.residual_debt
    }

    pub fn adoption_shaped_followthrough(
        &self,
    ) -> &[FoundationalDiagnosticAdoptionShapedFollowthrough] {
        &self.adoption_shaped_followthrough
    }

    pub fn phase_gates(&self) -> &[FoundationalDiagnosticPhaseGateEvidence] {
        &self.phase_gates
    }
}
