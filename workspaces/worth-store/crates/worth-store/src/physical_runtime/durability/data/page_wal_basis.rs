use sha2::{Digest, Sha256};
use worth_proof::CanonicalVec;
use worth_store_physical_format::{decode_data_frame_page_lsn, DurableFrameKind, PhysicalPageLsn};
use worth_store_wal::LogSequenceNumber;

use super::{CertifiedPriorPageBasis, PhysicalDataFrameIdentity};

#[cfg(test)]
#[path = "page_wal_basis/causal_extension_tests.rs"]
mod causal_extension_tests;

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
    pub(in crate::physical_runtime) fn from_encoded_frame(
        target: PhysicalDataFrameIdentity,
        prior: CertifiedPriorPageBasis,
        delta: CanonicalVec<PhysicalRedoLsn>,
        encoded_frame: &[u8],
    ) -> Option<Self> {
        if !prior.admits_target(target) {
            return None;
        }
        let resulting = strictly_advancing_result(prior.page_lsn(), delta.as_slice())?;
        let resulting_page_lsn = PhysicalPageLsn::new(resulting.get());
        if decode_data_frame_page_lsn(encoded_frame, durable_kind(target.kind()))
            != Ok(resulting_page_lsn)
        {
            return None;
        }
        Some(Self {
            target,
            prior,
            delta,
            resulting_page_lsn,
            resulting_payload_digest: Sha256::digest(encoded_frame).into(),
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

const fn durable_kind(kind: super::PhysicalDataFrameKind) -> DurableFrameKind {
    match kind {
        super::PhysicalDataFrameKind::InlinePage => DurableFrameKind::InlinePage,
        super::PhysicalDataFrameKind::ExtentChunk => DurableFrameKind::Extent,
    }
}

fn strictly_advancing_result(
    prior: PhysicalPageLsn,
    delta: &[PhysicalRedoLsn],
) -> Option<LogSequenceNumber> {
    let mut previous = prior.get();
    for redo in delta {
        let current = redo.lsn().get();
        if current <= previous {
            return None;
        }
        previous = current;
    }
    delta.last().map(|redo| redo.lsn())
}
