use super::inventory;
use super::scoped_inventory;
use super::vocabulary::{
    FoundationalTransitionCertifiedSurface, FoundationalTransitionCertifiedSurfaceEvidence,
    FoundationalTransitionCompileFailBoundary, FoundationalTransitionCompileFailEvidence,
    FoundationalTransitionWORTHProofApi, FoundationalTransitionWORTHProofApiEvidence,
    FoundationalTransitionWORTHProofForbiddenSurface, FoundationalTransitionWORTHProofSurface,
    FoundationalTransitionMilestone5PhaseGate, FoundationalTransitionPhaseGateEvidence,
    FoundationalTransitionProductionReadinessScope, FoundationalTransitionResidualDebt,
    FoundationalTransitionRuntimeAssumption, FoundationalTransitionRuntimeNonAssumption,
    FoundationalTransitionSyntheticPressureEvidence,
    FoundationalTransitionSyntheticRuntimePressure,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalTransitionProductionReadinessReport {
    scope: FoundationalTransitionProductionReadinessScope,
    certified_surfaces: Vec<FoundationalTransitionCertifiedSurface>,
    certified_surface_evidence: Vec<FoundationalTransitionCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<FoundationalTransitionSyntheticRuntimePressure>,
    synthetic_pressure_evidence: Vec<FoundationalTransitionSyntheticPressureEvidence>,
    compile_fail_boundaries: Vec<FoundationalTransitionCompileFailBoundary>,
    compile_fail_evidence: Vec<FoundationalTransitionCompileFailEvidence>,
    worth_proof_required_surfaces: Vec<FoundationalTransitionWORTHProofSurface>,
    worth_proof_api_appendix: Vec<FoundationalTransitionWORTHProofApi>,
    worth_proof_api_evidence: Vec<FoundationalTransitionWORTHProofApiEvidence>,
    worth_proof_forbidden_surfaces: Vec<FoundationalTransitionWORTHProofForbiddenSurface>,
    assumptions: Vec<FoundationalTransitionRuntimeAssumption>,
    non_assumptions: Vec<FoundationalTransitionRuntimeNonAssumption>,
    residual_debt: Vec<FoundationalTransitionResidualDebt>,
    phase_gates: Vec<FoundationalTransitionPhaseGateEvidence>,
}

impl FoundationalTransitionProductionReadinessReport {
    pub(super) fn milestone_5() -> Self {
        Self {
            scope: FoundationalTransitionProductionReadinessScope::milestone_5(),
            certified_surfaces: inventory::certified_surfaces(),
            certified_surface_evidence: inventory::certified_surface_evidence(),
            synthetic_pressures: inventory::synthetic_pressures(),
            synthetic_pressure_evidence: inventory::synthetic_pressure_evidence(),
            compile_fail_boundaries: inventory::compile_fail_boundaries(),
            compile_fail_evidence: inventory::compile_fail_evidence(),
            worth_proof_required_surfaces: inventory::worth_proof_required_surfaces(),
            worth_proof_api_appendix: inventory::worth_proof_api_appendix(),
            worth_proof_api_evidence: inventory::worth_proof_api_evidence(),
            worth_proof_forbidden_surfaces: inventory::worth_proof_forbidden_surfaces(),
            assumptions: inventory::runtime_assumptions(),
            non_assumptions: inventory::runtime_non_assumptions(),
            residual_debt: inventory::residual_debt(),
            phase_gates: inventory::phase_gates(),
        }
    }

    pub(super) fn milestone_9_scoped_merge() -> Self {
        Self {
            scope: FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge(),
            certified_surfaces: scoped_inventory::certified_surfaces(),
            certified_surface_evidence: scoped_inventory::certified_surface_evidence(),
            synthetic_pressures: scoped_inventory::synthetic_pressures(),
            synthetic_pressure_evidence: scoped_inventory::synthetic_pressure_evidence(),
            compile_fail_boundaries: scoped_inventory::compile_fail_boundaries(),
            compile_fail_evidence: scoped_inventory::compile_fail_evidence(),
            worth_proof_required_surfaces: scoped_inventory::worth_proof_required_surfaces(),
            worth_proof_api_appendix: scoped_inventory::worth_proof_api_appendix(),
            worth_proof_api_evidence: scoped_inventory::worth_proof_api_evidence(),
            worth_proof_forbidden_surfaces: scoped_inventory::worth_proof_forbidden_surfaces(),
            assumptions: scoped_inventory::runtime_assumptions(),
            non_assumptions: scoped_inventory::runtime_non_assumptions(),
            residual_debt: scoped_inventory::residual_debt(),
            phase_gates: scoped_inventory::phase_gates(),
        }
    }

    pub const fn scope(&self) -> FoundationalTransitionProductionReadinessScope {
        self.scope
    }

    pub fn certified_surfaces(&self) -> &[FoundationalTransitionCertifiedSurface] {
        &self.certified_surfaces
    }

    pub fn certified_surface_evidence(&self) -> &[FoundationalTransitionCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }

    pub fn synthetic_pressures(&self) -> &[FoundationalTransitionSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }

    pub fn synthetic_pressure_evidence(
        &self,
    ) -> &[FoundationalTransitionSyntheticPressureEvidence] {
        &self.synthetic_pressure_evidence
    }

    pub fn compile_fail_boundaries(&self) -> &[FoundationalTransitionCompileFailBoundary] {
        &self.compile_fail_boundaries
    }

    pub fn compile_fail_evidence(&self) -> &[FoundationalTransitionCompileFailEvidence] {
        &self.compile_fail_evidence
    }

    pub fn worth_proof_required_surfaces(&self) -> &[FoundationalTransitionWORTHProofSurface] {
        &self.worth_proof_required_surfaces
    }

    pub fn worth_proof_api_appendix(&self) -> &[FoundationalTransitionWORTHProofApi] {
        &self.worth_proof_api_appendix
    }

    pub fn worth_proof_api_evidence(&self) -> &[FoundationalTransitionWORTHProofApiEvidence] {
        &self.worth_proof_api_evidence
    }

    pub fn worth_proof_forbidden_surfaces(
        &self,
    ) -> &[FoundationalTransitionWORTHProofForbiddenSurface] {
        &self.worth_proof_forbidden_surfaces
    }

    pub fn assumptions(&self) -> &[FoundationalTransitionRuntimeAssumption] {
        &self.assumptions
    }

    pub fn non_assumptions(&self) -> &[FoundationalTransitionRuntimeNonAssumption] {
        &self.non_assumptions
    }

    pub fn residual_debt(&self) -> &[FoundationalTransitionResidualDebt] {
        &self.residual_debt
    }

    pub fn phase_gates(&self) -> &[FoundationalTransitionPhaseGateEvidence] {
        &self.phase_gates
    }

    pub fn passes_readiness_checklist(&self) -> bool {
        has_exact_inventory(
            &self.certified_surfaces,
            &required_certified_surfaces(self.scope),
        ) && has_one_evidence_per_surface(
            &self.certified_surfaces,
            &self.certified_surface_evidence,
        ) && has_exact_inventory(
            &self.synthetic_pressures,
            &required_synthetic_pressures(self.scope),
        ) && has_one_evidence_per_pressure(
            &self.synthetic_pressures,
            &self.synthetic_pressure_evidence,
        ) && has_exact_inventory(
            &self.compile_fail_boundaries,
            &required_compile_fail_boundaries(self.scope),
        ) && has_one_evidence_per_boundary(
            &self.compile_fail_boundaries,
            &self.compile_fail_evidence,
        ) && has_exact_inventory(
            &self.worth_proof_required_surfaces,
            &required_worth_proof_surfaces(self.scope),
        ) && has_exact_inventory(
            &self.worth_proof_api_appendix,
            &required_worth_proof_api(self.scope),
        ) && has_one_evidence_per_worth_proof_api(
            &self.worth_proof_api_appendix,
            &self.worth_proof_api_evidence,
        ) && has_exact_inventory(
            &self.worth_proof_forbidden_surfaces,
            &required_forbidden_worth_proof_surfaces(self.scope),
        ) && has_exact_inventory(&self.assumptions, &required_runtime_assumptions(self.scope))
            && has_exact_inventory(
                &self.non_assumptions,
                &required_runtime_non_assumptions(self.scope),
            )
            && has_exact_inventory(&self.residual_debt, &required_residual_debt(self.scope))
            && self
                .phase_gates
                .iter()
                .map(|evidence| evidence.gate())
                .eq(required_phase_gates(self.scope))
    }
}

fn has_exact_inventory<T: PartialEq>(actual: &[T], required: &[T]) -> bool {
    actual.len() == required.len() && required.iter().all(|value| actual.contains(value))
}

fn has_one_evidence_per_surface(
    surfaces: &[FoundationalTransitionCertifiedSurface],
    evidence: &[FoundationalTransitionCertifiedSurfaceEvidence],
) -> bool {
    evidence.len() == surfaces.len()
        && surfaces.iter().all(|surface| {
            evidence
                .iter()
                .filter(|row| row.surface() == *surface)
                .count()
                == 1
        })
}

fn has_one_evidence_per_pressure(
    pressures: &[FoundationalTransitionSyntheticRuntimePressure],
    evidence: &[FoundationalTransitionSyntheticPressureEvidence],
) -> bool {
    evidence.len() == pressures.len()
        && pressures.iter().all(|pressure| {
            evidence
                .iter()
                .filter(|row| row.pressure() == *pressure)
                .count()
                == 1
        })
}

fn has_one_evidence_per_boundary(
    boundaries: &[FoundationalTransitionCompileFailBoundary],
    evidence: &[FoundationalTransitionCompileFailEvidence],
) -> bool {
    evidence.len() == boundaries.len()
        && boundaries.iter().all(|boundary| {
            evidence
                .iter()
                .filter(|row| row.boundary() == *boundary)
                .count()
                == 1
        })
}

fn has_one_evidence_per_worth_proof_api(
    apis: &[FoundationalTransitionWORTHProofApi],
    evidence: &[FoundationalTransitionWORTHProofApiEvidence],
) -> bool {
    evidence.len() == apis.len()
        && apis
            .iter()
            .all(|api| evidence.iter().filter(|row| row.api() == *api).count() == 1)
}

fn required_certified_surfaces(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionCertifiedSurface> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::certified_surfaces();
    }
    inventory::certified_surfaces()
}

