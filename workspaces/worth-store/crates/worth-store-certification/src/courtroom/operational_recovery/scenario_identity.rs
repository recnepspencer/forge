use super::{
    S10HostileProgramEvidence, S10OperationalQosEvidence, S10OperationalScenarioProgram,
    S10PhaseInvocationEvidence, S10ScenarioExecutionMatrix, ScenarioScaleEvidence,
};
use sha2::{Digest, Sha256};
use worth_store_formal_models::{
    OperationalRecoveryMutationSensitivitySuite, OperationalRecoveryRefinementReceipt,
};
use worth_store_operations::{
    AuditCompletenessReceipt, OperationalCounterReceipt, OperationalSessionKind,
};

pub(super) struct S10ScenarioIdentityInputs<'a> {
    pub(super) program: S10OperationalScenarioProgram,
    pub(super) hostile_program: S10HostileProgramEvidence,
    pub(super) scale: ScenarioScaleEvidence,
    pub(super) execution: &'a S10ScenarioExecutionMatrix,
    pub(super) refinement: &'a OperationalRecoveryRefinementReceipt,
    pub(super) mutation_sensitivity: &'a OperationalRecoveryMutationSensitivitySuite,
    pub(super) qos: &'a S10OperationalQosEvidence,
    pub(super) audits: &'a [AuditCompletenessReceipt],
    pub(super) counters: &'a [OperationalCounterReceipt],
    pub(super) phase_invocations: &'a [S10PhaseInvocationEvidence],
}

pub(super) fn evidence_identity(inputs: &S10ScenarioIdentityInputs<'_>) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-operational-scenario-v2");
    digest.update(inputs.program.kind().token().as_bytes());
    digest.update(inputs.program.profile().token().as_bytes());
    digest.update(inputs.hostile_program.evidence_identity());
    let dimensions = inputs.scale.dimensions();
    digest.update(dimensions.store_bytes().to_be_bytes());
    digest.update(dimensions.blob_bytes().to_be_bytes());
    digest.update(dimensions.wal_tail_bytes().to_be_bytes());
    digest.update(dimensions.damaged_region_bytes().to_be_bytes());
    digest.update(dimensions.artifact_count().to_be_bytes());
    digest.update(dimensions.candidate_count().to_be_bytes());
    digest.update(inputs.scale.resident_budget_bytes().to_be_bytes());
    digest.update(inputs.scale.schedules_executed().to_be_bytes());
    digest.update(inputs.execution.matrix_identity());
    digest.update(inputs.refinement.refinement_identity());
    digest.update(inputs.mutation_sensitivity.suite_identity());
    digest.update(inputs.qos.evidence_identity());
    digest.update(audit_set_identity(inputs.audits));
    for counter in inputs.counters {
        digest.update(counter.session().fingerprint());
        digest.update([session_kind_tag(counter.kind())]);
        digest.update([session_disposition_tag(counter.disposition())]);
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
    for invocation in inputs.phase_invocations {
        digest.update([invocation.phase().number()]);
        digest.update(invocation.production_artifact_identity());
    }
    digest.finalize().into()
}

const fn session_disposition_tag(
    disposition: worth_store_operations::OperationalSessionDisposition,
) -> u8 {
    match disposition {
        worth_store_operations::OperationalSessionDisposition::Completed => 1,
        worth_store_operations::OperationalSessionDisposition::Abandoned => 2,
    }
}

pub(super) fn audit_set_identity(audits: &[AuditCompletenessReceipt]) -> [u8; 32] {
    let mut identities = audits
        .iter()
        .map(|audit| audit.terminal_record_identity())
        .collect::<Vec<_>>();
    identities.sort_unstable();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-complete-audit-set-v1");
    digest.update((identities.len() as u64).to_be_bytes());
    for identity in identities {
        digest.update(identity);
    }
    digest.finalize().into()
}

pub(super) fn phase_16_identity(execution: &S10ScenarioExecutionMatrix) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-phase-16-invocation-v1");
    digest.update(execution.matrix_identity());
    digest.finalize().into()
}

pub(super) fn phase_17_identity(
    refinement: &OperationalRecoveryRefinementReceipt,
    mutation: &OperationalRecoveryMutationSensitivitySuite,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-phase-17-invocation-v1");
    digest.update(refinement.refinement_identity());
    digest.update(mutation.suite_identity());
    digest.finalize().into()
}

pub(super) fn phase_18_identity(
    scale: ScenarioScaleEvidence,
    qos: &S10OperationalQosEvidence,
    counters: &[OperationalCounterReceipt],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-phase-18-invocation-v1");
    digest.update(scale.store_bytes().to_be_bytes());
    digest.update(scale.resident_budget_bytes().to_be_bytes());
    digest.update(scale.schedules_executed().to_be_bytes());
    digest.update(qos.evidence_identity());
    for counter in counters {
        digest.update(counter.session().fingerprint());
        digest.update(counter.work_units().to_be_bytes());
        digest.update(counter.maximum_resident_bytes().to_be_bytes());
    }
    digest.finalize().into()
}

pub(super) fn phase_19_identity(
    program: S10OperationalScenarioProgram,
    audit: [u8; 32],
    driver: [u8; 32],
    model: [u8; 32],
    performance: [u8; 32],
) -> [u8; 32] {
    phase_19_join_identity(
        program,
        [Some(audit), Some(driver), Some(model), Some(performance)],
    )
    .expect("scenario certification supplies every Phase 19 join component")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MissingPhase19JoinComponent(pub(super) usize);

pub(super) fn phase_19_join_identity(
    program: S10OperationalScenarioProgram,
    components: [Option<[u8; 32]>; 4],
) -> Result<[u8; 32], MissingPhase19JoinComponent> {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-phase-19-scenario-join-v1");
    digest.update(program.kind().token().as_bytes());
    digest.update(program.profile().token().as_bytes());
    for (index, component) in components.into_iter().enumerate() {
        let component = component.ok_or(MissingPhase19JoinComponent(index))?;
        digest.update(component);
    }
    Ok(digest.finalize().into())
}

const fn session_kind_tag(kind: OperationalSessionKind) -> u8 {
    match kind {
        OperationalSessionKind::Backup => 1,
        OperationalSessionKind::Restore => 2,
        OperationalSessionKind::PointInTimeRecovery => 3,
        OperationalSessionKind::Rollback => 4,
        OperationalSessionKind::Repair => 5,
        OperationalSessionKind::ReplicaBootstrap => 6,
        OperationalSessionKind::ReplicaPromotion => 7,
        OperationalSessionKind::ForensicAcquisition => 8,
        OperationalSessionKind::OfflineVerification => 9,
    }
}
