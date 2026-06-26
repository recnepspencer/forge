use crate::{
    ResidentFrameCounterSnapshot, ResidentFrameDenial, ResidentFrameIdentity,
    ResidentFrameLoadRequest, ResidentFrameSlot, ResidentFrameToken,
};
use forge_store_physical_format::PhysicalReference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameAdmission {
    identity: ResidentFrameIdentity,
    request: ResidentFrameLoadRequest,
    hit_miss_report: ResidentFrameHitMissReport,
}

impl ResidentFrameAdmission {
    pub(crate) const fn new(
        identity: ResidentFrameIdentity,
        request: ResidentFrameLoadRequest,
        hit_miss_report: ResidentFrameHitMissReport,
    ) -> Self {
        Self {
            identity,
            request,
            hit_miss_report,
        }
    }

    pub const fn identity(self) -> ResidentFrameIdentity {
        self.identity
    }

    pub const fn slot(self) -> ResidentFrameSlot {
        self.identity.slot()
    }

    pub const fn resident_frame_token(self) -> ResidentFrameToken {
        self.identity.token()
    }

    pub const fn request(self) -> ResidentFrameLoadRequest {
        self.request
    }

    pub const fn hit_miss_report(self) -> ResidentFrameHitMissReport {
        self.hit_miss_report
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameHitMissReport {
    counters: ResidentFrameCounterSnapshot,
}

impl ResidentFrameHitMissReport {
    pub(crate) const fn new(counters: ResidentFrameCounterSnapshot) -> Self {
        Self { counters }
    }

    pub const fn counters(self) -> ResidentFrameCounterSnapshot {
        self.counters
    }

    pub const fn hit_count(self) -> u64 {
        self.counters.hit_count()
    }

    pub const fn miss_count(self) -> u64 {
        self.counters.miss_count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameResidence {
    identity: ResidentFrameIdentity,
    request: ResidentFrameLoadRequest,
}

impl ResidentFrameResidence {
    pub(crate) const fn new(
        identity: ResidentFrameIdentity,
        request: ResidentFrameLoadRequest,
    ) -> Self {
        Self { identity, request }
    }

    pub const fn identity(self) -> ResidentFrameIdentity {
        self.identity
    }

    pub const fn resident_frame_token(self) -> ResidentFrameToken {
        self.identity.token()
    }

    pub const fn physical_reference(self) -> PhysicalReference {
        self.request.reference().reference()
    }

    pub const fn frame_size_bytes(self) -> u64 {
        self.request.frame_size().as_bytes()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentGenerationSeparationProof {
    previous_identity: ResidentFrameIdentity,
    replacement_identity: ResidentFrameIdentity,
    stale_token: ResidentFrameToken,
    stale_denial: ResidentFrameDenial,
    counters: ResidentFrameCounterSnapshot,
}

impl ResidentGenerationSeparationProof {
    pub(crate) const fn new(
        previous_identity: ResidentFrameIdentity,
        replacement_identity: ResidentFrameIdentity,
        stale_token: ResidentFrameToken,
        stale_denial: ResidentFrameDenial,
        counters: ResidentFrameCounterSnapshot,
    ) -> Self {
        Self {
            previous_identity,
            replacement_identity,
            stale_token,
            stale_denial,
            counters,
        }
    }

    pub const fn previous_identity(self) -> ResidentFrameIdentity {
        self.previous_identity
    }

    pub const fn replacement_identity(self) -> ResidentFrameIdentity {
        self.replacement_identity
    }

    pub const fn stale_token(self) -> ResidentFrameToken {
        self.stale_token
    }

    pub const fn stale_denial(self) -> ResidentFrameDenial {
        self.stale_denial
    }

    pub const fn counters(self) -> ResidentFrameCounterSnapshot {
        self.counters
    }
}
