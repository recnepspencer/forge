use std::collections::BTreeSet;

use super::phase_invocation::derive_phase_invocations;
use super::scenario_audit_binding::require_audits_from_control_history;
use super::scenario_counter_binding::require_operation_counter_bindings;
use super::scenario_identity::{
    audit_set_identity, evidence_identity, phase_16_identity, phase_17_identity, phase_18_identity,
    phase_19_identity, S10ScenarioIdentityInputs,
};
use super::scenario_mutation_requirements::require_mutation_families;
use super::scenario_trace_binding::require_production_trace_binding;
use super::{
    S10OperationalQosEvidence, S10OperationalScenarioKind, S10OperationalScenarioProgram,
    S10PhaseInvocationDenial, S10PhaseInvocationEvidence, S10ScenarioExecutionMatrix,
    S10ScenarioProductionEvidence, S10StructuralPreflightEvidence, ScenarioScaleDenial,
    ScenarioScaleEvidence,
};
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
    execution: S10ScenarioExecutionMatrix,
    qos: S10OperationalQosEvidence,
    mut counters: Vec<OperationalCounterReceipt>,
    mut audits: Vec<AuditCompletenessReceipt>,
) -> Result<S10OperationalScenarioEvidence, S10ScenarioCertificationDenial> {
    let scale = ScenarioScaleEvidence::from_execution(program.profile(), &execution)
        .map_err(S10ScenarioCertificationDenial::Scale)?;
    let (refinement, mutation_sensitivity) =
        check_operational_recovery_mutation_sensitivity(production.control_records())
            .map_err(S10ScenarioCertificationDenial::MutationSensitivity)?;
    let driver_trace = execution.driver_trace();
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

impl S10OperationalScenarioEvidence {
    pub const fn program(&self) -> S10OperationalScenarioProgram {
        self.program
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

fn required_yieldpoints(kind: S10OperationalScenarioKind) -> Vec<OperationalRecoveryYieldpoint> {
    OperationalRecoveryYieldpoint::ALL
        .into_iter()
        .filter(|point| scenario_uses_yieldpoint(kind, *point))
        .collect()
}

fn scenario_uses_yieldpoint(
    scenario: S10OperationalScenarioKind,
    point: OperationalRecoveryYieldpoint,
) -> bool {
    use OperationalRecoveryControlTransitionKind as Control;
    use OperationalRecoveryYieldpoint as Point;
    match point {
        Point::BeforeDurableControlTransition(control)
        | Point::AfterDurableControlTransition(control) => match control {
            Control::RepairExecutionOpen
            | Control::RepairOwnerEffect
            | Control::RepairOwnerReceipt
            | Control::RepairDisposition => scenario != S10OperationalScenarioKind::BurningPrimary,
            Control::WorkflowAbandonment => scenario == S10OperationalScenarioKind::BurningPrimary,
            Control::ReplicaBootstrapTransfer
            | Control::ReplicaBootstrapCompletion
            | Control::ReplicaPromotionFence
            | Control::ReplicaPromotionRecord
            | Control::ReplicaPromotionPublication
            | Control::ReplicaPromotionReadmission => {
                scenario != S10OperationalScenarioKind::AuthorityRepairRollback
            }
            Control::OldPrimaryRejoinPlan | Control::OldPrimaryRejoinCompletion => {
                scenario == S10OperationalScenarioKind::SplitBrainPromotion
            }
            _ => true,
        },
        Point::BeforeOldPrimaryRejoinPlan
        | Point::AfterOldPrimaryRejoinPlan
        | Point::BeforeOldPrimaryRejoinExecution
        | Point::AfterOldPrimaryRejoinExecution
        | Point::BeforeOldPrimaryRejoinCompletion
        | Point::AfterOldPrimaryRejoinCompletion => {
            scenario == S10OperationalScenarioKind::SplitBrainPromotion
        }
        Point::BeforeBootstrapTransfer
        | Point::AfterBootstrapTransfer
        | Point::BeforeBootstrapControlRecord
        | Point::AfterBootstrapControlRecord
        | Point::BeforeBootstrapPostVerification
        | Point::AfterBootstrapPostVerification
        | Point::BeforeBootstrapCompletion
        | Point::AfterBootstrapCompletion
        | Point::BeforePromotionExternalFence
        | Point::AfterPromotionExternalFence
        | Point::BeforePromotionFenceRecord
        | Point::AfterPromotionFenceRecord
        | Point::BeforePromotionRecord
        | Point::AfterPromotionRecord
        | Point::BeforePromotionPostVerification
        | Point::AfterPromotionPostVerification
        | Point::BeforePromotionPublication
        | Point::AfterPromotionPublication
        | Point::BeforePromotionReadmission
        | Point::AfterPromotionReadmission => {
            scenario != S10OperationalScenarioKind::AuthorityRepairRollback
        }
        _ => true,
    }
}

fn require_counter_kinds(
    kind: S10OperationalScenarioKind,
    counters: &[OperationalCounterReceipt],
) -> Result<(), S10ScenarioCertificationDenial> {
    let required = match kind {
        S10OperationalScenarioKind::BurningPrimary => vec![
            OperationalSessionKind::Backup,
            OperationalSessionKind::Restore,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::Rollback,
            OperationalSessionKind::ReplicaBootstrap,
            OperationalSessionKind::ReplicaPromotion,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
        S10OperationalScenarioKind::SplitBrainPromotion => vec![
            OperationalSessionKind::Backup,
            OperationalSessionKind::Restore,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::Repair,
            OperationalSessionKind::ReplicaBootstrap,
            OperationalSessionKind::ReplicaPromotion,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
        S10OperationalScenarioKind::AuthorityRepairRollback => vec![
            OperationalSessionKind::Backup,
            OperationalSessionKind::Restore,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::Rollback,
            OperationalSessionKind::Repair,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
    };
    for required_kind in required {
        if !counters
            .iter()
            .any(|receipt| receipt.kind() == required_kind)
        {
            return Err(S10ScenarioCertificationDenial::MissingOperationCounters);
        }
    }
    Ok(())
}

fn required_model_transitions(
    kind: S10OperationalScenarioKind,
) -> Vec<OperationalRecoveryActionKind> {
    use OperationalRecoveryActionKind as Action;
    let mut required = vec![
        Action::AuthorizationConsumed,
        Action::StagingCompleted,
        Action::PublicationPrepared,
        Action::PublicationPending,
        Action::PublicationDisposition,
        Action::FenceReleased,
    ];
    match kind {
        S10OperationalScenarioKind::BurningPrimary => required.extend([
            Action::SourceLeasePersisted,
            Action::MaterializationOpened,
            Action::MaterializationRecorded,
            Action::IndependentVerificationRecorded,
            Action::Abandoned,
            Action::WorkflowOwnerReceiptPersisted,
            Action::ReplicaBootstrapTransferRecorded,
            Action::ReplicaBootstrapCompleted,
            Action::ReplicaPromotionFenceRecorded,
            Action::ReplicaPromotionRecorded,
            Action::ReplicaPromotionPublished,
            Action::ReplicaPromotionReadmitted,
        ]),
        S10OperationalScenarioKind::SplitBrainPromotion => required.extend([
            Action::SourceLeasePersisted,
            Action::MaterializationOpened,
            Action::MaterializationRecorded,
            Action::IndependentVerificationRecorded,
            Action::WorkflowOwnerReceiptPersisted,
            Action::ReplicaBootstrapTransferRecorded,
            Action::ReplicaBootstrapCompleted,
            Action::ReplicaPromotionFenceRecorded,
            Action::ReplicaPromotionRecorded,
            Action::ReplicaPromotionPublished,
            Action::ReplicaPromotionReadmitted,
            Action::OldPrimaryRejoinPlanned,
            Action::OldPrimaryRejoinCompleted,
        ]),
        S10OperationalScenarioKind::AuthorityRepairRollback => required.extend([
            Action::SourceLeasePersisted,
            Action::MaterializationOpened,
            Action::MaterializationRecorded,
            Action::IndependentVerificationRecorded,
            Action::OwnerExecutionOpened,
            Action::OwnerEffectStarted,
            Action::OwnerReceiptPersisted,
            Action::WorkflowOwnerReceiptPersisted,
            Action::DispositionRecorded,
        ]),
    }
    required
}
