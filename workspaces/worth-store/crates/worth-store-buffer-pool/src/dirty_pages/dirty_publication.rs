use crate::{
    DirtyPageCount, DirtyPageCounterSnapshot, DirtyPageIdentity, LeaseEpoch,
    ResidentFrameLoadRequest,
};
use worth_store_physical_format::{PageGenerationCell, PhysicalGenerationAuthority};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirtyPublicationEpoch(u64);

impl DirtyPublicationEpoch {
    pub(crate) const fn initial() -> Self {
        Self(0)
    }

    pub(crate) const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DirtyPublicationPlan {
    identity: DirtyPageIdentity,
    request: ResidentFrameLoadRequest,
    lease_epoch: LeaseEpoch,
    dirty_epoch: DirtyPublicationEpoch,
    counters: DirtyPageCounterSnapshot,
}

impl DirtyPublicationPlan {
    pub(crate) const fn new(
        identity: DirtyPageIdentity,
        request: ResidentFrameLoadRequest,
        lease_epoch: LeaseEpoch,
        dirty_epoch: DirtyPublicationEpoch,
        counters: DirtyPageCounterSnapshot,
    ) -> Self {
        Self {
            identity,
            request,
            lease_epoch,
            dirty_epoch,
            counters,
        }
    }

    pub const fn dirty_identity(&self) -> DirtyPageIdentity {
        self.identity
    }

    pub const fn frame_size_bytes(&self) -> u64 {
        self.request.frame_size().as_bytes()
    }

    pub const fn frame_request(&self) -> ResidentFrameLoadRequest {
        self.request
    }

    pub const fn lease_epoch(&self) -> LeaseEpoch {
        self.lease_epoch
    }

    pub(crate) const fn dirty_publication_epoch(&self) -> DirtyPublicationEpoch {
        self.dirty_epoch
    }

    pub const fn counters(&self) -> DirtyPageCounterSnapshot {
        self.counters
    }

    pub const fn write_scheduling_attempt_count(&self) -> u64 {
        self.counters.write_scheduling_attempt_count()
    }

    pub const fn proves_durability(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyPublicationReceipt {
    identity: DirtyPageIdentity,
    request: ResidentFrameLoadRequest,
    released_dirty_pages: DirtyPageCount,
    counters: DirtyPageCounterSnapshot,
}

impl DirtyPublicationReceipt {
    pub(crate) const fn new(
        identity: DirtyPageIdentity,
        request: ResidentFrameLoadRequest,
        released_dirty_pages: DirtyPageCount,
        counters: DirtyPageCounterSnapshot,
    ) -> Self {
        Self {
            identity,
            request,
            released_dirty_pages,
            counters,
        }
    }

    pub const fn dirty_identity(self) -> DirtyPageIdentity {
        self.identity
    }

    pub fn page_generation(self) -> PageGenerationCell {
        let reference = self.request.reference().reference();
        let segment_id = reference
            .segment_id()
            .expect("resident frame requests are admitted page-slot references");
        let page_id = reference
            .page_id()
            .expect("resident frame requests are admitted page-slot references");
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .page_cell(segment_id, page_id)
            .with_page_generation(reference.generation())
    }

    pub const fn counters(self) -> DirtyPageCounterSnapshot {
        self.counters
    }

    pub const fn released_dirty_pages(self) -> DirtyPageCount {
        self.released_dirty_pages
    }

    pub const fn write_scheduling_attempt_count(self) -> u64 {
        self.counters.write_scheduling_attempt_count()
    }

    pub const fn proves_durability(self) -> bool {
        false
    }
}
