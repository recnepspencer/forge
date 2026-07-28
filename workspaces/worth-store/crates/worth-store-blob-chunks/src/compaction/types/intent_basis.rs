use super::{
    BlobCompactionColdReadiness, BlobCompactionIntent, BlobCompactionPacingAdmission,
    BlobCompactionPacingDenial, BlobCompactionPhysicalInterlock, BlobCompactionReadHold,
};
use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRegisteredDedupeReference,
    BlobChunkRootPublication, BlobCorruptionGuard, LifecycleReceipt,
};
use worth_store_io_scheduler::BackgroundIdleCapacityLease;
use worth_store_physical_isolation::{CompactionReadInterlockDenial, CompactionReadInterlockPlan};
use worth_store_tiering::ColdPlacementState;

/// Fully configured blob-compaction meaning before scheduler execution authority is attached.
#[derive(Debug)]
pub struct BlobCompactionIntentBasis {
    pub(crate) lifecycle: LifecycleReceipt,
    pub(crate) uncompacted_publication: BlobChunkRootPublication,
    pub(crate) reachability: Option<BlobChunkReachabilityProofSet>,
    pub(crate) placement: AdmittedBlobPlacement,
    pub(crate) dedupe_references: Vec<BlobChunkRegisteredDedupeReference>,
    pub(crate) quarantine_holds: Vec<BlobCorruptionGuard>,
    pub(crate) read_hold: BlobCompactionReadHold,
    pub(crate) cold: BlobCompactionColdReadiness,
    pub(crate) physical: BlobCompactionPhysicalInterlock,
}

impl BlobCompactionIntentBasis {
    pub fn for_published_generation(
        lifecycle: LifecycleReceipt,
        uncompacted_publication: BlobChunkRootPublication,
        reachability: BlobChunkReachabilityProofSet,
        placement: AdmittedBlobPlacement,
        read_hold: BlobCompactionReadHold,
        physical: CompactionReadInterlockPlan,
    ) -> Self {
        Self {
            lifecycle,
            uncompacted_publication,
            reachability: Some(reachability),
            placement,
            dedupe_references: Vec::new(),
            quarantine_holds: Vec::new(),
            read_hold,
            cold: BlobCompactionColdReadiness::Available(ColdPlacementState::HotAvailable),
            physical: BlobCompactionPhysicalInterlock::Admitted(Box::new(physical)),
        }
    }

    pub fn without_reachability(
        lifecycle: LifecycleReceipt,
        uncompacted_publication: BlobChunkRootPublication,
        placement: AdmittedBlobPlacement,
        read_hold: BlobCompactionReadHold,
        physical: CompactionReadInterlockPlan,
    ) -> Self {
        Self {
            lifecycle,
            uncompacted_publication,
            reachability: None,
            placement,
            dedupe_references: Vec::new(),
            quarantine_holds: Vec::new(),
            read_hold,
            cold: BlobCompactionColdReadiness::Available(ColdPlacementState::HotAvailable),
            physical: BlobCompactionPhysicalInterlock::Admitted(Box::new(physical)),
        }
    }

    pub fn with_read_hold(mut self, read_hold: BlobCompactionReadHold) -> Self {
        self.read_hold = read_hold;
        self
    }

    pub fn with_cold_readiness(mut self, cold: BlobCompactionColdReadiness) -> Self {
        self.cold = cold;
        self
    }

    pub fn with_dedupe_references(
        mut self,
        references: impl IntoIterator<Item = BlobChunkRegisteredDedupeReference>,
    ) -> Self {
        self.dedupe_references = references.into_iter().collect();
        self
    }

    pub fn with_quarantine_holds(
        mut self,
        holds: impl IntoIterator<Item = BlobCorruptionGuard>,
    ) -> Self {
        self.quarantine_holds = holds.into_iter().collect();
        self
    }

    pub fn with_physical_interlock_denial(mut self, denial: CompactionReadInterlockDenial) -> Self {
        self.physical = BlobCompactionPhysicalInterlock::Denied(denial);
        self
    }

    pub fn with_scheduler_pacing(
        self,
        lease: BackgroundIdleCapacityLease,
    ) -> Result<BlobCompactionIntent, BlobCompactionPacingDenial> {
        let pacing = BlobCompactionPacingAdmission::from_scheduler_lease(lease)?;
        Ok(BlobCompactionIntent::from_basis(self, pacing))
    }
}
