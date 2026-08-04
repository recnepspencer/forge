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
            let source_artifact_identity = record.stable_fingerprint();
            if let Some(prior_identity) = observed_transitions.get(record.transition_id().as_str())
            {
                if *prior_identity != source_artifact_identity {
                    return Err(OperationalAuditDerivationDenial::ConflictingDuplicateTransition);
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
        OperationalControlRecordKind::ReplicaBootstrapTransferRecorded { .. } => {
            OperationalAuditTransitionKind::ReplicaBootstrapTransferRecorded
        }
        OperationalControlRecordKind::ReplicaBootstrapCompleted { .. } => {
            OperationalAuditTransitionKind::ReplicaBootstrapCompleted
        }
        OperationalControlRecordKind::ReplicaBootstrapAbandoned { .. } => {
            OperationalAuditTransitionKind::Abandoned
        }
        OperationalControlRecordKind::ReplicaPromotionFenceRecorded { .. } => {
            OperationalAuditTransitionKind::ReplicaPromotionFenceRecorded
        }
        OperationalControlRecordKind::ReplicaPromotionRecorded { .. } => {
            OperationalAuditTransitionKind::ReplicaPromotionRecorded
        }
        OperationalControlRecordKind::ReplicaPromotionPublished { .. } => {
            OperationalAuditTransitionKind::ReplicaPromotionPublished
        }
        OperationalControlRecordKind::ReplicaPromotionReadmitted { .. } => {
            OperationalAuditTransitionKind::ReplicaPromotionReadmitted
        }
        OperationalControlRecordKind::OldPrimaryRejoinPlanned { .. } => {
            OperationalAuditTransitionKind::OldPrimaryRejoinPlanned
        }
        OperationalControlRecordKind::OldPrimaryRejoinCompleted { .. } => {
            OperationalAuditTransitionKind::OldPrimaryRejoinCompleted
        }
        OperationalControlRecordKind::RepairDispositionRecorded { .. } => {
            OperationalAuditTransitionKind::DispositionRecorded
        }
        OperationalControlRecordKind::RecoveryStagingCompleted { .. } => {
            OperationalAuditTransitionKind::StagingCompleted
        }
    }
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
    digest.update(
        parent
            .map(AuditCausalParent::record_identity)
            .unwrap_or([0; 32]),
    );
    digest.update([kind as u8]);
    digest.update(source);
    digest.finalize().into()
}
