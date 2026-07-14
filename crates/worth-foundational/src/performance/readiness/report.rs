use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries,
    documentation_surface_inventory, harness_expansion_points, phase_gates,
    public_surface_compile_fail_path, public_surface_documentation_coverage,
    public_surface_evidence_path, public_surface_inventory, residual_debt,
    runtime_adoption_pressure_evidence, runtime_adoption_pressures, runtime_assumptions,
    runtime_non_assumptions, synthetic_pressures, worth_proof_api_appendix,
    worth_proof_forbidden_surfaces, worth_proof_required_surfaces,
};
use super::vocabulary::{
    FoundationalPerformanceCertifiedSurface, FoundationalPerformanceCertifiedSurfaceEvidence,
    FoundationalPerformanceCompileFailBoundary, FoundationalPerformanceHarnessExpansionPoint,
    FoundationalPerformancePhaseGateEvidence,
    FoundationalPerformancePublicSurfaceDocumentationCoverage, FoundationalPerformanceResidualDebt,
    FoundationalPerformanceRuntimeAdoptionPressure,
    FoundationalPerformanceRuntimeAdoptionPressureEvidence,
    FoundationalPerformanceRuntimeAssumption, FoundationalPerformanceRuntimeNonAssumption,
    FoundationalPerformanceSyntheticRuntimePressure, FoundationalPerformanceWORTHProofApi,
    FoundationalPerformanceWORTHProofForbiddenSurface, FoundationalPerformanceWORTHProofSurface,
};
use crate::performance_api::FoundationalPerformancePublicSurfaceEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceProductionReadinessReport {
    certified_surfaces: Vec<FoundationalPerformanceCertifiedSurface>,
    certified_surface_evidence: Vec<FoundationalPerformanceCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<FoundationalPerformanceSyntheticRuntimePressure>,
    compile_fail_boundaries: Vec<FoundationalPerformanceCompileFailBoundary>,
    worth_proof_required_surfaces: Vec<FoundationalPerformanceWORTHProofSurface>,
    worth_proof_api_appendix: Vec<FoundationalPerformanceWORTHProofApi>,
    worth_proof_forbidden_surfaces: Vec<FoundationalPerformanceWORTHProofForbiddenSurface>,
    assumptions: Vec<FoundationalPerformanceRuntimeAssumption>,
    non_assumptions: Vec<FoundationalPerformanceRuntimeNonAssumption>,
    residual_debt: Vec<FoundationalPerformanceResidualDebt>,
    runtime_adoption_pressures: Vec<FoundationalPerformanceRuntimeAdoptionPressure>,
    runtime_adoption_pressure_evidence: Vec<FoundationalPerformanceRuntimeAdoptionPressureEvidence>,
    phase_gates: Vec<FoundationalPerformancePhaseGateEvidence>,
    harness_expansion_points: Vec<FoundationalPerformanceHarnessExpansionPoint>,
    public_surface_inventory: Vec<FoundationalPerformancePublicSurfaceEntry>,
    documentation_surface_inventory: Vec<&'static str>,
    public_surface_documentation_coverage:
        Vec<FoundationalPerformancePublicSurfaceDocumentationCoverage>,
    public_surface_evidence_path: &'static str,
    public_surface_compile_fail_path: &'static str,
}

impl FoundationalPerformanceProductionReadinessReport {
    pub(super) fn new() -> Self {
        Self {
            certified_surfaces: certified_surfaces(),
            certified_surface_evidence: certified_surface_evidence(),
            synthetic_pressures: synthetic_pressures(),
            compile_fail_boundaries: compile_fail_boundaries(),
            worth_proof_required_surfaces: worth_proof_required_surfaces(),
            worth_proof_api_appendix: worth_proof_api_appendix(),
            worth_proof_forbidden_surfaces: worth_proof_forbidden_surfaces(),
            assumptions: runtime_assumptions(),
            non_assumptions: runtime_non_assumptions(),
            residual_debt: residual_debt(),
            runtime_adoption_pressures: runtime_adoption_pressures(),
            runtime_adoption_pressure_evidence: runtime_adoption_pressure_evidence(),
            phase_gates: phase_gates(),
            harness_expansion_points: harness_expansion_points(),
            public_surface_inventory: public_surface_inventory(),
            documentation_surface_inventory: documentation_surface_inventory(),
            public_surface_documentation_coverage: public_surface_documentation_coverage(),
            public_surface_evidence_path: public_surface_evidence_path(),
            public_surface_compile_fail_path: public_surface_compile_fail_path(),
        }
    }

