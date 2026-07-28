use super::{
    BlobCompactionColdReadiness, BlobCompactionIntentBasis, BlobCompactionPacingAdmission,
    BlobCompactionPhysicalInterlock, BlobCompactionReadHold,
};
use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRegisteredDedupeReference,
    BlobChunkRootPublication, BlobCorruptionGuard, LifecycleReceipt,
};

/// A compaction intent that has consumed exact scheduler-issued execution capacity.
#[derive(Debug)]
pub struct BlobCompactionIntent {
    basis: BlobCompactionIntentBasis,
    pacing: BlobCompactionPacingAdmission,
}

pub(crate) struct BlobCompactionIntentParts {
    pub(crate) lifecycle: LifecycleReceipt,
    pub(crate) uncompacted_publication: BlobChunkRootPublication,
    pub(crate) reachability: Option<BlobChunkReachabilityProofSet>,
    pub(crate) placement: AdmittedBlobPlacement,
    pub(crate) dedupe_references: Vec<BlobChunkRegisteredDedupeReference>,
    pub(crate) pacing: BlobCompactionPacingAdmission,
    pub(crate) physical: BlobCompactionPhysicalInterlock,
}

impl BlobCompactionIntent {
    pub(super) const fn from_basis(
        basis: BlobCompactionIntentBasis,
        pacing: BlobCompactionPacingAdmission,
    ) -> Self {
        Self { basis, pacing }
    }

    pub(crate) fn lifecycle(&self) -> &LifecycleReceipt {
        &self.basis.lifecycle
    }

    pub(crate) const fn uncompacted_publication(&self) -> &BlobChunkRootPublication {
        &self.basis.uncompacted_publication
    }

    pub(crate) fn reachability(&self) -> Option<&BlobChunkReachabilityProofSet> {
        self.basis.reachability.as_ref()
    }

    pub(crate) const fn placement(&self) -> &AdmittedBlobPlacement {
        &self.basis.placement
    }

    pub(crate) fn dedupe_references(&self) -> &[BlobChunkRegisteredDedupeReference] {
        &self.basis.dedupe_references
    }

    pub(crate) fn quarantine_holds(&self) -> &[BlobCorruptionGuard] {
        &self.basis.quarantine_holds
    }

    pub(crate) const fn read_hold(&self) -> BlobCompactionReadHold {
        self.basis.read_hold
    }

    pub(crate) const fn cold(&self) -> BlobCompactionColdReadiness {
        self.basis.cold
    }

    pub(crate) const fn physical(&self) -> &BlobCompactionPhysicalInterlock {
        &self.basis.physical
    }

    pub(crate) fn into_parts(self) -> BlobCompactionIntentParts {
        BlobCompactionIntentParts {
            lifecycle: self.basis.lifecycle,
            uncompacted_publication: self.basis.uncompacted_publication,
            reachability: self.basis.reachability,
            placement: self.basis.placement,
            dedupe_references: self.basis.dedupe_references,
            pacing: self.pacing,
            physical: self.basis.physical,
        }
    }
}
