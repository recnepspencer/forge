use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::{OperationalControlRecord, OperationalControlRecordKind};

use super::{
    AuditCausalParent, OperationLocalSequence, OperationalAuditRecord,
    OperationalAuditTransitionKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalAuditDerivationDenial {
    SequenceOverflow,
    ConflictingDuplicateTransition,
}

pub fn derive_operational_audit_records(
    durable_records: &[OperationalControlRecord],
) -> Result<Vec<OperationalAuditRecord>, OperationalAuditDerivationDenial> {
    let mut operations = BTreeMap::<String, Vec<&OperationalControlRecord>>::new();
    for record in durable_records {
        operations
            .entry(record.operation_id().as_str().to_owned())
            .or_default()
            .push(record);
    }
    let mut audit_records = Vec::new();
    for records in operations.values() {
        let mut observed_transitions = BTreeMap::<String, [u8; 32]>::new();
        let mut causal_parent = None;
        let mut sequence_value = 0_u64;
        for record in records.iter() {
            let source_artifact_identity = control_record_fingerprint(record);
            if let Some(prior_identity) =
                observed_transitions.get(record.transition_id().as_str())
            {
                if *prior_identity != source_artifact_identity {
                    return Err(
                        OperationalAuditDerivationDenial::ConflictingDuplicateTransition,
                    );
                }
                continue;
            }
            sequence_value = sequence_value
                .checked_add(1)
                .ok_or(OperationalAuditDerivationDenial::SequenceOverflow)?;
            let sequence = OperationLocalSequence::new(sequence_value)
                .ok_or(OperationalAuditDerivationDenial::SequenceOverflow)?;
            let transition_kind = transition_kind(record.kind());
            let record_identity = audit_record_identity(
                record.operation_id().as_str(),
                record.transition_id().as_str(),
                sequence,
                causal_parent,
                transition_kind,
                source_artifact_identity,
            );
            let audit = OperationalAuditRecord {
                operation_id: record.operation_id().clone(),
                transition_id: record.transition_id().clone(),
                sequence,
                causal_parent,
                transition_kind,
                source_artifact_identity,
                record_identity,
            };
            causal_parent = Some(AuditCausalParent::from_record(&audit));
            observed_transitions.insert(
                record.transition_id().as_str().to_owned(),
                source_artifact_identity,
            );
            audit_records.push(audit);
        }
    }
    Ok(audit_records)
}

fn transition_kind(kind: &OperationalControlRecordKind) -> OperationalAuditTransitionKind {
    match kind {
        OperationalControlRecordKind::WorkflowOpened { .. } => {
            OperationalAuditTransitionKind::WorkflowOpened
        }
        OperationalControlRecordKind::SourceLeasePersisted { .. } => {
            OperationalAuditTransitionKind::SourceLeasePersisted
        }
        OperationalControlRecordKind::BackupMaterializationOpened { .. } => {
            OperationalAuditTransitionKind::MaterializationOpened
        }
        OperationalControlRecordKind::BackupMaterializationRecorded { .. } => {
            OperationalAuditTransitionKind::MaterializationRecorded
        }
        OperationalControlRecordKind::IndependentBackupVerificationRecordedAndSourceLeaseReleased { .. } => {
            OperationalAuditTransitionKind::IndependentVerificationRecorded
        }
        OperationalControlRecordKind::BackupAbandoned { .. } => {
            OperationalAuditTransitionKind::Abandoned
        }
        OperationalControlRecordKind::AuthorizationConsumed { .. } => {
            OperationalAuditTransitionKind::AuthorizationConsumed
        }
        OperationalControlRecordKind::RepairExecutionOpened { .. } => {
            OperationalAuditTransitionKind::OwnerExecutionOpened
        }
        OperationalControlRecordKind::RepairOwnerEffectStarted { .. } => {
            OperationalAuditTransitionKind::OwnerEffectStarted
        }
        OperationalControlRecordKind::RepairOwnerReceiptPersisted { .. } => {
            OperationalAuditTransitionKind::OwnerReceiptPersisted
        }
        OperationalControlRecordKind::OperationalOwnerReceiptPersisted { .. } => {
            OperationalAuditTransitionKind::OwnerReceiptPersisted
        }
        OperationalControlRecordKind::RepairDispositionRecorded { .. } => {
            OperationalAuditTransitionKind::DispositionRecorded
        }
        OperationalControlRecordKind::RecoveryStagingCompleted { .. } => {
            OperationalAuditTransitionKind::StagingCompleted
        }
        OperationalControlRecordKind::RecoveryPublicationPrepared { .. } => {
            OperationalAuditTransitionKind::PublicationPrepared
        }
        OperationalControlRecordKind::RecoveryPublicationPending { .. } => {
            OperationalAuditTransitionKind::PublicationPending
        }
        OperationalControlRecordKind::RecoveryPublicationDisposition { .. } => {
            OperationalAuditTransitionKind::PublicationDisposition
        }
        OperationalControlRecordKind::RecoveryPublicationFenceReleased { .. } => {
            OperationalAuditTransitionKind::FenceReleased
        }
    }
}

fn control_record_fingerprint(record: &OperationalControlRecord) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-control-artifact-v1");
    digest.update(record.authority_identity().fingerprint());
    digest.update(record.operation_id().as_str().as_bytes());
    digest.update(record.transition_id().as_str().as_bytes());
    fingerprint_kind(record.kind(), &mut digest);
    digest.finalize().into()
}

