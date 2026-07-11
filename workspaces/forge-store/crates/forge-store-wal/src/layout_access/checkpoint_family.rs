use forge_store_physical_backend::{
    StoreDurabilityCounterSnapshot, StoreDurabilityOrderingBarrierDurable,
    StoreDurabilityPublicationKind, StoreDurabilityState,
};

use crate::CheckpointDurablePublicationScope;

use super::{WalLayoutAccessDenial, WalLayoutAccessDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedCheckpointLayoutRule {
    _private: (),
}

impl AdmittedCheckpointLayoutRule {
    pub(crate) const fn internal_phase21() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointLayoutAdmission {
    _private: (),
}

impl CheckpointLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        self,
        _rule: &AdmittedCheckpointLayoutRule,
    ) -> Result<CheckpointLayoutAdmission, WalLayoutAccessDenial> {
        Ok(CheckpointLayoutAdmission { _private: () })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedCheckpointLayoutFamily {
    _admission: CheckpointLayoutAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedCheckpointPublicationReceipt {
    scope: CheckpointDurablePublicationScope,
    counters: StoreDurabilityCounterSnapshot,
    publication: StoreDurabilityPublicationKind,
    state: StoreDurabilityState,
    persisted_path: std::path::PathBuf,
    persisted_bytes: u64,
}

impl AdmittedCheckpointLayoutFamily {
    pub(crate) const fn new(admission: CheckpointLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn admit_checkpoint_publication_receipt(
        &self,
        receipt: &StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
    ) -> Result<AdmittedCheckpointPublicationReceipt, WalLayoutAccessDenial> {
        self.publication_report(receipt, StoreDurabilityPublicationKind::Checkpoint)
    }

    pub fn admit_manifest_publication_receipt(
        &self,
        receipt: &StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
    ) -> Result<AdmittedCheckpointPublicationReceipt, WalLayoutAccessDenial> {
        self.publication_report(receipt, StoreDurabilityPublicationKind::Manifest)
    }

    pub fn publication_report_for(
        &self,
        receipt: &AdmittedCheckpointPublicationReceipt,
    ) -> CheckpointPublicationLayoutReport {
        CheckpointPublicationLayoutReport::from_admitted_receipt(receipt)
    }

    fn publication_report(
        &self,
        receipt: &StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope>,
        expected: StoreDurabilityPublicationKind,
    ) -> Result<AdmittedCheckpointPublicationReceipt, WalLayoutAccessDenial> {
        if receipt.publication() != expected {
            return Err(WalLayoutAccessDenial::new(
                WalLayoutAccessDenialKind::WrongPublicationKind,
            ));
        }
        Ok(AdmittedCheckpointPublicationReceipt::from_receipt(receipt))
    }
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

    pub(crate) fn persisted_path(&self) -> &std::path::Path {
        &self.persisted_path
    }

    pub(crate) const fn persisted_bytes(&self) -> u64 {
        self.persisted_bytes
    }
}
