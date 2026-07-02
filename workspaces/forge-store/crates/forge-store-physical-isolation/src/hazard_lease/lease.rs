use crate::{
    CompactProtectedReferenceSet, CurrentGenerationPhysicalReference,
    PhysicalReadPlanAdmissionDenial, PhysicalReadReachabilityBarrier,
    ProtectedPhysicalReferenceSet, PublishedReaderHazard, ReadPlanAdmissionScratchArena,
};

use super::HazardLeaseDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HazardLeaseKind {
    ForegroundRead,
    ScrubWindow,
    RecoveryVerifier,
    CheckpointPreservation,
    QuarantineHold,
    FutureChunkHold,
    BufferPoolPin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedReferenceLease {
    kind: HazardLeaseKind,
    barrier: PhysicalReadReachabilityBarrier,
    footprint: CompactProtectedReferenceSet,
}

impl ProtectedReferenceLease {
    pub fn from_reader_hazard(
        hazard: &PublishedReaderHazard,
        footprint: CompactProtectedReferenceSet,
    ) -> Result<Self, HazardLeaseDenial> {
        Self::from_barrier(
            HazardLeaseKind::ForegroundRead,
            hazard.reachability_barrier(),
            footprint,
        )
    }

    pub fn from_barrier(
        kind: HazardLeaseKind,
        barrier: PhysicalReadReachabilityBarrier,
        footprint: CompactProtectedReferenceSet,
    ) -> Result<Self, HazardLeaseDenial> {
        if barrier.footprint_basis() != footprint.declared_footprint_basis() {
            return Err(HazardLeaseDenial::HazardFootprintMismatch {
                expected: barrier.footprint_basis(),
                observed: footprint.declared_footprint_basis(),
            });
        }
        if footprint.ranges().ranges().is_empty() {
            return Err(HazardLeaseDenial::MissingProtectedRanges);
        }
        Ok(Self {
            kind,
            barrier,
            footprint,
        })
    }

    pub fn from_buffer_pool_pin(
        reference: CurrentGenerationPhysicalReference,
        scratch: ReadPlanAdmissionScratchArena,
    ) -> Result<Self, PhysicalReadPlanAdmissionDenial> {
        let protected = ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
            [reference],
            scratch.clone(),
        )?;
        let footprint =
            CompactProtectedReferenceSet::from_reference_set_with_scratch(protected, scratch)?;
        let release = crate::PhysicalReadPlanReleaseSemantics::reader_releases_all();
        let barrier = PhysicalReadReachabilityBarrier::from_footprint_basis(
            footprint.footprint_basis(),
            release,
        );
        Ok(Self {
            kind: HazardLeaseKind::BufferPoolPin,
            barrier,
            footprint,
        })
    }

    pub const fn kind(&self) -> HazardLeaseKind {
        self.kind
    }

    pub const fn barrier(&self) -> PhysicalReadReachabilityBarrier {
        self.barrier
    }

    pub const fn footprint(&self) -> &CompactProtectedReferenceSet {
        &self.footprint
    }
}
