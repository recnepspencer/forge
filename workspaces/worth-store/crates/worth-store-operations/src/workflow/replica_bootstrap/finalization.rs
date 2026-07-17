use std::path::Path;

use worth_store_offline_verifier::{
    verify_replica_bootstrap_target, IndependentlyVerifiedReplicaTarget,
    ReplicaTargetVerificationBudget, ReplicaTargetVerificationDenial,
};
use worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt;

use crate::{OperationalControlStore, OperationalControlStorePort, OperationalTransitionId};

use super::{ExecutedReplicaBootstrap, RecoveredReplicaBootstrap};

#[derive(Debug)]
pub enum ReplicaBootstrapFinalizationDenial {
    Verification(ReplicaTargetVerificationDenial),
    EmptyAbandonmentReason,
    Control(crate::OperationalControlAppendDenial),
    SourceLease(worth_store_physical_isolation::RecoverySourceLeaseDenial),
}

#[derive(Debug)]
pub struct PostVerifiedReplicaBootstrap {
    executed: ExecutedReplicaBootstrap,
    verification: IndependentlyVerifiedReplicaTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedReplicaBootstrap {
    operation_identity: [u8; 32],
    receipt_identity: [u8; 32],
    verification_identity: [u8; 32],
    source_release: RecoverySourceLeaseReleaseReceipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbandonedReplicaBootstrap {
    operation_identity: [u8; 32],
    receipt_identity: [u8; 32],
    source_release: RecoverySourceLeaseReleaseReceipt,
}

impl ExecutedReplicaBootstrap {
    pub fn post_verify(
        self,
        target_root: &Path,
        budget: ReplicaTargetVerificationBudget,
    ) -> Result<PostVerifiedReplicaBootstrap, ReplicaBootstrapFinalizationDenial> {
        post_verify(self, target_root, budget)
    }

    pub fn abandon(
        self,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        reason: String,
    ) -> Result<AbandonedReplicaBootstrap, ReplicaBootstrapFinalizationDenial> {
        abandon(self, control, transition, reason)
    }
}

impl RecoveredReplicaBootstrap {
    pub fn post_verify(
        self,
        target_root: &Path,
        budget: ReplicaTargetVerificationBudget,
    ) -> Result<PostVerifiedReplicaBootstrap, ReplicaBootstrapFinalizationDenial> {
        post_verify(self.into_executed(), target_root, budget)
    }

    pub fn abandon(
        self,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        reason: String,
    ) -> Result<AbandonedReplicaBootstrap, ReplicaBootstrapFinalizationDenial> {
        abandon(self.into_executed(), control, transition, reason)
    }

    fn into_executed(self) -> ExecutedReplicaBootstrap {
        ExecutedReplicaBootstrap {
            operation_id: self.operation_id,
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            receipt: self.receipt,
            retained_source_lease: self.retained_source_lease,
        }
    }
}

impl PostVerifiedReplicaBootstrap {
    pub const fn verification(&self) -> &IndependentlyVerifiedReplicaTarget {
        &self.verification
    }

    pub fn complete(
        self,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
    ) -> Result<CompletedReplicaBootstrap, ReplicaBootstrapFinalizationDenial> {
        let record = crate::OperationalControlRecord::replica_bootstrap_completed(
            self.executed.authority_identity,
            self.executed.operation_id.clone(),
            transition,
            &self.executed.receipt,
            &self.verification,
        );
        control
            .append(&record)
            .map_err(ReplicaBootstrapFinalizationDenial::Control)?;
        let source_release = self
            .executed
            .retained_source_lease
            .release()
            .map_err(ReplicaBootstrapFinalizationDenial::SourceLease)?;
        Ok(CompletedReplicaBootstrap {
            operation_identity: self.executed.operation_id.stable_fingerprint(),
            receipt_identity: self.executed.receipt.receipt_identity(),
            verification_identity: self.verification.verification_identity(),
            source_release,
        })
    }
}

fn post_verify(
    executed: ExecutedReplicaBootstrap,
    target_root: &Path,
    budget: ReplicaTargetVerificationBudget,
) -> Result<PostVerifiedReplicaBootstrap, ReplicaBootstrapFinalizationDenial> {
    let verification = verify_replica_bootstrap_target(&executed.receipt, target_root, budget)
        .map_err(ReplicaBootstrapFinalizationDenial::Verification)?;
    Ok(PostVerifiedReplicaBootstrap {
        executed,
        verification,
    })
}

fn abandon(
    executed: ExecutedReplicaBootstrap,
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    reason: String,
) -> Result<AbandonedReplicaBootstrap, ReplicaBootstrapFinalizationDenial> {
    if reason.trim().is_empty() {
        return Err(ReplicaBootstrapFinalizationDenial::EmptyAbandonmentReason);
    }
    let record = crate::OperationalControlRecord::replica_bootstrap_abandoned(
        executed.authority_identity,
        executed.operation_id.clone(),
        transition,
        &executed.receipt,
        reason,
    );
    control
        .append(&record)
        .map_err(ReplicaBootstrapFinalizationDenial::Control)?;
    let source_release = executed
        .retained_source_lease
        .release()
        .map_err(ReplicaBootstrapFinalizationDenial::SourceLease)?;
    Ok(AbandonedReplicaBootstrap {
        operation_identity: executed.operation_id.stable_fingerprint(),
        receipt_identity: executed.receipt.receipt_identity(),
        source_release,
    })
}

impl CompletedReplicaBootstrap {
    pub const fn operation_identity(self) -> [u8; 32] {
        self.operation_identity
    }
    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }
    pub const fn verification_identity(self) -> [u8; 32] {
        self.verification_identity
    }
    pub const fn source_release(self) -> RecoverySourceLeaseReleaseReceipt {
        self.source_release
    }
}

impl AbandonedReplicaBootstrap {
    pub const fn operation_identity(self) -> [u8; 32] {
        self.operation_identity
    }
    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }
    pub const fn source_release(self) -> RecoverySourceLeaseReleaseReceipt {
        self.source_release
    }
}
