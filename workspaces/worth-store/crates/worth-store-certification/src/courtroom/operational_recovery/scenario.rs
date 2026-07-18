use std::collections::BTreeSet;

use super::phase_invocation::derive_phase_invocations;
use super::scenario_audit_binding::require_audits_from_control_history;
use super::scenario_counter_binding::require_operation_counter_bindings;
use super::scenario_identity::{
    audit_set_identity, evidence_identity, phase_16_identity, phase_17_identity, phase_18_identity,
    phase_19_identity, S10ScenarioIdentityInputs,
};
use super::scenario_mutation_requirements::require_mutation_families;
use super::scenario_owner_topology::require_scenario_owner_topology;
use super::scenario_trace_binding::require_production_trace_binding;
mod requirements;
use super::{
    S10HostileProgramEvidence, S10OperationalQosEvidence, S10OperationalScenarioKind,
    S10OperationalScenarioProgram, S10PhaseInvocationDenial, S10PhaseInvocationEvidence,
    S10ScenarioExecutionMatrix, S10ScenarioProductionEvidence, S10StructuralPreflightEvidence,
    ScenarioScaleDenial, ScenarioScaleEvidence,
};
use requirements::{require_counter_kinds, required_model_transitions, required_yieldpoints};
use worth_store_formal_models::{
    check_operational_recovery_mutation_sensitivity, OperationalRecoveryActionKind,
    OperationalRecoveryMutationSensitivitySuite, OperationalRecoveryRefinementReceipt,
};
use worth_store_operations::{
    AuditCompletenessReceipt, OperationalCounterReceipt, OperationalSessionIdentity,
    OperationalSessionKind,
};
use worth_store_physical_certification::{
    OperationalRecoveryControlTransitionKind, OperationalRecoveryDriverTrace,
    OperationalRecoveryYieldpoint, PhysicalCertificationEvidenceBundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S10ScenarioCertificationDenial {
    Scale(ScenarioScaleDenial),
    MissingYieldpoint(OperationalRecoveryYieldpoint),
    MissingDriverControlArtifact([u8; 32]),
    MissingControlledProductionDefect,
    ExecutionScenarioMismatch,
    ScenarioOwnerTopologyMismatch { expected: u8, observed: u8 },
    HostileProgramScenarioMismatch,
    HostileProgramExecutionMismatch,
    DriverControlTransitionCountMismatch(OperationalRecoveryControlTransitionKind),
    DriverInspectionEvidenceMismatch,
    DriverTruthEvidenceMismatch,
    DriverModelOperationMismatch,
    MissingOperationCounters,
    DuplicateOperationCounterSession(OperationalSessionIdentity),
    CounterModelOperationMismatch,
    InvalidCounterStructure(OperationalSessionKind),
    ForeignWorkInOperationCounters,
    ForbiddenMaterializationObserved,
    MissingModelTransition(OperationalRecoveryActionKind),
    MissingMutationFamily(worth_store_formal_models::OperationalRecoveryModelFamily),
    MutationSensitivity(worth_store_formal_models::OperationalRecoveryMutationSensitivityDenial),
    AuditNotDerivedFromScenarioControlHistory,
    PhaseInvocation(S10PhaseInvocationDenial),
}

#[derive(Debug, Clone)]
pub struct S10OperationalScenarioEvidence {
    program: S10OperationalScenarioProgram,
    hostile_program: S10HostileProgramEvidence,
    scale: ScenarioScaleEvidence,
    execution: S10ScenarioExecutionMatrix,
    refinement: OperationalRecoveryRefinementReceipt,
    mutation_sensitivity: OperationalRecoveryMutationSensitivitySuite,
    qos: S10OperationalQosEvidence,
    counters: Vec<OperationalCounterReceipt>,
    audits: Vec<AuditCompletenessReceipt>,
    phase_invocations: Vec<S10PhaseInvocationEvidence>,
    evidence_identity: [u8; 32],
}

#[allow(clippy::too_many_arguments)]
pub fn certify_s10_operational_scenario(
    program: S10OperationalScenarioProgram,
    preflight: &S10StructuralPreflightEvidence,
    production: S10ScenarioProductionEvidence<'_>,
    hostile_program: S10HostileProgramEvidence,
    execution: S10ScenarioExecutionMatrix,
    qos: S10OperationalQosEvidence,
    mut counters: Vec<OperationalCounterReceipt>,
    mut audits: Vec<AuditCompletenessReceipt>,
) -> Result<S10OperationalScenarioEvidence, S10ScenarioCertificationDenial> {
    let scale = ScenarioScaleEvidence::from_execution(program.profile(), &execution)
        .map_err(S10ScenarioCertificationDenial::Scale)?;
    if execution.scenario_kind() != Some(program.kind()) {
        return Err(S10ScenarioCertificationDenial::ExecutionScenarioMismatch);
    }
    let (refinement, mutation_sensitivity) =
        check_operational_recovery_mutation_sensitivity(production.control_records())
            .map_err(S10ScenarioCertificationDenial::MutationSensitivity)?;
    let driver_trace = execution.driver_trace();
    require_scenario_owner_topology(program.kind(), production.control_records())?;
    if hostile_program.kind() != program.kind() {
        return Err(S10ScenarioCertificationDenial::HostileProgramScenarioMismatch);
    }
    if !hostile_program.matches_crash_coverage(execution.crash_reopen_coverage()) {
        return Err(S10ScenarioCertificationDenial::HostileProgramExecutionMismatch);
    }
    if execution.controlled_defects().is_empty() {
        return Err(S10ScenarioCertificationDenial::MissingControlledProductionDefect);
    }
    require_production_trace_binding(production, driver_trace)?;
    require_audits_from_control_history(production, &audits)?;
    let reached = driver_trace
        .reached()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for point in required_yieldpoints(program.kind()) {
        if !reached.contains(&point) {
            return Err(S10ScenarioCertificationDenial::MissingYieldpoint(point));
        }
    }
    require_operation_counter_bindings(&refinement, driver_trace, &counters)?;
    if counters
        .iter()
        .any(|receipt| receipt.foreign_work_units() != 0)
    {
        return Err(S10ScenarioCertificationDenial::ForeignWorkInOperationCounters);
    }
    if counters
        .iter()
        .any(|receipt| receipt.forbidden_full_materializations() != 0)
    {
        return Err(S10ScenarioCertificationDenial::ForbiddenMaterializationObserved);
    }
    for receipt in &counters {
        if receipt.validate_structure().is_err() {
            return Err(S10ScenarioCertificationDenial::InvalidCounterStructure(
                receipt.kind(),
            ));
        }
    }
    require_counter_kinds(program.kind(), &counters)?;
    for transition in required_model_transitions(program.kind()) {
        if !refinement.reached_model_transitions().contains(&transition) {
            return Err(S10ScenarioCertificationDenial::MissingModelTransition(
                transition,
            ));
        }
    }
    require_mutation_families(program.kind(), &mutation_sensitivity)?;
    counters.sort_by_key(|counter| counter.session());
    audits.sort_by(|left, right| left.operation_id().cmp(right.operation_id()));
    let phase_15_identity = audit_set_identity(&audits);
    let phase_16_identity = phase_16_identity(&execution);
    let phase_17_identity = phase_17_identity(&refinement, &mutation_sensitivity);
    let phase_18_identity = phase_18_identity(scale, &qos, &counters);
    let phase_19_identity = phase_19_identity(
        program,
        phase_15_identity,
        phase_16_identity,
        phase_17_identity,
        phase_18_identity,
    );
    let phase_invocations = derive_phase_invocations(
        program.kind(),
        preflight,
        production,
        [
            phase_15_identity,
            phase_16_identity,
            phase_17_identity,
            phase_18_identity,
            phase_19_identity,
        ],
    )
    .map_err(S10ScenarioCertificationDenial::PhaseInvocation)?;
    let evidence_identity = evidence_identity(&S10ScenarioIdentityInputs {
        program,
        hostile_program,
        scale,
        execution: &execution,
        refinement: &refinement,
        mutation_sensitivity: &mutation_sensitivity,
        qos: &qos,
        audits: &audits,
        counters: &counters,
        phase_invocations: &phase_invocations,
    });
    Ok(S10OperationalScenarioEvidence {
        program,
        hostile_program,
        scale,
        execution,
        refinement,
        mutation_sensitivity,
        qos,
        counters,
        audits,
        phase_invocations,
        evidence_identity,
    })
}

pub fn required_s10_crash_reopen_yieldpoints(
    kind: S10OperationalScenarioKind,
) -> Vec<OperationalRecoveryYieldpoint> {
    required_yieldpoints(kind)
}

impl S10OperationalScenarioEvidence {
    pub const fn program(&self) -> S10OperationalScenarioProgram {
        self.program
    }
    pub const fn hostile_program(&self) -> S10HostileProgramEvidence {
        self.hostile_program
    }
    pub const fn scale(&self) -> ScenarioScaleEvidence {
        self.scale
    }
    pub const fn evidence_identity(&self) -> [u8; 32] {
        self.evidence_identity
    }
    pub fn physical(&self) -> &PhysicalCertificationEvidenceBundle {
        self.execution.primary()
    }
    pub const fn driver_trace(&self) -> &OperationalRecoveryDriverTrace {
        self.execution.driver_trace()
    }
    pub const fn execution_matrix(&self) -> &S10ScenarioExecutionMatrix {
        &self.execution
    }
    pub fn missing_crash_reopen_yieldpoint(&self) -> Option<OperationalRecoveryYieldpoint> {
        let covered = self
            .execution
            .crash_reopen_coverage()
            .iter()
            .map(|evidence| evidence.yieldpoint())
            .collect::<BTreeSet<_>>();
        required_yieldpoints(self.program.kind())
            .into_iter()
            .find(|point| !covered.contains(point))
    }
    pub const fn refinement(&self) -> &OperationalRecoveryRefinementReceipt {
        &self.refinement
    }
    pub const fn mutation_sensitivity(&self) -> &OperationalRecoveryMutationSensitivitySuite {
        &self.mutation_sensitivity
    }
    pub const fn qos(&self) -> &S10OperationalQosEvidence {
        &self.qos
    }
    pub fn counters(&self) -> &[OperationalCounterReceipt] {
        &self.counters
    }
    pub fn audits(&self) -> &[AuditCompletenessReceipt] {
        &self.audits
    }
    pub fn phase_invocations(&self) -> &[S10PhaseInvocationEvidence] {
        &self.phase_invocations
    }
}
