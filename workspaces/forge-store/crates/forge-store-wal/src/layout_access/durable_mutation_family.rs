use forge_store_physical_backend::{
    StoreDurabilityCounterSnapshot, StoreDurabilityOrderingBarrierDurable,
    StoreDurabilityPublicationKind, StoreDurabilityState,
};

use crate::WalFrameDurablePublicationScope;

use super::{WalLayoutAccessDenial, WalLayoutAccessDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedWalAppendLayoutRule {
    _private: (),
}

impl AdmittedWalAppendLayoutRule {
    pub(crate) const fn internal_phase21() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurableMutationLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurableMutationLayoutAdmission {
    _private: (),
}

impl DurableMutationLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        self,
        _rule: &AdmittedWalAppendLayoutRule,
    ) -> Result<DurableMutationLayoutAdmission, WalLayoutAccessDenial> {
        Ok(DurableMutationLayoutAdmission { _private: () })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedDurableMutationLayoutFamily {
    _admission: DurableMutationLayoutAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedWalAppendReceipt {
    scope: WalFrameDurablePublicationScope,
    counters: StoreDurabilityCounterSnapshot,
    state: StoreDurabilityState,
    persisted_path: std::path::PathBuf,
    persisted_bytes: u64,
}

impl AdmittedDurableMutationLayoutFamily {
    pub(crate) const fn new(admission: DurableMutationLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn admit_append_receipt(
        &self,
        receipt: &StoreDurabilityOrderingBarrierDurable<WalFrameDurablePublicationScope>,
    ) -> Result<AdmittedWalAppendReceipt, WalLayoutAccessDenial> {
        if receipt.publication() != StoreDurabilityPublicationKind::WalFrame {
            return Err(WalLayoutAccessDenial::new(
                WalLayoutAccessDenialKind::WrongPublicationKind,
            ));
        }
        Ok(AdmittedWalAppendReceipt::from_receipt(receipt))
    }

    pub fn append_report(&self, receipt: &AdmittedWalAppendReceipt) -> WalAppendLayoutReport {
        WalAppendLayoutReport::from_admitted_receipt(receipt)
    }
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

    pub(crate) fn persisted_path(&self) -> &std::path::Path {
        &self.persisted_path
    }

    pub(crate) const fn persisted_bytes(&self) -> u64 {
        self.persisted_bytes
    }
}
