#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStagingOperationKind {
    BackupRestore,
    PointInTimeRecovery,
    Rollback,
}

/// Durable evidence that staging authorization was consumed and publication
/// has not begun. Rebinding the exact lowered plan is required after restart,
/// including when owner execution completed before process loss.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateRecoveryStagingHandle {
    operation_id: super::OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_kind: RecoveryStagingOperationKind,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    completed_media_identity: Option<[u8; 32]>,
}

impl IndeterminateRecoveryStagingHandle {
    pub(crate) const fn new(
        operation_id: super::OperationalOperationId,
        authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
        operation_kind: RecoveryStagingOperationKind,
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        completed_media_identity: Option<[u8; 32]>,
    ) -> Self {
        Self {
            operation_id,
            authority_identity,
            operation_kind,
            authorization_identity,
            plan_fingerprint,
            execution_plan_fingerprint,
            completed_media_identity,
        }
    }
    pub const fn operation_id(&self) -> &super::OperationalOperationId {
        &self.operation_id
    }
    pub const fn operation_kind(&self) -> RecoveryStagingOperationKind {
        self.operation_kind
    }
    pub const fn authorization_identity(&self) -> [u8; 32] {
        self.authorization_identity
    }
    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn execution_plan_fingerprint(&self) -> [u8; 32] {
        self.execution_plan_fingerprint
    }
    pub const fn completed_media_identity(&self) -> Option<[u8; 32]> {
        self.completed_media_identity
    }
    pub(crate) const fn authority_identity(
        &self,
    ) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }
}
