#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredReplicaBootstrapTransfer {
    receipt_identity: [u8; 32],
    durable_target_identity: [u8; 32],
    source_lease_identity: [u8; 32],
    execution_counters: worth_store_replication::ReplicaBootstrapExecutionCounters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveredReplicaBootstrapDisposition {
    Completed { verification_identity: [u8; 32] },
    Abandoned,
}

impl RecoveredReplicaBootstrapTransfer {
    pub(crate) const fn new(
        receipt_identity: [u8; 32],
        durable_target_identity: [u8; 32],
        source_lease_identity: [u8; 32],
        execution_counters: worth_store_replication::ReplicaBootstrapExecutionCounters,
    ) -> Self {
        Self {
            receipt_identity,
            durable_target_identity,
            source_lease_identity,
            execution_counters,
        }
    }

    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }

    pub const fn durable_target_identity(self) -> [u8; 32] {
        self.durable_target_identity
    }

    pub const fn source_lease_identity(self) -> [u8; 32] {
        self.source_lease_identity
    }

    pub const fn execution_counters(
        self,
    ) -> worth_store_replication::ReplicaBootstrapExecutionCounters {
        self.execution_counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaBootstrapRecoveryHandle {
    operation_id: super::OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    authorization_identity: [u8; 32],
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    transfer: Option<RecoveredReplicaBootstrapTransfer>,
    disposition: Option<RecoveredReplicaBootstrapDisposition>,
}

impl ReplicaBootstrapRecoveryHandle {
    pub(crate) const fn new(
        operation_id: super::OperationalOperationId,
        authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
        authorization_identity: [u8; 32],
        authorization_plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        transfer: Option<RecoveredReplicaBootstrapTransfer>,
        disposition: Option<RecoveredReplicaBootstrapDisposition>,
    ) -> Self {
        Self {
            operation_id,
            authority_identity,
            authorization_identity,
            authorization_plan_fingerprint,
            execution_plan_fingerprint,
            transfer,
            disposition,
        }
    }

    pub const fn operation_id(&self) -> &super::OperationalOperationId {
        &self.operation_id
    }

    pub const fn authorization_identity(&self) -> [u8; 32] {
        self.authorization_identity
    }

    pub const fn authorization_plan_fingerprint(&self) -> [u8; 32] {
        self.authorization_plan_fingerprint
    }

    pub const fn execution_plan_fingerprint(&self) -> [u8; 32] {
        self.execution_plan_fingerprint
    }

    pub const fn transfer(&self) -> Option<RecoveredReplicaBootstrapTransfer> {
        self.transfer
    }

    pub const fn disposition(&self) -> Option<RecoveredReplicaBootstrapDisposition> {
        self.disposition
    }

    pub const fn authority_identity(&self) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredReplicaPromotionFence {
    fence_identity: [u8; 32],
    promoted_epoch: u64,
}

impl RecoveredReplicaPromotionFence {
    pub(crate) const fn new(fence_identity: [u8; 32], promoted_epoch: u64) -> Self {
        Self {
            fence_identity,
            promoted_epoch,
        }
    }

    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }

    pub const fn promoted_epoch(self) -> u64 {
        self.promoted_epoch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredReplicaPromotionReceipt {
    receipt_identity: [u8; 32],
    fence_identity: [u8; 32],
    promoted_epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredReplicaPromotionPublication {
    publication_identity: [u8; 32],
    verification_identity: [u8; 32],
}

impl RecoveredReplicaPromotionPublication {
    pub(crate) const fn new(
        publication_identity: [u8; 32],
        verification_identity: [u8; 32],
    ) -> Self {
        Self {
            publication_identity,
            verification_identity,
        }
    }
    pub const fn publication_identity(self) -> [u8; 32] {
        self.publication_identity
    }
    pub const fn verification_identity(self) -> [u8; 32] {
        self.verification_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredReplicaPromotionReadmission {
    serve_lease_identity: [u8; 32],
    serving_epoch: u64,
}

impl RecoveredReplicaPromotionReadmission {
    pub(crate) const fn new(serve_lease_identity: [u8; 32], serving_epoch: u64) -> Self {
        Self {
            serve_lease_identity,
            serving_epoch,
        }
    }
    pub const fn serve_lease_identity(self) -> [u8; 32] {
        self.serve_lease_identity
    }
    pub const fn serving_epoch(self) -> u64 {
        self.serving_epoch
    }
}

impl RecoveredReplicaPromotionReceipt {
    pub(crate) const fn new(
        receipt_identity: [u8; 32],
        fence_identity: [u8; 32],
        promoted_epoch: u64,
    ) -> Self {
        Self {
            receipt_identity,
            fence_identity,
            promoted_epoch,
        }
    }

    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }

    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }

    pub const fn promoted_epoch(self) -> u64 {
        self.promoted_epoch
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicaPromotionRecoveryHandle {
    operation_id: super::OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    authorization_identity: [u8; 32],
    authorization_plan_fingerprint: [u8; 32],
    execution_plan_fingerprint: [u8; 32],
    fence: Option<RecoveredReplicaPromotionFence>,
    receipt: Option<RecoveredReplicaPromotionReceipt>,
    publication: Option<RecoveredReplicaPromotionPublication>,
    readmission: Option<RecoveredReplicaPromotionReadmission>,
    rejoin_plan_fingerprint: Option<[u8; 32]>,
    rejoin: Option<RecoveredOldPrimaryRejoin>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredOldPrimaryRejoin {
    receipt_identity: [u8; 32],
    forensic_retention_identity: Option<[u8; 32]>,
    rebootstrap_target_identity: Option<[u8; 32]>,
}

impl RecoveredOldPrimaryRejoin {
    pub(crate) const fn new(
        receipt_identity: [u8; 32],
        forensic_retention_identity: Option<[u8; 32]>,
        rebootstrap_target_identity: Option<[u8; 32]>,
    ) -> Self {
        Self {
            receipt_identity,
            forensic_retention_identity,
            rebootstrap_target_identity,
        }
    }

    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }
    pub const fn forensic_retention_identity(self) -> Option<[u8; 32]> {
        self.forensic_retention_identity
    }
    pub const fn rebootstrap_target_identity(self) -> Option<[u8; 32]> {
        self.rebootstrap_target_identity
    }
}

impl ReplicaPromotionRecoveryHandle {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        operation_id: super::OperationalOperationId,
        authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
        authorization_identity: [u8; 32],
        authorization_plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        fence: Option<RecoveredReplicaPromotionFence>,
        receipt: Option<RecoveredReplicaPromotionReceipt>,
        publication: Option<RecoveredReplicaPromotionPublication>,
        readmission: Option<RecoveredReplicaPromotionReadmission>,
        rejoin_plan_fingerprint: Option<[u8; 32]>,
        rejoin: Option<RecoveredOldPrimaryRejoin>,
    ) -> Self {
        Self {
            operation_id,
            authority_identity,
            authorization_identity,
            authorization_plan_fingerprint,
            execution_plan_fingerprint,
            fence,
            receipt,
            publication,
            readmission,
            rejoin_plan_fingerprint,
            rejoin,
        }
    }

    pub const fn operation_id(&self) -> &super::OperationalOperationId {
        &self.operation_id
    }

    pub const fn authority_identity(&self) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }

    pub const fn authorization_identity(&self) -> [u8; 32] {
        self.authorization_identity
    }

    pub const fn authorization_plan_fingerprint(&self) -> [u8; 32] {
        self.authorization_plan_fingerprint
    }

    pub const fn execution_plan_fingerprint(&self) -> [u8; 32] {
        self.execution_plan_fingerprint
    }

    pub const fn fence(&self) -> Option<RecoveredReplicaPromotionFence> {
        self.fence
    }

    pub const fn receipt(&self) -> Option<RecoveredReplicaPromotionReceipt> {
        self.receipt
    }
    pub const fn publication(&self) -> Option<RecoveredReplicaPromotionPublication> {
        self.publication
    }
    pub const fn readmission(&self) -> Option<RecoveredReplicaPromotionReadmission> {
        self.readmission
    }
    pub const fn rejoin_plan_fingerprint(&self) -> Option<[u8; 32]> {
        self.rejoin_plan_fingerprint
    }
    pub const fn completed_rejoin(&self) -> Option<RecoveredOldPrimaryRejoin> {
        self.rejoin
    }
}
