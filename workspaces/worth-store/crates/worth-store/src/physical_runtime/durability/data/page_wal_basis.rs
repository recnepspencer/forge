use worth_proof::CanonicalVec;
use worth_store_physical_format::PhysicalPageLsn;
use worth_store_wal::LogSequenceNumber;

use super::{CertifiedPriorPageBasis, PhysicalDataFrameIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalRedoLsn {
    ordinal: u32,
    lsn: LogSequenceNumber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalRedoTargetClaim {
    target: PhysicalDataFrameIdentity,
    resulting_payload_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageWalBasis {
    target: PhysicalDataFrameIdentity,
    prior: CertifiedPriorPageBasis,
    delta: CanonicalVec<PhysicalRedoLsn>,
    resulting_page_lsn: PhysicalPageLsn,
    resulting_payload_digest: [u8; 32],
}

impl PhysicalRedoLsn {
    pub(in crate::physical_runtime) const fn new(ordinal: u32, lsn: LogSequenceNumber) -> Self {
        Self { ordinal, lsn }
    }

    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub const fn lsn(self) -> LogSequenceNumber {
        self.lsn
    }
}

impl PhysicalRedoTargetClaim {
    pub(in crate::physical_runtime) const fn new(
        target: PhysicalDataFrameIdentity,
        resulting_payload_digest: [u8; 32],
    ) -> Self {
        Self {
            target,
            resulting_payload_digest,
        }
    }

    pub const fn target(self) -> PhysicalDataFrameIdentity {
        self.target
    }

    pub const fn resulting_payload_digest(self) -> [u8; 32] {
        self.resulting_payload_digest
    }
}

impl PageWalBasis {
    pub(in crate::physical_runtime) fn new(
        target: PhysicalDataFrameIdentity,
        prior: CertifiedPriorPageBasis,
        delta: CanonicalVec<PhysicalRedoLsn>,
        resulting_payload_digest: [u8; 32],
    ) -> Option<Self> {
        if !prior.admits_target(target) {
            return None;
        }
        let resulting = delta.as_slice().last()?.lsn();
        if resulting.get() <= prior.page_lsn().get() {
            return None;
        }
        Some(Self {
            target,
            prior,
            delta,
            resulting_page_lsn: PhysicalPageLsn::new(resulting.get()),
            resulting_payload_digest,
        })
    }

    pub const fn target(&self) -> PhysicalDataFrameIdentity {
        self.target
    }

    pub const fn prior(&self) -> CertifiedPriorPageBasis {
        self.prior
    }

    pub fn delta(&self) -> &[PhysicalRedoLsn] {
        self.delta.as_slice()
    }

    pub const fn resulting_page_lsn(&self) -> PhysicalPageLsn {
        self.resulting_page_lsn
    }

    pub const fn resulting_payload_digest(&self) -> [u8; 32] {
        self.resulting_payload_digest
    }
}
