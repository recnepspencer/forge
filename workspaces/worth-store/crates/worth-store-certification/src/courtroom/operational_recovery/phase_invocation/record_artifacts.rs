use sha2::{Digest, Sha256};
use worth_store_operations::{
    OperationalControlRecord, OperationalControlRecordKind, OperationalWorkflowKind,
};

use super::{S10OperationalScenarioKind, S10PhaseInvocationDenial};

pub(super) struct PhaseRecordArtifacts {
    identity: [u8; 32],
    localization_members: Vec<[u8; 32]>,
}

impl PhaseRecordArtifacts {
    pub(super) const fn identity(&self) -> [u8; 32] {
        self.identity
    }

    pub(super) fn into_localization_members(self) -> Vec<[u8; 32]> {
        self.localization_members
    }
}

pub(super) fn backup_cut_identity(
    records: &[OperationalControlRecord],
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    record_set_identity(records, b"s10-phase-5", |record| {
        matches!(
            record.kind(),
            OperationalControlRecordKind::SourceLeasePersisted { .. }
        )
    })
    .ok_or(S10PhaseInvocationDenial::MissingBackupCut)
}

pub(super) fn backup_bundle_identity(
    records: &[OperationalControlRecord],
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    let operation = operation_with(records, |operation_records| {
        has(operation_records, materialization_opened)
            && has(operation_records, materialization_recorded)
            && has(operation_records, independent_verification)
    })
    .ok_or(S10PhaseInvocationDenial::MissingBackupBundleLifecycle)?;
    Ok(operation_record_identity(
        records,
        &operation,
        b"s10-phase-6",
        |record| {
            materialization_opened(record)
                || materialization_recorded(record)
                || independent_verification(record)
        },
    ))
}

pub(super) fn authorization_identity(
    records: &[OperationalControlRecord],
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    record_set_identity(records, b"s10-phase-7", authorization)
        .ok_or(S10PhaseInvocationDenial::MissingAuthorization)
}

pub(super) fn staged_workflow_identity(
    records: &[OperationalControlRecord],
    operation_tag: u8,
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    let operation = operation_with(records, |operation_records| {
        operation_records
            .iter()
            .any(|record| authorization_with_tag(record, operation_tag))
            && owner_receipts_complete(operation_records, operation_tag)
            && has(operation_records, staging_completed)
    })
    .ok_or(S10PhaseInvocationDenial::MissingStagedWorkflow(
        operation_tag,
    ))?;
    let mut artifacts =
        operation_record_identity(records, &operation, b"s10-staged-workflow", |record| {
            authorization_with_tag(record, operation_tag)
                || owner_receipt_for_workflow(record, operation_tag)
                || staging_completed(record)
        });
    artifacts.localization_members = matching_record_fingerprints(records, |record| {
        record.operation_id().as_str() == operation
            && (owner_receipt_for_workflow(record, operation_tag) || staging_completed(record))
    });
    Ok(artifacts)
}

pub(super) fn repair_plan_identity(
    records: &[OperationalControlRecord],
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    let operation = operation_with(records, |operation_records| {
        operation_records
            .iter()
            .any(|record| authorization_with_tag(record, 5))
            && has(operation_records, repair_opened)
    })
    .ok_or(S10PhaseInvocationDenial::MissingRepairPlan)?;
    let mut artifacts = operation_record_identity(records, &operation, b"s10-phase-11", |record| {
        authorization_with_tag(record, 5) || repair_opened(record)
    });
    artifacts.localization_members = matching_record_fingerprints(records, |record| {
        record.operation_id().as_str() == operation && repair_opened(record)
    });
    Ok(artifacts)
}

pub(super) fn repair_execution_identity(
    records: &[OperationalControlRecord],
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    let operation = operation_with(records, |operation_records| {
        has(operation_records, repair_effect_started)
            && has(operation_records, repair_receipt)
            && has(operation_records, repair_disposition)
    })
    .ok_or(S10PhaseInvocationDenial::MissingRepairExecution)?;
    Ok(operation_record_identity(
        records,
        &operation,
        b"s10-phase-12",
        |record| {
            repair_effect_started(record) || repair_receipt(record) || repair_disposition(record)
        },
    ))
}

