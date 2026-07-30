use sha2::{Digest, Sha256};
use worth_store_buffer_pool::{
    FrameWritebackCleanAuthority, PhysicalResidencyDenial, PhysicalWritebackClaim,
};
use worth_store_physical_backend::CompletedArtifactRangeWrite;

pub(in crate::physical_runtime) struct PhysicalResidencyWritebackCompletion {
    identity: super::super::super::PhysicalWorkIdentity,
    claim: PhysicalWritebackClaim,
    receipt: CompletedArtifactRangeWrite,
}

impl PhysicalResidencyWritebackCompletion {
    pub(in crate::physical_runtime) const fn new(
        identity: super::super::super::PhysicalWorkIdentity,
        claim: PhysicalWritebackClaim,
        receipt: CompletedArtifactRangeWrite,
    ) -> Self {
        Self {
            identity,
            claim,
            receipt,
        }
    }

    pub(in crate::physical_runtime) const fn identity(
        &self,
    ) -> super::super::super::PhysicalWorkIdentity {
        self.identity
    }

    pub(in crate::physical_runtime) fn publish_clean(
        self,
        authority: &FrameWritebackCleanAuthority,
    ) -> Result<(), PhysicalResidencyDenial> {
        if !receipt_matches_claim(&self.claim, &self.receipt) {
            return Err(PhysicalResidencyDenial::WriteBackReceiptMismatch);
        }
        self.claim.complete_writeback(authority)
    }
}

fn receipt_matches_claim(
    claim: &PhysicalWritebackClaim,
    receipt: &CompletedArtifactRangeWrite,
) -> bool {
    let [frame] = claim.frames() else {
        return false;
    };
    let Some(bytes) = claim.frame_bytes(0) else {
        return false;
    };
    let digest: [u8; 32] = Sha256::digest(bytes).into();
    receipt.store() == claim.store_identity()
        && receipt.coordinate() == frame.coordinate()
        && receipt.completed_bytes() == bytes.len() as u64
        && receipt.payload_digest() == digest
}

#[cfg(test)]
#[path = "residency_writeback/tests.rs"]
mod tests;
