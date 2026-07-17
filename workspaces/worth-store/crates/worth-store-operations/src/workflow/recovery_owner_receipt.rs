use sha2::{Digest, Sha256};
use worth_store_physical_backend::NonCurrentStagingExecutionReceipt;
use worth_store_recovery_physics::{
    PointInTimeRecoveryReceipt, RecoveredBackupFrontierReceipt, RollbackExecutionReceipt,
};

use crate::{
    AuthorizationConsumptionReceipt, OperationalControlAppendDenial, OperationalControlRecord,
    OperationalControlStorePort, OperationalOperationId, OperationalTransitionId,
    OperationalWorkflowKind,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn persist_recovery_owner_receipts(
    control: &(impl OperationalControlStorePort + ?Sized),
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_id: &OperationalOperationId,
    authorization: AuthorizationConsumptionReceipt,
    workflow: OperationalWorkflowKind,
    backend: &NonCurrentStagingExecutionReceipt,
    recovery_receipt_identity: [u8; 32],
) -> Result<(), OperationalControlAppendDenial> {
    persist(
        control,
        authority_identity,
        operation_id,
        OperationalTransitionId::backend_owner_receipt(),
        workflow,
        authorization.plan_fingerprint(),
        backend_owner_receipt_identity(backend),
        1,
    )?;
    persist(
        control,
        authority_identity,
        operation_id,
        OperationalTransitionId::recovery_owner_receipt(),
        workflow,
        authorization.plan_fingerprint(),
        recovery_receipt_identity,
        2,
    )
}

#[allow(clippy::too_many_arguments)]
fn persist(
    control: &(impl OperationalControlStorePort + ?Sized),
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_id: &OperationalOperationId,
    transition_id: OperationalTransitionId,
    workflow: OperationalWorkflowKind,
    plan_fingerprint: [u8; 32],
    receipt_fingerprint: [u8; 32],
    owner_tag: u8,
) -> Result<(), OperationalControlAppendDenial> {
    control.append(
        &OperationalControlRecord::operational_owner_receipt_persisted(
            authority_identity,
            operation_id.clone(),
            transition_id,
            workflow,
            plan_fingerprint,
            receipt_fingerprint,
            owner_tag,
        ),
    )?;
    Ok(())
}

pub(crate) fn backend_owner_receipt_identity(
    value: &NonCurrentStagingExecutionReceipt,
) -> [u8; 32] {
    fingerprint(b"worth-store-backend-recovery-owner-receipt-v1", |digest| {
        digest.update(value.plan_fingerprint());
        digest.update(value.bytes_copied().to_be_bytes());
        digest.update(value.artifacts_materialized().to_be_bytes());
        digest.update(value.media().content_fingerprint());
    })
}

pub(crate) fn restored_frontier_owner_receipt_identity(
    value: RecoveredBackupFrontierReceipt,
) -> [u8; 32] {
    let replay = value.replay_source();
    let application = value.application();
    fingerprint(b"worth-store-recovery-owner-receipt-v1", |digest| {
        digest.update(value.plan_fingerprint());
        digest.update(value.durable_checkpoint_lsn().to_be_bytes());
        digest.update(value.wal_end_exclusive_lsn().to_be_bytes());
        digest.update(value.acknowledged_frontier().to_be_bytes());
        digest.update(value.root_generation().to_be_bytes());
        digest.update(replay.identity());
        digest.update(replay.manifest_digest());
        digest.update(replay.frame_count().to_be_bytes());
        digest.update(replay.bytes_verified().to_be_bytes());
        digest.update(replay.interval().0.to_be_bytes());
        digest.update(replay.interval().1.to_be_bytes());
        digest.update(application.identity());
        digest.update(application.application_identity());
        digest.update(application.replay_source_identity());
        digest.update(application.resulting_frontier_identity());
        digest.update(application.applied_frames().to_be_bytes());
    })
}

pub(crate) fn pitr_owner_receipt_identity(value: PointInTimeRecoveryReceipt) -> [u8; 32] {
    let frontier = value.exact_frontier();
    replay_owner_identity(
        b"worth-store-pitr-owner-receipt-v1",
        value.plan_fingerprint(),
        frontier,
        value.replay_source(),
        value.application(),
    )
}

pub(crate) fn rollback_owner_receipt_identity(value: RollbackExecutionReceipt) -> [u8; 32] {
    let frontier = value.frontier();
    replay_owner_identity(
        b"worth-store-rollback-owner-receipt-v1",
        value.plan_fingerprint(),
        frontier,
        value.replay_source(),
        value.application(),
    )
}

#[allow(clippy::too_many_arguments)]
fn replay_owner_identity(
    domain: &[u8],
    plan_fingerprint: [u8; 32],
    frontier: worth_store_recovery_physics::ExactRecoveryFrontier,
    replay: worth_store_recovery_physics::StagedWalReplaySourceReceipt,
    application: worth_store_recovery_physics::StagedWalApplicationReceipt,
) -> [u8; 32] {
    fingerprint(domain, |digest| {
        digest.update(plan_fingerprint);
        digest.update(frontier.identity());
        digest.update(frontier.checkpoint_durability().to_be_bytes());
        digest.update(frontier.wal_structural().to_be_bytes());
        digest.update(frontier.local_durable_commit().to_be_bytes());
        digest.update(frontier.client_acknowledged().to_be_bytes());
        digest.update(frontier.replication_acknowledged().to_be_bytes());
        digest.update(frontier.authority_identity().fingerprint());
        digest.update(frontier.source_lineage());
        digest.update(replay.identity());
        digest.update(replay.manifest_digest());
        digest.update(replay.frame_count().to_be_bytes());
        digest.update(replay.bytes_verified().to_be_bytes());
        digest.update(replay.interval().0.to_be_bytes());
        digest.update(replay.interval().1.to_be_bytes());
        digest.update(application.identity());
        digest.update(application.application_identity());
        digest.update(application.replay_source_identity());
        digest.update(application.resulting_frontier_identity());
        digest.update(application.applied_frames().to_be_bytes());
    })
}

fn fingerprint(domain: &[u8], update: impl FnOnce(&mut Sha256)) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    update(&mut digest);
    digest.finalize().into()
}
