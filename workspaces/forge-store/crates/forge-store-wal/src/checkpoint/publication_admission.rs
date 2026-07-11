use forge_store_physical_backend::{
    StoreDurabilityCounterSnapshot, StoreDurabilityOrderingBarrierDurable,
    StoreDurabilityPublicationKind, StoreDurabilityState,
};

use crate::CheckpointDurablePublicationScope;

use crate::{WalOperationDenial, WalOperationDenialKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedCheckpointPublicationReceipt {
    scope: CheckpointDurablePublicationScope,
    counters: StoreDurabilityCounterSnapshot,
    publication: StoreDurabilityPublicationKind,
    state: StoreDurabilityState,
    persisted_path: std::path::PathBuf,
    persisted_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPublicationLayoutReport {
    scope: CheckpointDurablePublicationScope,
    counters: StoreDurabilityCounterSnapshot,
    publication: StoreDurabilityPublicationKind,
    state: StoreDurabilityState,
}

impl CheckpointPublicationLayoutReport {
    fn from_admitted_receipt(receipt: &AdmittedCheckpointPublicationReceipt) -> Self {
        Self {
            scope: receipt.scope.clone(),
            counters: receipt.counters,
            publication: receipt.publication,
            state: receipt.state,
        }
    }

    pub fn scope(&self) -> &CheckpointDurablePublicationScope {
        &self.scope
    }

    pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }

    pub const fn publication(&self) -> StoreDurabilityPublicationKind {
        self.publication
    }

    pub const fn state(&self) -> StoreDurabilityState {
        self.state
    }

    pub const fn range_span(&self) -> u64 {
        self.scope.covered_lsn_end() - self.scope.covered_lsn_start()
    }
}

impl AdmittedCheckpointPublicationReceipt {
    pub(crate) fn from_receipt(
        receipt: &StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
    ) -> Self {
        Self {
            scope: receipt.scope().clone(),
            counters: receipt.counters(),
            publication: receipt.publication(),
            state: receipt.state(),
            persisted_path: receipt.persisted_artifact().path().to_path_buf(),
            persisted_bytes: receipt.persisted_artifact().bytes(),
        }
    }

    pub fn scope(&self) -> &CheckpointDurablePublicationScope {
        &self.scope
    }

    pub fn report(&self) -> CheckpointPublicationLayoutReport {
        CheckpointPublicationLayoutReport::from_admitted_receipt(self)
    }

    pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }

    pub const fn publication(&self) -> StoreDurabilityPublicationKind {
        self.publication
    }

    pub const fn state(&self) -> StoreDurabilityState {
        self.state
    }

    pub const fn range_span(&self) -> u64 {
        self.scope.covered_lsn_end() - self.scope.covered_lsn_start()
    }

    pub fn persisted_path(&self) -> &std::path::Path {
        &self.persisted_path
    }

    pub const fn persisted_bytes(&self) -> u64 {
        self.persisted_bytes
    }
}

pub fn admit_checkpoint_publication(
    receipt: &StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
) -> Result<AdmittedCheckpointPublicationReceipt, WalOperationDenial> {
    if receipt.publication() != StoreDurabilityPublicationKind::Manifest {
        return Err(WalOperationDenial::new(
            WalOperationDenialKind::WrongPublicationKind,
        ));
    }
    Ok(AdmittedCheckpointPublicationReceipt::from_receipt(receipt))
}
