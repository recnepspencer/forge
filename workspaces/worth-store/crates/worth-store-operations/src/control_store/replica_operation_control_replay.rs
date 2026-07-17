use std::collections::HashMap;

use super::RecoveredReplicaBootstrapDisposition;
use super::{
    OperationalControlHistoryViolationKind, OperationalOperationId,
    RecoveredReplicaBootstrapTransfer, RecoveredReplicaPromotionFence,
    RecoveredReplicaPromotionReceipt, ReplicaBootstrapRecoveryHandle,
    ReplicaPromotionRecoveryHandle,
};
use super::{RecoveredReplicaPromotionPublication, RecoveredReplicaPromotionReadmission};

pub(super) struct ReplayedReplicaBootstrap {
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    authorization_identity: [u8; 32],
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    transfer: Option<RecoveredReplicaBootstrapTransfer>,
    disposition: Option<RecoveredReplicaBootstrapDisposition>,
}

pub(super) struct ReplayedReplicaPromotion {
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    authorization_identity: [u8; 32],
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    fence: Option<RecoveredReplicaPromotionFence>,
    pub(super) receipt: Option<RecoveredReplicaPromotionReceipt>,
    publication: Option<RecoveredReplicaPromotionPublication>,
    pub(super) readmission: Option<RecoveredReplicaPromotionReadmission>,
    pub(super) rejoin_plan_fingerprint: Option<[u8; 32]>,
    pub(super) rejoin_disposition_tag: Option<u8>,
    pub(super) rejoin: Option<super::RecoveredOldPrimaryRejoin>,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_authorization(
    bootstraps: &mut HashMap<OperationalOperationId, ReplayedReplicaBootstrap>,
    promotions: &mut HashMap<OperationalOperationId, ReplayedReplicaPromotion>,
    operation: &OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_tag: u8,
    authorization_identity: [u8; 32],
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: Option<[u8; 32]>,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let Some(execution_plan_fingerprint) = execution_plan_fingerprint else {
        if matches!(operation_tag, 10 | 11) {
            return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
        }
        return Ok(());
    };
    match operation_tag {
        10 => {
            if bootstraps.contains_key(operation) {
                return Err(OperationalControlHistoryViolationKind::DuplicateReplicaOperation);
            }
            bootstraps
                .try_reserve(1)
                .map_err(|_| OperationalControlHistoryViolationKind::DuplicateReplicaOperation)?;
            bootstraps.insert(
                operation.clone(),
                ReplayedReplicaBootstrap {
                    authority_identity,
                    authorization_identity,
                    authorization_plan_fingerprint,
                    execution_plan_fingerprint,
                    transfer: None,
                    disposition: None,
                },
            );
        }
        11 => {
            if promotions.contains_key(operation) {
                return Err(OperationalControlHistoryViolationKind::DuplicateReplicaOperation);
            }
            promotions
                .try_reserve(1)
                .map_err(|_| OperationalControlHistoryViolationKind::DuplicateReplicaOperation)?;
            promotions.insert(
                operation.clone(),
                ReplayedReplicaPromotion {
                    authority_identity,
                    authorization_identity,
                    authorization_plan_fingerprint,
                    execution_plan_fingerprint,
                    fence: None,
                    receipt: None,
                    publication: None,
                    readmission: None,
                    rejoin_plan_fingerprint: None,
                    rejoin_disposition_tag: None,
                    rejoin: None,
                },
            );
        }
        _ => {}
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_bootstrap_transfer(
    bootstraps: &mut HashMap<OperationalOperationId, ReplayedReplicaBootstrap>,
    operation: &OperationalOperationId,
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    receipt_identity: [u8; 32],
    durable_target_identity: [u8; 32],
    source_lease_identity: [u8; 32],
    execution_counters: worth_store_replication::ReplicaBootstrapExecutionCounters,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = bootstraps
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    require_binding(
        state.authorization_plan_fingerprint,
        state.execution_plan_fingerprint,
        authorization_plan_fingerprint,
        execution_plan_fingerprint,
    )?;
    if state.transfer.is_some()
        || [
            receipt_identity,
            durable_target_identity,
            source_lease_identity,
        ]
        .contains(&[0; 32])
    {
        return Err(OperationalControlHistoryViolationKind::DuplicateReplicaOperationStage);
    }
    state.transfer = Some(RecoveredReplicaBootstrapTransfer::new(
        receipt_identity,
        durable_target_identity,
        source_lease_identity,
        execution_counters,
    ));
    Ok(())
}

pub(super) fn observe_promotion_fence(
    promotions: &mut HashMap<OperationalOperationId, ReplayedReplicaPromotion>,
    operation: &OperationalOperationId,
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    fence_identity: [u8; 32],
    promoted_epoch: u64,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = promotions
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    require_binding(
        state.authorization_plan_fingerprint,
        state.execution_plan_fingerprint,
        authorization_plan_fingerprint,
        execution_plan_fingerprint,
    )?;
    if state.fence.is_some() || fence_identity == [0; 32] || promoted_epoch == 0 {
        return Err(OperationalControlHistoryViolationKind::DuplicateReplicaOperationStage);
    }
    state.fence = Some(RecoveredReplicaPromotionFence::new(
        fence_identity,
        promoted_epoch,
    ));
    Ok(())
}

pub(super) fn observe_bootstrap_terminal(
    bootstraps: &mut HashMap<OperationalOperationId, ReplayedReplicaBootstrap>,
    operation: &OperationalOperationId,
    receipt_identity: [u8; 32],
    source_lease_identity: [u8; 32],
    disposition: RecoveredReplicaBootstrapDisposition,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = bootstraps
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    let transfer = state
        .transfer
        .ok_or(OperationalControlHistoryViolationKind::ReplicaBootstrapTerminalBeforeTransfer)?;
    if state.disposition.is_some() {
        return Err(OperationalControlHistoryViolationKind::ReplicaRecordAfterTerminal);
    }
    if transfer.receipt_identity() != receipt_identity
        || transfer.source_lease_identity() != source_lease_identity
    {
        return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
    }
    state.disposition = Some(disposition);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_promotion_receipt(
    promotions: &mut HashMap<OperationalOperationId, ReplayedReplicaPromotion>,
    operation: &OperationalOperationId,
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    receipt_identity: [u8; 32],
    fence_identity: [u8; 32],
    promoted_epoch: u64,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = promotions
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    require_binding(
        state.authorization_plan_fingerprint,
        state.execution_plan_fingerprint,
        authorization_plan_fingerprint,
        execution_plan_fingerprint,
    )?;
    let fence = state
        .fence
        .ok_or(OperationalControlHistoryViolationKind::ReplicaPromotionBeforeFence)?;
    if state.receipt.is_some()
        || receipt_identity == [0; 32]
        || fence.fence_identity() != fence_identity
        || fence.promoted_epoch() != promoted_epoch
    {
        return Err(OperationalControlHistoryViolationKind::DuplicateReplicaOperationStage);
    }
    state.receipt = Some(RecoveredReplicaPromotionReceipt::new(
        receipt_identity,
        fence_identity,
        promoted_epoch,
    ));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_promotion_publication(
    promotions: &mut HashMap<OperationalOperationId, ReplayedReplicaPromotion>,
    operation: &OperationalOperationId,
    receipt_identity: [u8; 32],
    verification_identity: [u8; 32],
    publication_identity: [u8; 32],
    target_identity: [u8; 32],
    promoted_epoch: u64,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = promotions
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    let receipt = state
        .receipt
        .ok_or(OperationalControlHistoryViolationKind::DuplicateReplicaOperationStage)?;
    if state.publication.is_some()
        || receipt.receipt_identity() != receipt_identity
        || receipt.promoted_epoch() != promoted_epoch
        || [verification_identity, publication_identity, target_identity].contains(&[0; 32])
    {
        return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
    }
    state.publication = Some(RecoveredReplicaPromotionPublication::new(
        publication_identity,
        verification_identity,
    ));
    Ok(())
}

pub(super) fn observe_promotion_readmission(
    promotions: &mut HashMap<OperationalOperationId, ReplayedReplicaPromotion>,
    operation: &OperationalOperationId,
    publication_identity: [u8; 32],
    serve_lease_identity: [u8; 32],
    serving_epoch: u64,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = promotions
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    let publication = state
        .publication
        .ok_or(OperationalControlHistoryViolationKind::DuplicateReplicaOperationStage)?;
    let promoted_epoch = state
        .receipt
        .ok_or(OperationalControlHistoryViolationKind::DuplicateReplicaOperationStage)?
        .promoted_epoch();
    if state.readmission.is_some()
        || publication.publication_identity() != publication_identity
        || serve_lease_identity == [0; 32]
        || serving_epoch < promoted_epoch
    {
        return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
    }
    state.readmission = Some(RecoveredReplicaPromotionReadmission::new(
        serve_lease_identity,
        serving_epoch,
    ));
    Ok(())
}

fn require_binding(
    expected_authorization: [u8; 32],
    expected_execution: [u8; 32],
    observed_authorization: [u8; 32],
    observed_execution: [u8; 32],
) -> Result<(), OperationalControlHistoryViolationKind> {
    if expected_authorization != observed_authorization || expected_execution != observed_execution
    {
        return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
    }
    Ok(())
}

impl ReplayedReplicaBootstrap {
    pub(super) fn recovery_handle(
        self,
        operation_id: OperationalOperationId,
    ) -> ReplicaBootstrapRecoveryHandle {
        ReplicaBootstrapRecoveryHandle::new(
            operation_id,
            self.authority_identity,
            self.authorization_identity,
            self.authorization_plan_fingerprint,
            self.execution_plan_fingerprint,
            self.transfer,
            self.disposition,
        )
    }
}

impl ReplayedReplicaPromotion {
    pub(super) fn recovery_handle(
        self,
        operation_id: OperationalOperationId,
    ) -> ReplicaPromotionRecoveryHandle {
        ReplicaPromotionRecoveryHandle::new(
            operation_id,
            self.authority_identity,
            self.authorization_identity,
            self.authorization_plan_fingerprint,
            self.execution_plan_fingerprint,
            self.fence,
            self.receipt,
            self.publication,
            self.readmission,
            self.rejoin_plan_fingerprint,
            self.rejoin,
        )
    }
}
