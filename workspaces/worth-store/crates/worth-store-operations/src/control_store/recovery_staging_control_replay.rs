use std::collections::HashMap;

use super::{
    IndeterminateRecoveryStagingHandle, OperationalControlHistoryViolationKind,
    OperationalOperationId, RecoveryStagingOperationKind,
};

pub(super) struct ReplayedRecoveryStaging {
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_kind: RecoveryStagingOperationKind,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    completed_media_identity: Option<[u8; 32]>,
}

pub(super) fn observe_authorized_staging(
    stages: &mut HashMap<OperationalOperationId, ReplayedRecoveryStaging>,
    operation: &OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_tag: u8,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: Option<[u8; 32]>,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let operation_kind = match operation_tag {
        1 => RecoveryStagingOperationKind::BackupRestore,
        2 => RecoveryStagingOperationKind::PointInTimeRecovery,
        3 => RecoveryStagingOperationKind::Rollback,
        _ => return Ok(()),
    };
    let execution_plan_fingerprint = execution_plan_fingerprint
        .ok_or(OperationalControlHistoryViolationKind::RecoveryStagingBindingMismatch)?;
    if stages.contains_key(operation) {
        return Err(OperationalControlHistoryViolationKind::DuplicateRecoveryStaging);
    }
    stages
        .try_reserve(1)
        .map_err(|_| OperationalControlHistoryViolationKind::DuplicateRecoveryStaging)?;
    stages.insert(
        operation.clone(),
        ReplayedRecoveryStaging {
            authority_identity,
            operation_kind,
            authorization_identity,
            plan_fingerprint,
            execution_plan_fingerprint,
            completed_media_identity: None,
        },
    );
    Ok(())
}

pub(super) fn observe_staging_completed(
    stages: &mut HashMap<OperationalOperationId, ReplayedRecoveryStaging>,
    operation: &OperationalOperationId,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    staged_media_identity: [u8; 32],
) -> Result<(), OperationalControlHistoryViolationKind> {
    let stage = stages.get_mut(operation).ok_or(
        OperationalControlHistoryViolationKind::RecoveryStagingCompletionBeforeAuthorization,
    )?;
    if stage.authorization_identity != authorization_identity
        || stage.plan_fingerprint != plan_fingerprint
        || stage.execution_plan_fingerprint != execution_plan_fingerprint
    {
        return Err(OperationalControlHistoryViolationKind::RecoveryStagingBindingMismatch);
    }
    if staged_media_identity == [0; 32] || stage.completed_media_identity.is_some() {
        return Err(OperationalControlHistoryViolationKind::RecoveryStagingBindingMismatch);
    }
    stage.completed_media_identity = Some(staged_media_identity);
    Ok(())
}

pub(super) fn consume_completed_for_publication(
    stages: &mut HashMap<OperationalOperationId, ReplayedRecoveryStaging>,
    operation: &OperationalOperationId,
    publication_operation_tag: u8,
) -> Result<(), OperationalControlHistoryViolationKind> {
    let expected_kind = match publication_operation_tag {
        1 => RecoveryStagingOperationKind::BackupRestore,
        2 => RecoveryStagingOperationKind::PointInTimeRecovery,
        3 => RecoveryStagingOperationKind::Rollback,
        _ => return Ok(()),
    };
    let stage = stages.get(operation).ok_or(
        OperationalControlHistoryViolationKind::RecoveryPublicationBeforeStagingCompletion,
    )?;
    if stage.operation_kind != expected_kind || stage.completed_media_identity.is_none() {
        return Err(
            OperationalControlHistoryViolationKind::RecoveryPublicationBeforeStagingCompletion,
        );
    }
    stages.remove(operation);
    Ok(())
}

impl ReplayedRecoveryStaging {
    pub(super) fn pending_handle(
        self,
        operation_id: OperationalOperationId,
    ) -> IndeterminateRecoveryStagingHandle {
        IndeterminateRecoveryStagingHandle::new(
            operation_id,
            self.authority_identity,
            self.operation_kind,
            self.authorization_identity,
            self.plan_fingerprint,
            self.execution_plan_fingerprint,
            self.completed_media_identity,
        )
    }
}
