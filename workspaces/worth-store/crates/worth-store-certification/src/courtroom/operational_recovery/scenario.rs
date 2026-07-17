use std::collections::BTreeSet;

use super::{
    S10OperationalQosEvidence, S10OperationalScenarioKind, S10OperationalScenarioProgram,
    ScenarioScaleEvidence,
};
use sha2::{Digest, Sha256};
use worth_store_formal_models::{
    OperationalRecoveryActionKind, OperationalRecoveryMutationSensitivitySuite,
    OperationalRecoveryRefinementReceipt,
};
use worth_store_operations::{
    AuditCompletenessReceipt, OperationalCounterReceipt, OperationalOperationId,
    OperationalSessionIdentity, OperationalSessionKind,
};
use worth_store_physical_certification::{
    OperationalRecoveryDriverTrace, OperationalRecoveryYieldpoint,
    PhysicalCertificationEvidenceBundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S10ScenarioCertificationDenial {
    ProfileMismatch,
    PhysicalEvidenceIncomplete,
    MissingYieldpoint(OperationalRecoveryYieldpoint),
    MissingDriverOperationBinding,
    DriverModelOperationMismatch,
    AuditModelOperationMismatch,
    MissingOperationCounters,
    DuplicateOperationCounterKind(OperationalSessionKind),
    CounterModelOperationMismatch,
    InvalidCounterStructure(OperationalSessionKind),
    ForeignWorkInOperationCounters,
    ForbiddenMaterializationObserved,
    MissingModelTransition(OperationalRecoveryActionKind),
}

#[derive(Debug, Clone)]
pub struct S10OperationalScenarioEvidence {
    program: S10OperationalScenarioProgram,
    scale: ScenarioScaleEvidence,
    physical: PhysicalCertificationEvidenceBundle,
    driver_trace: OperationalRecoveryDriverTrace,
    refinement: OperationalRecoveryRefinementReceipt,
    mutation_sensitivity: OperationalRecoveryMutationSensitivitySuite,
    qos: S10OperationalQosEvidence,
    counters: Vec<OperationalCounterReceipt>,
    audit: AuditCompletenessReceipt,
    evidence_identity: [u8; 32],
}

#[allow(clippy::too_many_arguments)]
pub fn certify_s10_operational_scenario(
    program: S10OperationalScenarioProgram,
    scale: ScenarioScaleEvidence,
    physical: PhysicalCertificationEvidenceBundle,
    driver_trace: OperationalRecoveryDriverTrace,
    refinement: OperationalRecoveryRefinementReceipt,
    mutation_sensitivity: OperationalRecoveryMutationSensitivitySuite,
    qos: S10OperationalQosEvidence,
    counters: Vec<OperationalCounterReceipt>,
    audit: AuditCompletenessReceipt,
) -> Result<S10OperationalScenarioEvidence, S10ScenarioCertificationDenial> {
    if program.profile() != scale.profile() {
        return Err(S10ScenarioCertificationDenial::ProfileMismatch);
    }
    let primary = physical.primary();
    if primary.oracle_verdict_count() == 0 || primary.counter_row_count() == 0 {
        return Err(S10ScenarioCertificationDenial::PhysicalEvidenceIncomplete);
    }
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
    if !driver_trace
        .operation_identities()
        .iter()
        .any(|identity| identity == audit.operation_id().as_str())
    {
        return Err(S10ScenarioCertificationDenial::MissingDriverOperationBinding);
    }
    if driver_trace
        .operation_identities()
        .iter()
        .any(|identity| !refinement.operation_identities().contains(identity))
    {
        return Err(S10ScenarioCertificationDenial::DriverModelOperationMismatch);
    }
    if !refinement
        .operation_identities()
        .contains(audit.operation_id().as_str())
    {
        return Err(S10ScenarioCertificationDenial::AuditModelOperationMismatch);
    }
    let model_sessions = refinement
        .operation_identities()
        .iter()
        .map(|identity| OperationalOperationId::new(identity.clone()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| S10ScenarioCertificationDenial::CounterModelOperationMismatch)?
        .iter()
        .map(OperationalSessionIdentity::from_operation)
        .collect::<BTreeSet<_>>();
    if counters
        .iter()
        .any(|receipt| !model_sessions.contains(&receipt.session()))
    {
        return Err(S10ScenarioCertificationDenial::CounterModelOperationMismatch);
    }
    let session = OperationalSessionIdentity::from_operation(audit.operation_id());
    if !counters.iter().any(|receipt| receipt.session() == session) {
        return Err(S10ScenarioCertificationDenial::MissingOperationCounters);
    }
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
    let evidence_identity = evidence_identity(&S10ScenarioIdentityInputs {
        program,
        scale,
        physical: &physical,
        trace: &driver_trace,
        refinement: &refinement,
        mutation_sensitivity: &mutation_sensitivity,
        qos: &qos,
        audit: &audit,
        counters: &counters,
    });
    Ok(S10OperationalScenarioEvidence {
        program,
        scale,
        physical,
        driver_trace,
        refinement,
        mutation_sensitivity,
        qos,
        counters,
        audit,
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
    pub const fn physical(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.physical
    }
    pub const fn driver_trace(&self) -> &OperationalRecoveryDriverTrace {
        &self.driver_trace
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
    pub const fn audit(&self) -> &AuditCompletenessReceipt {
        &self.audit
    }
}

fn required_yieldpoints(kind: S10OperationalScenarioKind) -> Vec<OperationalRecoveryYieldpoint> {
    if kind != S10OperationalScenarioKind::AuthorityRepairRollback {
        return OperationalRecoveryYieldpoint::ALL.to_vec();
    }
    OperationalRecoveryYieldpoint::ALL
        .into_iter()
        .filter(|point| {
            matches!(
                point,
                OperationalRecoveryYieldpoint::BeforeForensicSourceAcquisition
                    | OperationalRecoveryYieldpoint::AfterForensicSourceRecord
                    | OperationalRecoveryYieldpoint::BeforeForensicFinalization
                    | OperationalRecoveryYieldpoint::AfterForensicFinalization
                    | OperationalRecoveryYieldpoint::BeforeAuditDerivation
                    | OperationalRecoveryYieldpoint::AfterAuditDerivation
                    | OperationalRecoveryYieldpoint::BeforeAuditExport
                    | OperationalRecoveryYieldpoint::AfterAuditExport
            )
        })
        .collect()
}

fn require_counter_kinds(
    kind: S10OperationalScenarioKind,
    counters: &[OperationalCounterReceipt],
) -> Result<(), S10ScenarioCertificationDenial> {
    let observed = counters
        .iter()
        .map(|counter| counter.kind())
        .collect::<BTreeSet<_>>();
    let required = match kind {
        S10OperationalScenarioKind::BurningPrimary => vec![
            OperationalSessionKind::Backup,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
        S10OperationalScenarioKind::SplitBrainPromotion => vec![
            OperationalSessionKind::ReplicaBootstrap,
            OperationalSessionKind::ReplicaPromotion,
            OperationalSessionKind::OfflineVerification,
        ],
        S10OperationalScenarioKind::AuthorityRepairRollback => vec![
            OperationalSessionKind::Restore,
            OperationalSessionKind::Rollback,
            OperationalSessionKind::Repair,
            OperationalSessionKind::OfflineVerification,
            OperationalSessionKind::ForensicAcquisition,
        ],
    };
    for required_kind in required {
        let count = counters
            .iter()
            .filter(|receipt| receipt.kind() == required_kind)
            .count();
        match count {
            1 => {}
            0 => return Err(S10ScenarioCertificationDenial::MissingOperationCounters),
            _ => {
                return Err(
                    S10ScenarioCertificationDenial::DuplicateOperationCounterKind(required_kind),
                )
            }
        }
    }
    if observed.len() != counters.len() {
        let duplicate = counters
            .iter()
            .map(|receipt| receipt.kind())
            .find(|candidate| {
                counters
                    .iter()
                    .filter(|receipt| receipt.kind() == *candidate)
                    .count()
                    > 1
            })
            .expect("a cardinality mismatch identifies a duplicate kind");
        return Err(S10ScenarioCertificationDenial::DuplicateOperationCounterKind(duplicate));
    }
    Ok(())
}

fn required_model_transitions(
    kind: S10OperationalScenarioKind,
) -> Vec<OperationalRecoveryActionKind> {
    use OperationalRecoveryActionKind as Action;
    let mut required = vec![Action::WorkflowOpened, Action::AuthorizationConsumed];
    if kind != S10OperationalScenarioKind::AuthorityRepairRollback {
        required.extend([
            Action::ReplicaBootstrapTransferRecorded,
            Action::ReplicaBootstrapCompleted,
            Action::ReplicaPromotionFenceRecorded,
            Action::ReplicaPromotionRecorded,
            Action::ReplicaPromotionPublished,
            Action::ReplicaPromotionReadmitted,
            Action::OldPrimaryRejoinPlanned,
            Action::OldPrimaryRejoinCompleted,
        ]);
    }
    required
}

struct S10ScenarioIdentityInputs<'a> {
    program: S10OperationalScenarioProgram,
    scale: ScenarioScaleEvidence,
    physical: &'a PhysicalCertificationEvidenceBundle,
    trace: &'a OperationalRecoveryDriverTrace,
    refinement: &'a OperationalRecoveryRefinementReceipt,
    mutation_sensitivity: &'a OperationalRecoveryMutationSensitivitySuite,
    qos: &'a S10OperationalQosEvidence,
    audit: &'a AuditCompletenessReceipt,
    counters: &'a [OperationalCounterReceipt],
}

fn evidence_identity(inputs: &S10ScenarioIdentityInputs<'_>) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-operational-scenario-v1");
    digest.update([inputs.program.kind() as u8, inputs.program.profile() as u8]);
    let dimensions = inputs.scale.dimensions();
    digest.update(dimensions.store_bytes().to_be_bytes());
    digest.update(dimensions.blob_bytes().to_be_bytes());
    digest.update(dimensions.wal_tail_bytes().to_be_bytes());
    digest.update(dimensions.damaged_region_bytes().to_be_bytes());
    digest.update(dimensions.artifact_count().to_be_bytes());
    digest.update(dimensions.candidate_count().to_be_bytes());
    digest.update(inputs.scale.resident_budget_bytes().to_be_bytes());
    digest.update(inputs.scale.schedules_executed().to_be_bytes());
    digest.update(inputs.physical.primary().transcript_digest());
    for point in inputs.trace.reached() {
        digest.update(point.token().as_bytes());
    }
    for operation in inputs.trace.operation_identities() {
        digest.update((operation.len() as u64).to_be_bytes());
        digest.update(operation.as_bytes());
    }
    digest.update(inputs.refinement.refinement_identity());
    digest.update(inputs.mutation_sensitivity.suite_identity());
    digest.update(inputs.qos.evidence_identity());
    digest.update(inputs.audit.terminal_record_identity());
    for counter in inputs.counters {
        digest.update(counter.session().fingerprint());
        digest.update([counter.kind() as u8]);
        digest.update(counter.source_bytes_read().to_be_bytes());
        digest.update(counter.output_bytes_written().to_be_bytes());
        digest.update(counter.durable_protocol_transitions().to_be_bytes());
        digest.update(counter.external_fence_grants().to_be_bytes());
        digest.update(counter.retained_source_leases().to_be_bytes());
        digest.update(counter.work_units().to_be_bytes());
        digest.update(counter.maximum_resident_bytes().to_be_bytes());
        digest.update(counter.authorization_consumptions().to_be_bytes());
        digest.update(counter.owner_receipts().to_be_bytes());
        digest.update(counter.forbidden_full_materializations().to_be_bytes());
        digest.update(counter.foreign_work_units().to_be_bytes());
    }
    digest.finalize().into()
}