fn required_synthetic_pressures(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionSyntheticRuntimePressure> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::synthetic_pressures();
    }
    inventory::synthetic_pressures()
}

fn required_compile_fail_boundaries(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionCompileFailBoundary> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::compile_fail_boundaries();
    }
    inventory::compile_fail_boundaries()
}

fn required_worth_proof_surfaces(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionWORTHProofSurface> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::worth_proof_required_surfaces();
    }
    inventory::worth_proof_required_surfaces()
}

fn required_worth_proof_api(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionWORTHProofApi> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::worth_proof_api_appendix();
    }
    inventory::worth_proof_api_appendix()
}

fn required_forbidden_worth_proof_surfaces(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionWORTHProofForbiddenSurface> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::worth_proof_forbidden_surfaces();
    }
    inventory::worth_proof_forbidden_surfaces()
}

fn required_runtime_assumptions(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionRuntimeAssumption> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::runtime_assumptions();
    }
    inventory::runtime_assumptions()
}

fn required_runtime_non_assumptions(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionRuntimeNonAssumption> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::runtime_non_assumptions();
    }
    inventory::runtime_non_assumptions()
}

fn required_residual_debt(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionResidualDebt> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::residual_debt();
    }
    inventory::residual_debt()
}

fn required_phase_gates(
    scope: FoundationalTransitionProductionReadinessScope,
) -> Vec<FoundationalTransitionMilestone5PhaseGate> {
    if scope == FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge() {
        return scoped_inventory::phase_gates()
            .iter()
            .map(|evidence| evidence.gate())
            .collect();
    }
    inventory::phase_gates()
        .iter()
        .map(|evidence| evidence.gate())
        .collect()
}
