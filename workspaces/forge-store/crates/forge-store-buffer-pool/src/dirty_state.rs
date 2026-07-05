use crate::{
    AccessPolicyBufferLifecycle, DirtyPageCounterSnapshot, ResidentFrameIdentity,
    ResidentFrameLoadRequest, ResidentFrameToken,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyPageAccessOrigin {
    StoreBuffer,
    Mmap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyPageIdentity {
    identity: ResidentFrameIdentity,
}

impl DirtyPageIdentity {
    pub(crate) const fn new(identity: ResidentFrameIdentity) -> Self {
        Self { identity }
    }

    pub const fn resident_frame_token(self) -> ResidentFrameToken {
        self.identity.token()
    }

    pub const fn resident_frame_identity(self) -> ResidentFrameIdentity {
        self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyPageState {
    identity: DirtyPageIdentity,
    request: ResidentFrameLoadRequest,
    access_origin: DirtyPageAccessOrigin,
    counters: DirtyPageCounterSnapshot,
}

impl DirtyPageState {
    pub(crate) const fn new(
        identity: DirtyPageIdentity,
        request: ResidentFrameLoadRequest,
        access_origin: DirtyPageAccessOrigin,
        counters: DirtyPageCounterSnapshot,
    ) -> Self {
        Self {
            identity,
            request,
            access_origin,
            counters,
        }
    }

    pub const fn identity(self) -> DirtyPageIdentity {
        self.identity
    }

    pub const fn resident_frame_token(self) -> ResidentFrameToken {
        self.identity.resident_frame_token()
    }

    pub const fn frame_size_bytes(self) -> u64 {
        self.request.frame_size().as_bytes()
    }

    pub const fn access_origin(self) -> DirtyPageAccessOrigin {
        self.access_origin
    }

    pub const fn access_policy_lifecycle_proof(self) -> AccessPolicyBufferLifecycle {
        match self.access_origin {
            DirtyPageAccessOrigin::StoreBuffer => AccessPolicyBufferLifecycle::dirty_page_tracked(),
            DirtyPageAccessOrigin::Mmap => AccessPolicyBufferLifecycle::dirty_mmap_page(),
        }
    }

    pub const fn counters(self) -> DirtyPageCounterSnapshot {
        self.counters
    }

    pub const fn dirty_page_count(self) -> crate::DirtyPageCount {
        self.counters.dirty_pages()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirtyShutdownPosture {
    CleanNoDirtyPages,
    UnflushedDirtyPagesRemain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyShutdownReport {
    posture: DirtyShutdownPosture,
    counters: DirtyPageCounterSnapshot,
}

impl DirtyShutdownReport {
    pub(crate) const fn new(
        posture: DirtyShutdownPosture,
        counters: DirtyPageCounterSnapshot,
    ) -> Self {
        Self { posture, counters }
    }

    pub const fn posture(self) -> DirtyShutdownPosture {
        self.posture
    }

    pub const fn counters(self) -> DirtyPageCounterSnapshot {
        self.counters
    }

    pub fn unflushed_dirty_pages(self) -> crate::DirtyPageCount {
        self.counters.unflushed_dirty_pages()
    }

    pub fn unflushed_dirty_bytes(self) -> crate::DirtyByteCount {
        self.counters.unflushed_dirty_bytes()
    }

    pub const fn proves_durability(self) -> bool {
        false
    }
}
