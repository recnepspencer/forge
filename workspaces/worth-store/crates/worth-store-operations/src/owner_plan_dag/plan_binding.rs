use sha2::{Digest, Sha256};
use worth_store_authority::StoreCurrentAuthorityIdentity;
use worth_store_security::{StoreSecurityScopeAdmissionReceipt, StoreSecurityScopeIdentity};

use super::CanonicalOwnerPlanDag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DestructiveOperationKind {
    BackupRestore,
    PointInTimeRecovery,
    Rollback,
    DerivedRepair,
    AuthorityAffectingRepair,
    BackupRestoreCutover,
    PointInTimeRecoveryCutover,
    RollbackCutover,
    AuthorityAffectingRepairCutover,
    ReplicaBootstrap,
    ReplicaPromotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalSecurityScope {
    identity: StoreSecurityScopeIdentity,
    fingerprint: [u8; 32],
}

impl OperationalSecurityScope {
    pub fn from_admission(receipt: StoreSecurityScopeAdmissionReceipt) -> Self {
        let identity = receipt.identity();
        Self {
            identity,
            fingerprint: identity.stable_fingerprint(),
        }
    }

    pub const fn identity(self) -> StoreSecurityScopeIdentity {
        self.identity
    }
    pub const fn fingerprint(self) -> [u8; 32] {
        self.fingerprint
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OperationalPlanBinding {
    operation: DestructiveOperationKind,
    dag: CanonicalOwnerPlanDag,
    authority_identity: StoreCurrentAuthorityIdentity,
    security_scope: OperationalSecurityScope,
    source_identity: [u8; 32],
    target_identity: [u8; 32],
    frontier_identity: [u8; 32],
    fingerprint: [u8; 32],
}

impl OperationalPlanBinding {
    pub(crate) fn bind(
        operation: DestructiveOperationKind,
        dag: CanonicalOwnerPlanDag,
        authority_identity: StoreCurrentAuthorityIdentity,
        security_scope: OperationalSecurityScope,
        source_identity: [u8; 32],
        target_identity: [u8; 32],
        frontier_identity: [u8; 32],
    ) -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-operational-plan-binding-v1");
        digest.update([operation_tag(operation)]);
        digest.update(dag.explanation().plan_fingerprint());
        digest.update(authority_identity.fingerprint());
        digest.update(security_scope.fingerprint);
        digest.update(source_identity);
        digest.update(target_identity);
        digest.update(frontier_identity);
        let fingerprint = digest.finalize().into();
        Self {
            operation,
            dag,
            authority_identity,
            security_scope,
            source_identity,
            target_identity,
            frontier_identity,
            fingerprint,
        }
    }

    pub(crate) const fn authority_identity(&self) -> StoreCurrentAuthorityIdentity {
        self.authority_identity
    }
    pub(crate) const fn operation_tag(&self) -> u8 {
        operation_tag(self.operation)
    }
    pub(crate) const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
    pub(crate) const fn source_identity(&self) -> [u8; 32] {
        self.source_identity
    }
    pub(crate) const fn target_identity(&self) -> [u8; 32] {
        self.target_identity
    }
    pub(crate) const fn frontier_identity(&self) -> [u8; 32] {
        self.frontier_identity
    }
    pub(crate) const fn security_scope(&self) -> OperationalSecurityScope {
        self.security_scope
    }
}

pub(crate) const fn operation_tag(operation: DestructiveOperationKind) -> u8 {
    match operation {
        DestructiveOperationKind::BackupRestore => 1,
        DestructiveOperationKind::PointInTimeRecovery => 2,
        DestructiveOperationKind::Rollback => 3,
        DestructiveOperationKind::DerivedRepair => 4,
        DestructiveOperationKind::AuthorityAffectingRepair => 5,
        DestructiveOperationKind::BackupRestoreCutover => 6,
        DestructiveOperationKind::PointInTimeRecoveryCutover => 7,
        DestructiveOperationKind::RollbackCutover => 8,
        DestructiveOperationKind::AuthorityAffectingRepairCutover => 9,
        DestructiveOperationKind::ReplicaBootstrap => 10,
        DestructiveOperationKind::ReplicaPromotion => 11,
    }
}
