use worth_store_physical_backend::{
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
    persisted_frame_offset: u64,
    persisted_offset: u64,
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
    ) -> Result<Self, WalOperationDenial> {
        let artifact = receipt.persisted_artifact();
        let (persisted_offset, persisted_bytes) =
            crate::artifact_store::validate_persisted_wal_frame(
                artifact.path(),
                artifact.offset(),
                artifact.bytes(),
                receipt.scope(),
            )
            .map_err(|_| {
                WalOperationDenial::new(WalOperationDenialKind::PersistedArtifactInvalid)
            })?;
        Ok(Self {
            scope: receipt.scope().clone(),
            counters: receipt.counters(),
            state: receipt.state(),
            persisted_path: artifact.path().to_path_buf(),
            persisted_frame_offset: artifact.offset(),
            persisted_offset,
            persisted_bytes,
        })
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

    pub const fn persisted_offset(&self) -> u64 {
        self.persisted_offset
    }

    pub const fn persisted_frame_offset(&self) -> u64 {
        self.persisted_frame_offset
    }

    pub fn persisted_payload_matches(&self, expected: &[u8]) -> bool {
        use std::io::{Read, Seek, SeekFrom};

        if self.persisted_bytes != expected.len() as u64 {
            return false;
        }
        let mut persisted = vec![0; expected.len()];
        std::fs::File::open(&self.persisted_path)
            .and_then(|mut file| {
                file.seek(SeekFrom::Start(self.persisted_offset))?;
                file.read_exact(&mut persisted)
            })
            .is_ok_and(|()| persisted == expected)
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
    AdmittedWalAppendReceipt::from_receipt(receipt)
}
