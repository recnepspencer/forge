use super::{
    BlobCompactionColdReadiness, BlobCompactionPacingAdmission, BlobCompactionPhysicalInterlock,
    BlobCompactionReadHold,
};
use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRegisteredDedupeReference,
    BlobChunkRootPublication, BlobCorruptionGuard, LifecycleReceipt,
};
use worth_store_io_scheduler::BackgroundPacingCapability;
use worth_store_physical_isolation::{CompactionReadInterlockDenial, CompactionReadInterlockPlan};
use worth_store_tiering::ColdPlacementState;

#[derive(Debug)]
pub struct BlobCompactionIntent {
    lifecycle: LifecycleReceipt,
    uncompacted_publication: BlobChunkRootPublication,
    reachability: Option<BlobChunkReachabilityProofSet>,
    placement: AdmittedBlobPlacement,
    dedupe_references: Vec<BlobChunkRegisteredDedupeReference>,
    quarantine_holds: Vec<BlobCorruptionGuard>,
    read_hold: BlobCompactionReadHold,
    pacing: BlobCompactionPacingAdmission,
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
            pacing: BlobCompactionPacingAdmission::admitted_compaction(0),
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
            pacing: BlobCompactionPacingAdmission::admitted_compaction(0),
            cold: BlobCompactionColdReadiness::Available(ColdPlacementState::HotAvailable),
            physical: BlobCompactionPhysicalInterlock::Admitted(Box::new(physical)),
        }
    }

    pub fn with_read_hold(mut self, read_hold: BlobCompactionReadHold) -> Self {
        self.read_hold = read_hold;
        self
    }

    pub fn with_pacing_admission(mut self, pacing: BlobCompactionPacingAdmission) -> Self {
        self.pacing = pacing;
        self
    }

    pub fn with_scheduler_pacing(
        mut self,
        capability: BackgroundPacingCapability,
        foreground_yields: u64,
    ) -> Self {
        self.pacing =
            BlobCompactionPacingAdmission::from_scheduler_capability(capability, foreground_yields);
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

    pub(crate) const fn pacing(&self) -> BlobCompactionPacingAdmission {
        self.pacing
    }

    pub(crate) const fn cold(&self) -> BlobCompactionColdReadiness {
        self.cold
    }

    pub(crate) const fn physical(&self) -> &BlobCompactionPhysicalInterlock {
        &self.physical
    }
}