pub(super) fn publication_identity(
    records: &[OperationalControlRecord],
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    let operation = operation_with(records, |operation_records| {
        has(operation_records, publication_prepared)
            && has(operation_records, publication_pending)
            && has(operation_records, publication_disposition)
            && has(operation_records, fence_released)
    })
    .ok_or(S10PhaseInvocationDenial::MissingPublicationLifecycle)?;
    Ok(operation_record_identity(
        records,
        &operation,
        b"s10-phase-13",
        |record| {
            publication_prepared(record)
                || publication_pending(record)
                || publication_disposition(record)
                || fence_released(record)
        },
    ))
}

pub(super) fn replica_identity(
    kind: S10OperationalScenarioKind,
    records: &[OperationalControlRecord],
) -> Result<PhaseRecordArtifacts, S10PhaseInvocationDenial> {
    let bootstrap = operation_with(records, |operation_records| {
        has(operation_records, bootstrap_transferred) && has(operation_records, bootstrap_completed)
    });
    let promotion = operation_with(records, |operation_records| {
        has(operation_records, promotion_fenced)
            && has(operation_records, promotion_recorded)
            && has(operation_records, promotion_published)
            && has(operation_records, promotion_readmitted)
            && (kind != S10OperationalScenarioKind::SplitBrainPromotion
                || (has(operation_records, rejoin_planned)
                    && has(operation_records, rejoin_completed)))
    });
    let (Some(bootstrap), Some(promotion)) = (bootstrap, promotion) else {
        return Err(S10PhaseInvocationDenial::MissingReplicaLifecycle);
    };
    record_set_identity(records, b"s10-phase-14", |record| {
        let operation = record.operation_id().as_str();
        (operation == bootstrap && (bootstrap_transferred(record) || bootstrap_completed(record)))
            || (operation == promotion
                && (promotion_fenced(record)
                    || promotion_recorded(record)
                    || promotion_published(record)
                    || promotion_readmitted(record)
                    || rejoin_planned(record)
                    || rejoin_completed(record)))
    })
    .ok_or(S10PhaseInvocationDenial::MissingReplicaLifecycle)
}

fn operation_with(
    records: &[OperationalControlRecord],
    accepts: impl Fn(&[&OperationalControlRecord]) -> bool,
) -> Option<String> {
    let mut operations = records
        .iter()
        .map(|record| record.operation_id().as_str())
        .collect::<Vec<_>>();
    operations.sort_unstable();
    operations.dedup();
    operations.into_iter().find_map(|operation| {
        let operation_records = records
            .iter()
            .filter(|record| record.operation_id().as_str() == operation)
            .collect::<Vec<_>>();
        accepts(&operation_records).then(|| operation.to_owned())
    })
}

fn has(
    records: &[&OperationalControlRecord],
    predicate: impl Fn(&OperationalControlRecord) -> bool,
) -> bool {
    records.iter().any(|record| predicate(record))
}

fn operation_record_identity(
    records: &[OperationalControlRecord],
    operation: &str,
    domain: &[u8],
    include: impl Fn(&OperationalControlRecord) -> bool,
) -> PhaseRecordArtifacts {
    record_set_identity(records, domain, |record| {
        record.operation_id().as_str() == operation && include(record)
    })
    .expect("the selected operation has required records")
}

fn record_set_identity(
    records: &[OperationalControlRecord],
    domain: &[u8],
    include: impl Fn(&OperationalControlRecord) -> bool,
) -> Option<PhaseRecordArtifacts> {
    let mut identities = matching_record_fingerprints(records, include);
    if identities.is_empty() {
        return None;
    }
    identities.sort_unstable();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-phase-production-artifacts-v2");
    digest.update((domain.len() as u64).to_be_bytes());
    digest.update(domain);
    digest.update((identities.len() as u64).to_be_bytes());
    for identity in &identities {
        digest.update(identity);
    }
    Some(PhaseRecordArtifacts {
        identity: digest.finalize().into(),
        localization_members: identities,
    })
}

fn matching_record_fingerprints(
    records: &[OperationalControlRecord],
    include: impl Fn(&OperationalControlRecord) -> bool,
) -> Vec<[u8; 32]> {
    let mut identities = records
        .iter()
        .filter(|record| include(record))
        .map(OperationalControlRecord::stable_fingerprint)
        .collect::<Vec<_>>();
    identities.sort_unstable();
    identities
}

