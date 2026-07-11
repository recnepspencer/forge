use forge_store_physical_backend::{
    StoreDurabilityCounterSnapshot, StoreDurabilityOrderingBarrierDurable,
    StoreDurabilityPublicationKind, StoreDurabilityState,
};

use crate::WalFrameDurablePublicationScope;

use crate::{WalOperationDenial, WalOperationDenialKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedWalAppendReceipt {
    scope: WalFrameDurablePublicationScope,
    counters: StoreDurabilityCounterSnapshot,
    state: StoreDurabilityState,
    persisted_path: std::path::PathBuf,
    persisted_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalAppendLayoutReport {
    scope: WalFrameDurablePublicationScope,
    counters: StoreDurabilityCounterSnapshot,
    state: StoreDurabilityState,
}

impl WalAppendLayoutReport {
    fn from_admitted_receipt(receipt: &AdmittedWalAppendReceipt) -> Self {
        Self {
            scope: receipt.scope.clone(),
            counters: receipt.counters,
            state: receipt.state,
        }
    }

    pub fn scope(&self) -> &WalFrameDurablePublicationScope {
        &self.scope
    }

    pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }

    pub const fn state(&self) -> StoreDurabilityState {
        self.state
    }

    pub const fn byte_count(&self) -> u64 {
        self.scope.expected_bytes()
    }

    pub const fn range_span(&self) -> u64 {
        self.scope.lsn_end() - self.scope.lsn_start()
    }
}

impl AdmittedWalAppendReceipt {
    pub(crate) fn from_receipt(
        receipt: &StoreDurabilityOrderingBarrierDurable<WalFrameDurablePublicationScope>,
    ) -> Self {
        Self {
            scope: receipt.scope().clone(),
            counters: receipt.counters(),
            state: receipt.state(),
            persisted_path: receipt.persisted_artifact().path().to_path_buf(),
            persisted_bytes: receipt.persisted_artifact().bytes(),
        }
    }

    pub fn scope(&self) -> &WalFrameDurablePublicationScope {
        &self.scope
    }

    pub fn report(&self) -> WalAppendLayoutReport {
        WalAppendLayoutReport::from_admitted_receipt(self)
    }

    pub const fn counters(&self) -> StoreDurabilityCounterSnapshot {
        self.counters
    }

    pub const fn state(&self) -> StoreDurabilityState {
        self.state
    }

    pub const fn byte_count(&self) -> u64 {
        self.scope.expected_bytes()
    }

    pub const fn range_span(&self) -> u64 {
        self.scope.lsn_end() - self.scope.lsn_start()
    }

    pub fn persisted_path(&self) -> &std::path::Path {
        &self.persisted_path
    }

    pub const fn persisted_bytes(&self) -> u64 {
        self.persisted_bytes
    }
}

pub fn admit_durable_append(
    receipt: &StoreDurabilityOrderingBarrierDurable<WalFrameDurablePublicationScope>,
) -> Result<AdmittedWalAppendReceipt, WalOperationDenial> {
    if receipt.publication() != StoreDurabilityPublicationKind::WalFrame {
        return Err(WalOperationDenial::new(
            WalOperationDenialKind::WrongPublicationKind,
        ));
    }
    Ok(AdmittedWalAppendReceipt::from_receipt(receipt))
}