fn fingerprint_kind(kind: &OperationalControlRecordKind, digest: &mut Sha256) {
    digest.update([transition_kind(kind) as u8]);
    match kind {
        OperationalControlRecordKind::WorkflowOpened { workflow } => digest.update([*workflow as u8]),
        OperationalControlRecordKind::SourceLeasePersisted { recovery, recovery_object } => {
            digest.update(recovery.recovery_bytes());
            digest.update(recovery_object.digest());
            digest.update(recovery_object.bytes().to_be_bytes());
        }
        OperationalControlRecordKind::BackupMaterializationOpened { plan } => {
            digest.update(plan.cut_identity());
            digest.update((plan.buffer_bytes() as u64).to_be_bytes());
            digest.update(plan.target_parent().as_os_str().to_string_lossy().as_bytes());
        }
        OperationalControlRecordKind::BackupMaterializationRecorded { manifest_digest } => digest.update(manifest_digest),
        OperationalControlRecordKind::IndependentBackupVerificationRecordedAndSourceLeaseReleased { verification_identity, release } => {
            digest.update(verification_identity);
            digest.update(release.recovery_bytes());
        }
        OperationalControlRecordKind::BackupAbandoned { reason, released_source_lease } => {
            digest.update(reason.as_bytes());
            digest.update(released_source_lease.recovery_bytes());
        }
        OperationalControlRecordKind::AuthorizationConsumed { authorization_identity, plan_fingerprint, operation_tag, execution_plan_fingerprint, assertion_identity, expires_at, replay_same_operation_identity } => {
            digest.update(authorization_identity);
            digest.update(plan_fingerprint);
            digest.update([*operation_tag]);
            digest.update(execution_plan_fingerprint.unwrap_or([0; 32]));
            digest.update(assertion_identity);
            digest.update(expires_at.to_be_bytes());
            digest.update([u8::from(*replay_same_operation_identity)]);
        }
        OperationalControlRecordKind::RepairExecutionOpened { authorization_identity, plan_fingerprint, owner_node_count, topology_tag } => {
            digest.update(authorization_identity); digest.update(plan_fingerprint); digest.update(owner_node_count.to_be_bytes()); digest.update([*topology_tag]);
        }
        OperationalControlRecordKind::RepairOwnerReceiptPersisted { plan_fingerprint, node_fingerprint, receipt_fingerprint, owner_tag } => {
            digest.update(plan_fingerprint); digest.update(node_fingerprint); digest.update(receipt_fingerprint); digest.update([*owner_tag]);
        }
        OperationalControlRecordKind::RepairOwnerEffectStarted { plan_fingerprint, node_fingerprint, owner_tag } => {
            digest.update(plan_fingerprint); digest.update(node_fingerprint); digest.update([*owner_tag]);
        }
        OperationalControlRecordKind::OperationalOwnerReceiptPersisted { workflow, plan_fingerprint, receipt_fingerprint, owner_tag } => {
            digest.update([*workflow as u8]); digest.update(plan_fingerprint);
            digest.update(receipt_fingerprint); digest.update([*owner_tag]);
        }
        OperationalControlRecordKind::RepairDispositionRecorded { plan_fingerprint, disposition_tag, disposition_basis } => {
            digest.update(plan_fingerprint); digest.update([*disposition_tag]); digest.update(disposition_basis);
        }
        OperationalControlRecordKind::RecoveryStagingCompleted { authorization_identity, plan_fingerprint, execution_plan_fingerprint, staged_media_identity } => {
            digest.update(authorization_identity); digest.update(plan_fingerprint); digest.update(execution_plan_fingerprint); digest.update(staged_media_identity);
        }
        OperationalControlRecordKind::RecoveryPublicationPrepared { binding }
        | OperationalControlRecordKind::RecoveryPublicationPending { binding } => fingerprint_publication_binding(binding, digest),
        OperationalControlRecordKind::RecoveryPublicationDisposition { publication_identity, disposition_tag, disposition_basis, observed_authority } => {
            digest.update(publication_identity); digest.update([*disposition_tag]); digest.update(disposition_basis); digest.update(observed_authority.fingerprint());
        }
        OperationalControlRecordKind::RecoveryPublicationFenceReleased { publication_identity, fence_identity, fence_plan_fingerprint, disposition_tag } => {
            digest.update(publication_identity); digest.update(fence_identity); digest.update(fence_plan_fingerprint); digest.update([*disposition_tag]);
        }
    }
}

fn fingerprint_publication_binding(
    binding: &crate::RecoveryPublicationControlBinding,
    digest: &mut Sha256,
) {
    digest.update([binding.operation_tag()]);
    digest.update(binding.cutover_plan_fingerprint());
    digest.update(binding.publication_plan_fingerprint());
    digest.update(binding.publication_identity());
    digest.update(binding.candidate_media_identity());
    digest.update(binding.fence_identity());
    digest.update(binding.fence_plan_fingerprint());
}

fn audit_record_identity(
    operation: &str,
    transition: &str,
    sequence: OperationLocalSequence,
    parent: Option<AuditCausalParent>,
    kind: OperationalAuditTransitionKind,
    source: [u8; 32],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-audit-record-v1");
    digest.update(operation.as_bytes());
    digest.update(transition.as_bytes());
    digest.update(sequence.get().to_be_bytes());
    digest.update(parent.map(AuditCausalParent::record_identity).unwrap_or([0; 32]));
    digest.update([kind as u8]);
    digest.update(source);
    digest.finalize().into()
}