    pub fn certified_surfaces(&self) -> &[FoundationalPerformanceCertifiedSurface] {
        &self.certified_surfaces
    }

    pub fn certified_surface_evidence(&self) -> &[FoundationalPerformanceCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }

    pub fn synthetic_pressures(&self) -> &[FoundationalPerformanceSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }

    pub fn compile_fail_boundaries(&self) -> &[FoundationalPerformanceCompileFailBoundary] {
        &self.compile_fail_boundaries
    }

    pub fn worth_proof_required_surfaces(&self) -> &[FoundationalPerformanceWORTHProofSurface] {
        &self.worth_proof_required_surfaces
    }

    pub fn worth_proof_api_appendix(&self) -> &[FoundationalPerformanceWORTHProofApi] {
        &self.worth_proof_api_appendix
    }

    pub fn worth_proof_forbidden_surfaces(
        &self,
    ) -> &[FoundationalPerformanceWORTHProofForbiddenSurface] {
        &self.worth_proof_forbidden_surfaces
    }

    pub fn assumptions(&self) -> &[FoundationalPerformanceRuntimeAssumption] {
        &self.assumptions
    }

    pub fn non_assumptions(&self) -> &[FoundationalPerformanceRuntimeNonAssumption] {
        &self.non_assumptions
    }

    pub fn residual_debt(&self) -> &[FoundationalPerformanceResidualDebt] {
        &self.residual_debt
    }

    pub fn runtime_adoption_pressures(&self) -> &[FoundationalPerformanceRuntimeAdoptionPressure] {
        &self.runtime_adoption_pressures
    }

    pub fn runtime_adoption_pressure_evidence(
        &self,
    ) -> &[FoundationalPerformanceRuntimeAdoptionPressureEvidence] {
        &self.runtime_adoption_pressure_evidence
    }

    pub fn phase_gates(&self) -> &[FoundationalPerformancePhaseGateEvidence] {
        &self.phase_gates
    }

    pub fn harness_expansion_points(&self) -> &[FoundationalPerformanceHarnessExpansionPoint] {
        &self.harness_expansion_points
    }

    pub fn public_surface_inventory(&self) -> &[FoundationalPerformancePublicSurfaceEntry] {
        &self.public_surface_inventory
    }

    pub fn documentation_surface_inventory(&self) -> &[&'static str] {
        &self.documentation_surface_inventory
    }

    pub fn public_surface_documentation_coverage(
        &self,
    ) -> &[FoundationalPerformancePublicSurfaceDocumentationCoverage] {
        &self.public_surface_documentation_coverage
    }

    pub fn public_surface_evidence_path(&self) -> &'static str {
        self.public_surface_evidence_path
    }

    pub fn public_surface_compile_fail_path(&self) -> &'static str {
        self.public_surface_compile_fail_path
    }
}

#[cfg(test)]
impl FoundationalPerformanceProductionReadinessReport {
    pub(super) fn with_runtime_adoption_pressure_evidence(
        mut self,
        runtime_adoption_pressure_evidence: Vec<
            FoundationalPerformanceRuntimeAdoptionPressureEvidence,
        >,
    ) -> Self {
        self.runtime_adoption_pressure_evidence = runtime_adoption_pressure_evidence;
        self
    }

    pub(super) fn with_public_surface_inventory(
        mut self,
        public_surface_inventory: Vec<FoundationalPerformancePublicSurfaceEntry>,
    ) -> Self {
        self.public_surface_inventory = public_surface_inventory;
        self
    }
}