fn authorization(record: &OperationalControlRecord) -> bool {
    matches!(
        record.kind(),
        OperationalControlRecordKind::AuthorizationConsumed { .. }
    )
}

fn authorization_with_tag(record: &OperationalControlRecord, expected: u8) -> bool {
    matches!(
        record.kind(),
        OperationalControlRecordKind::AuthorizationConsumed { operation_tag, .. }
            if *operation_tag == expected
    )
}

fn owner_receipts_complete(records: &[&OperationalControlRecord], operation_tag: u8) -> bool {
    [1_u8, 2_u8].into_iter().all(|expected_owner| {
        records.iter().any(|record| {
            owner_receipt_for_workflow(record, operation_tag)
                && matches!(
                    record.kind(),
                    OperationalControlRecordKind::OperationalOwnerReceiptPersisted {
                        owner_tag,
                        ..
                    } if *owner_tag == expected_owner
                )
        })
    })
}

fn owner_receipt_for_workflow(record: &OperationalControlRecord, operation_tag: u8) -> bool {
    let expected = match operation_tag {
        1 => OperationalWorkflowKind::Restore,
        2 => OperationalWorkflowKind::PointInTimeRecovery,
        3 => OperationalWorkflowKind::Rollback,
        _ => return false,
    };
    matches!(
        record.kind(),
        OperationalControlRecordKind::OperationalOwnerReceiptPersisted { workflow, .. }
            if *workflow == expected
    )
}

macro_rules! kind_predicate {
    ($name:ident, $pattern:pat) => {
        fn $name(record: &OperationalControlRecord) -> bool {
            matches!(record.kind(), $pattern)
        }
    };
}

kind_predicate!(
    materialization_opened,
    OperationalControlRecordKind::BackupMaterializationOpened { .. }
);
kind_predicate!(
    materialization_recorded,
    OperationalControlRecordKind::BackupMaterializationRecorded { .. }
);
kind_predicate!(
    independent_verification,
    OperationalControlRecordKind::IndependentBackupVerificationRecordedAndSourceLeaseReleased { .. }
);
kind_predicate!(
    staging_completed,
    OperationalControlRecordKind::RecoveryStagingCompleted { .. }
);
kind_predicate!(
    repair_opened,
    OperationalControlRecordKind::RepairExecutionOpened { .. }
);
kind_predicate!(
    repair_effect_started,
    OperationalControlRecordKind::RepairOwnerEffectStarted { .. }
);
kind_predicate!(
    repair_receipt,
    OperationalControlRecordKind::RepairOwnerReceiptPersisted { .. }
);
kind_predicate!(
    repair_disposition,
    OperationalControlRecordKind::RepairDispositionRecorded { .. }
);
kind_predicate!(
    publication_prepared,
    OperationalControlRecordKind::RecoveryPublicationPrepared { .. }
);
kind_predicate!(
    publication_pending,
    OperationalControlRecordKind::RecoveryPublicationPending { .. }
);
kind_predicate!(
    publication_disposition,
    OperationalControlRecordKind::RecoveryPublicationDisposition { .. }
);
kind_predicate!(
    fence_released,
    OperationalControlRecordKind::RecoveryPublicationFenceReleased { .. }
);
kind_predicate!(
    bootstrap_transferred,
    OperationalControlRecordKind::ReplicaBootstrapTransferRecorded { .. }
);
kind_predicate!(
    bootstrap_completed,
    OperationalControlRecordKind::ReplicaBootstrapCompleted { .. }
);
kind_predicate!(
    promotion_fenced,
    OperationalControlRecordKind::ReplicaPromotionFenceRecorded { .. }
);
kind_predicate!(
    promotion_recorded,
    OperationalControlRecordKind::ReplicaPromotionRecorded { .. }
);
kind_predicate!(
    promotion_published,
    OperationalControlRecordKind::ReplicaPromotionPublished { .. }
);
kind_predicate!(
    promotion_readmitted,
    OperationalControlRecordKind::ReplicaPromotionReadmitted { .. }
);
kind_predicate!(
    rejoin_planned,
    OperationalControlRecordKind::OldPrimaryRejoinPlanned { .. }
);
kind_predicate!(
    rejoin_completed,
    OperationalControlRecordKind::OldPrimaryRejoinCompleted { .. }
);
