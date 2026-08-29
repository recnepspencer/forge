//! Independently borrowable Relational owner settlement service.

use std::sync::Weak;

use crate::history::data::{CommitId, RelationalCommitReceipt};
use crate::publication::data::{DeferredPublicationSettlement, DeferredPublicationSettlementError};
use crate::runtime::{
    AdmittedRelationalRuntimeOperation, RelationalRuntime, RelationalRuntimeOwnerBinding,
    RelationalRuntimePublicationBinding, RelationalRuntimeState,
};
use crate::transactions::data::{CommitResult, TransactionCommitError};

/// Cloneable settlement and repair service bound to one live relational runtime
/// owner.
///
/// The service addresses its owner's exact state by weak binding, so it never
/// keeps a finished runtime alive and answers a typed owner-unavailable posture
/// once the owner is gone. It never reconstructs settlement authority: the
/// commit identity it accepts is only a lookup key into the runtime's own
/// pending-settlement registry.
#[derive(Debug, Clone)]
pub struct RelationalSettlementPort {
    runtime_instance_id: u64,
    lifecycle: RelationalRuntimeOwnerBinding,
    publication_binding: RelationalRuntimePublicationBinding,
    state: Weak<RelationalRuntimeState>,
}

impl RelationalSettlementPort {
    pub(crate) fn new(
        runtime_instance_id: u64,
        lifecycle: RelationalRuntimeOwnerBinding,
        publication_binding: RelationalRuntimePublicationBinding,
        state: Weak<RelationalRuntimeState>,
    ) -> Self {
        Self {
            runtime_instance_id,
            lifecycle,
            publication_binding,
            state,
        }
    }

    /// The runtime instance this service settles for.
    pub const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    /// Complete durability and derived publication for a movement already
    /// performed through the publication service.
    ///
    /// The work was installed in the runtime's pending-settlement registry
    /// before that movement, so this is the witness holder taking its turn at
    /// the one executor gate.
    pub fn settle_performed_publication(
        &self,
        performed: crate::mvcc::PerformedRelationalCommit,
    ) -> Result<CommitResult, TransactionCommitError> {
        let Some(owner) = self.owner() else {
            return Err(TransactionCommitError::publication_denied(
                crate::mvcc::RelationalPublicationDenial::OwnerUnavailable {
                    runtime_instance_id: self.runtime_instance_id,
                },
            ));
        };
        owner.runtime.settle_performed_publication(performed)
    }

    /// Retry the one missing durable append for an exact performed route.
    pub fn repair_deferred_publication_settlement(
        &self,
        settlement: &DeferredPublicationSettlement,
    ) -> Result<RelationalCommitReceipt, DeferredPublicationSettlementError> {
        let Some(owner) = self.owner() else {
            return Err(self.owner_unavailable());
        };
        owner
            .runtime
            .repair_deferred_publication_settlement(settlement)
    }

    /// Retry a performed publication this runtime retains, addressed by commit
    /// identity, even when the caller that first held the repair capability was
    /// lost.
    pub fn repair_pending_publication_settlement(
        &self,
        commit_id: CommitId,
    ) -> Result<RelationalCommitReceipt, DeferredPublicationSettlementError> {
        let Some(owner) = self.owner() else {
            return Err(self.owner_unavailable());
        };
        owner
            .runtime
            .repair_pending_publication_settlement(commit_id)
    }

    /// Whether this runtime still retains an unsettled record for the identity.
    pub fn retains_pending_settlement(&self, commit_id: CommitId) -> bool {
        self.publication_binding
            .pending_settlement(commit_id)
            .is_some()
    }

    /// A transient owner handle for one settlement operation.
    ///
    /// Upgrading keeps the state alive for exactly as long as the operation
    /// runs, so a concurrent owner drop cannot tear the state out from under a
    /// settlement already in flight. The lifecycle gate is consulted first so a
    /// closed owner denies before any work begins.
    fn owner(&self) -> Option<AdmittedSettlementOwner> {
        let operation = self.lifecycle.admit()?;
        let runtime = RelationalRuntime::from_shared(self.state.upgrade()?);
        Some(AdmittedSettlementOwner {
            runtime,
            _operation: operation,
        })
    }

    fn owner_unavailable(&self) -> DeferredPublicationSettlementError {
        DeferredPublicationSettlementError::OwnerUnavailable {
            runtime_instance_id: self.runtime_instance_id,
        }
    }
}

impl RelationalRuntime {
    /// The independently borrowable settlement service for this runtime.
    pub fn settlement_port(&self) -> RelationalSettlementPort {
        RelationalSettlementPort::new(
            self.runtime_instance_id(),
            self.owner_binding(),
            self.publication_binding(),
            self.state_binding(),
        )
    }
}

/// One admitted settlement operation and the owner handle it runs against.
///
/// Both are held for the whole operation: the admission keeps the owner from
/// finishing its close while work is outstanding, and the handle keeps the
/// state alive for exactly that long.
struct AdmittedSettlementOwner {
    runtime: RelationalRuntime,
    _operation: AdmittedRelationalRuntimeOperation,
}
