use std::collections::HashMap;

use super::replica_operation_control_replay::ReplayedReplicaPromotion;
use super::{
    OperationalControlHistoryViolationKind, OperationalOperationId, RecoveredOldPrimaryRejoin,
};

pub(super) fn observe_old_primary_rejoin(
    promotions: &mut HashMap<OperationalOperationId, ReplayedReplicaPromotion>,
    operation: &OperationalOperationId,
    promotion_receipt_identity: [u8; 32],
    rejoin_plan_fingerprint: [u8; 32],
    disposition_tag: u8,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = promotions
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    let receipt = state
        .receipt
        .ok_or(OperationalControlHistoryViolationKind::DuplicateReplicaOperationStage)?;
    if state.readmission.is_none()
        || state.rejoin_plan_fingerprint.is_some()
        || state.rejoin.is_some()
        || receipt.receipt_identity() != promotion_receipt_identity
        || rejoin_plan_fingerprint == [0; 32]
        || disposition_tag > 2
    {
        return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
    }
    state.rejoin_plan_fingerprint = Some(rejoin_plan_fingerprint);
    state.rejoin_disposition_tag = Some(disposition_tag);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn observe_old_primary_rejoin_completion(
    promotions: &mut HashMap<OperationalOperationId, ReplayedReplicaPromotion>,
    operation: &OperationalOperationId,
    rejoin_plan_fingerprint: [u8; 32],
    rejoin_receipt_identity: [u8; 32],
    forensic_retention_identity: [u8; 32],
    rebootstrap_target_identity: [u8; 32],
    disposition_tag: u8,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let state = promotions
        .get_mut(operation)
        .ok_or(OperationalControlHistoryViolationKind::ReplicaRecordBeforeAuthorization)?;
    if state.rejoin.is_some()
        || state.rejoin_plan_fingerprint != Some(rejoin_plan_fingerprint)
        || state.rejoin_disposition_tag != Some(disposition_tag)
        || rejoin_receipt_identity == [0; 32]
    {
        return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
    }
    let forensic = (forensic_retention_identity != [0; 32]).then_some(forensic_retention_identity);
    let target = (rebootstrap_target_identity != [0; 32]).then_some(rebootstrap_target_identity);
    let valid_disposition = match disposition_tag {
        0 => forensic.is_some() && target.is_none(),
        1 => target.is_none(),
        2 => forensic.is_some() && target.is_some(),
        _ => false,
    };
    if !valid_disposition {
        return Err(OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch);
    }
    state.rejoin = Some(RecoveredOldPrimaryRejoin::new(
        rejoin_receipt_identity,
        forensic,
        target,
    ));
    Ok(())
}
