use super::{
    BlobCompactionColdReadiness, BlobCompactionPhysicalInterlock, BlobCompactionReadHold,
    BlobCompactionS6Pacing,
};
use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRegisteredDedupeReference,
    BlobChunkRootPublication, BlobCorruptionGuard, LifecycleReceipt,
};
use forge_store_io_scheduler::S10CompactionIoReadinessHandoff;
use forge_store_physical_isolation::{CompactionReadInterlockDenial, CompactionReadInterlockPlan};
use forge_store_tiering::S7ColdPlacementState;

#[derive(Debug)]
pub struct BlobCompactionIntent {
    lifecycle: LifecycleReceipt,
    uncompacted_publication: BlobChunkRootPublication,
    reachability: Option<BlobChunkReachabilityProofSet>,
    placement: AdmittedBlobPlacement,
    dedupe_references: Vec<BlobChunkRegisteredDedupeReference>,
    quarantine_holds: Vec<BlobCorruptionGuard>,
    read_hold: BlobCompactionReadHold,
    pacing: BlobCompactionS6Pacing,
    cold: BlobCompactionColdReadiness,
    physical: BlobCompactionPhysicalInterlock,
}

impl BlobCompactionIntent {
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
            pacing: BlobCompactionS6Pacing::admitted_compaction(0),
            cold: BlobCompactionColdReadiness::Available(S7ColdPlacementState::HotAvailable),
            physical: BlobCompactionPhysicalInterlock::Admitted(physical),
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
            pacing: BlobCompactionS6Pacing::admitted_compaction(0),
            cold: BlobCompactionColdReadiness::Available(S7ColdPlacementState::HotAvailable),
            physical: BlobCompactionPhysicalInterlock::Admitted(physical),
        }
    }

    pub fn with_read_hold(mut self, read_hold: BlobCompactionReadHold) -> Self {
        self.read_hold = read_hold;
        self
    }

    pub fn with_s6_pacing(mut self, pacing: BlobCompactionS6Pacing) -> Self {
        self.pacing = pacing;
        self
    }

    pub fn with_s10_io_pacing(
        mut self,
        handoff: &S10CompactionIoReadinessHandoff,
        foreground_yields: u64,
    ) -> Self {
        self.pacing = BlobCompactionS6Pacing::from_s10_handoff(handoff, foreground_yields);
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

    pub(crate) fn lifecycle(&self) -> &LifecycleReceipt {
        &self.lifecycle
    }

    pub(crate) const fn uncompacted_publication(&self) -> &BlobChunkRootPublication {
        &self.uncompacted_publication
    }

    pub(crate) fn reachability(&self) -> Option<&BlobChunkReachabilityProofSet> {
        self.reachability.as_ref()
    }

    pub(crate) const fn placement(&self) -> &AdmittedBlobPlacement {
        &self.placement
    }

    pub(crate) fn dedupe_references(&self) -> &[BlobChunkRegisteredDedupeReference] {
        &self.dedupe_references
    }

    pub(crate) fn quarantine_holds(&self) -> &[BlobCorruptionGuard] {
        &self.quarantine_holds
    }

    pub(crate) const fn read_hold(&self) -> BlobCompactionReadHold {
        self.read_hold
    }

    pub(crate) const fn pacing(&self) -> BlobCompactionS6Pacing {
        self.pacing
    }

    pub(crate) const fn cold(&self) -> BlobCompactionColdReadiness {
        self.cold
    }

    pub(crate) const fn physical(&self) -> &BlobCompactionPhysicalInterlock {
        &self.physical
    }
}
