use crate::{
    EvictionCounterSnapshot, LeaseEpoch, ResidentByteCount, ResidentFrameDenial,
    ResidentFrameDenialKind, ResidentFrameIdentity, ResidentFrameLoadRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionPressure {
    resident_frames_requested: u32,
}

impl EvictionPressure {
    pub fn for_resident_frames(
        resident_frames_requested: u32,
    ) -> Result<Self, ResidentFrameDenial> {
        if resident_frames_requested == 0 {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::EvictionPressureIsZero,
            ));
        }
        Ok(Self {
            resident_frames_requested,
        })
    }

    pub const fn resident_frames_requested(self) -> u32 {
        self.resident_frames_requested
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionCandidateSet {
    selected_identity: ResidentFrameIdentity,
    resident_frames_scanned: u64,
    candidate_count: u64,
    protected_exclusions: EvictionProtectionSummary,
    policy_rank_count: u64,
    counters: EvictionCounterSnapshot,
}

impl EvictionCandidateSet {
    pub(crate) const fn new(
        selected_identity: ResidentFrameIdentity,
        resident_frames_scanned: u64,
        candidate_count: u64,
        protected_exclusions: EvictionProtectionSummary,
        policy_rank_count: u64,
        counters: EvictionCounterSnapshot,
    ) -> Self {
        Self {
            selected_identity,
            resident_frames_scanned,
            candidate_count,
            protected_exclusions,
            policy_rank_count,
            counters,
        }
    }

    pub const fn selected_identity(self) -> ResidentFrameIdentity {
        self.selected_identity
    }

    pub const fn resident_frames_scanned(self) -> u64 {
        self.resident_frames_scanned
    }

    pub const fn candidate_count(self) -> u64 {
        self.candidate_count
    }

    pub const fn protected_exclusions(self) -> EvictionProtectionSummary {
        self.protected_exclusions
    }

    pub const fn policy_rank_count(self) -> u64 {
        self.policy_rank_count
    }

    pub const fn includes_protected_frames(self) -> bool {
        false
    }

    pub const fn counters(self) -> EvictionCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EvictionPlan {
    pressure: EvictionPressure,
    candidate_set: EvictionCandidateSet,
    request: ResidentFrameLoadRequest,
    lease_epoch: LeaseEpoch,
}

impl EvictionPlan {
    pub(crate) const fn new(
        pressure: EvictionPressure,
        candidate_set: EvictionCandidateSet,
        request: ResidentFrameLoadRequest,
        lease_epoch: LeaseEpoch,
    ) -> Self {
        Self {
            pressure,
            candidate_set,
            request,
            lease_epoch,
        }
    }

    pub const fn pressure(&self) -> EvictionPressure {
        self.pressure
    }

    pub const fn candidate_set(&self) -> EvictionCandidateSet {
        self.candidate_set
    }

    pub const fn selected_identity(&self) -> ResidentFrameIdentity {
        self.candidate_set.selected_identity()
    }

    pub const fn frame_size_bytes(&self) -> u64 {
        self.request.frame_size().as_bytes()
    }

    pub(crate) const fn lease_epoch(&self) -> LeaseEpoch {
        self.lease_epoch
    }

    pub const fn counters(&self) -> EvictionCounterSnapshot {
        self.candidate_set.counters()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionReceipt {
    identity: ResidentFrameIdentity,
    released_resident_bytes: ResidentByteCount,
    counters: EvictionCounterSnapshot,
}

impl EvictionReceipt {
    pub(crate) const fn new(
        identity: ResidentFrameIdentity,
        released_resident_bytes: ResidentByteCount,
        counters: EvictionCounterSnapshot,
    ) -> Self {
        Self {
            identity,
            released_resident_bytes,
            counters,
        }
    }

    pub const fn identity(self) -> ResidentFrameIdentity {
        self.identity
    }

    pub const fn evicted_frame_count(self) -> u32 {
        1
    }

    pub const fn released_resident_bytes(self) -> ResidentByteCount {
        self.released_resident_bytes
    }

    pub const fn counters(self) -> EvictionCounterSnapshot {
        self.counters
    }

    pub const fn proves_durability(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionProtectionSummary {
    pinned_count: u64,
    dirty_unpublished_count: u64,
    verifier_protected_count: u64,
    recovery_protected_count: u64,
    streaming_protected_count: u64,
}

impl EvictionProtectionSummary {
    pub const fn empty() -> Self {
        Self {
            pinned_count: 0,
            dirty_unpublished_count: 0,
            verifier_protected_count: 0,
            recovery_protected_count: 0,
            streaming_protected_count: 0,
        }
    }

    pub(crate) const fn with_reason(self, reason: EvictionProtectionReason) -> Self {
        match reason {
            EvictionProtectionReason::Pinned => Self {
                pinned_count: self.pinned_count + 1,
                ..self
            },
            EvictionProtectionReason::DirtyUnpublished => Self {
                dirty_unpublished_count: self.dirty_unpublished_count + 1,
                ..self
            },
            EvictionProtectionReason::VerifierProtected => Self {
                verifier_protected_count: self.verifier_protected_count + 1,
                ..self
            },
            EvictionProtectionReason::RecoveryProtected => Self {
                recovery_protected_count: self.recovery_protected_count + 1,
                ..self
            },
            EvictionProtectionReason::StreamingProtected => Self {
                streaming_protected_count: self.streaming_protected_count + 1,
                ..self
            },
        }
    }

    pub const fn contains(self, reason: EvictionProtectionReason) -> bool {
        match reason {
            EvictionProtectionReason::Pinned => self.pinned_count > 0,
            EvictionProtectionReason::DirtyUnpublished => self.dirty_unpublished_count > 0,
            EvictionProtectionReason::VerifierProtected => self.verifier_protected_count > 0,
            EvictionProtectionReason::RecoveryProtected => self.recovery_protected_count > 0,
            EvictionProtectionReason::StreamingProtected => self.streaming_protected_count > 0,
        }
    }

    pub const fn is_empty(self) -> bool {
        self.pinned_count == 0
            && self.dirty_unpublished_count == 0
            && self.verifier_protected_count == 0
            && self.recovery_protected_count == 0
            && self.streaming_protected_count == 0
    }

    pub const fn total_protected_reasons(self) -> u64 {
        self.pinned_count
            + self.dirty_unpublished_count
            + self.verifier_protected_count
            + self.recovery_protected_count
            + self.streaming_protected_count
    }

    pub const fn pinned_count(self) -> u64 {
        self.pinned_count
    }

    pub const fn dirty_unpublished_count(self) -> u64 {
        self.dirty_unpublished_count
    }

    pub const fn verifier_protected_count(self) -> u64 {
        self.verifier_protected_count
    }

    pub const fn recovery_protected_count(self) -> u64 {
        self.recovery_protected_count
    }

    pub const fn streaming_protected_count(self) -> u64 {
        self.streaming_protected_count
    }

    pub(crate) const fn merge(self, other: Self) -> Self {
        Self {
            pinned_count: self.pinned_count + other.pinned_count,
            dirty_unpublished_count: self.dirty_unpublished_count + other.dirty_unpublished_count,
            verifier_protected_count: self.verifier_protected_count
                + other.verifier_protected_count,
            recovery_protected_count: self.recovery_protected_count
                + other.recovery_protected_count,
            streaming_protected_count: self.streaming_protected_count
                + other.streaming_protected_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionProtectionReason {
    Pinned,
    DirtyUnpublished,
    VerifierProtected,
    RecoveryProtected,
    StreamingProtected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedFrameDenial {
    reasons: EvictionProtectionSummary,
    counters: EvictionCounterSnapshot,
}

impl ProtectedFrameDenial {
    pub(crate) const fn new(
        reasons: EvictionProtectionSummary,
        counters: EvictionCounterSnapshot,
    ) -> Self {
        Self { reasons, counters }
    }

    pub const fn reasons(self) -> EvictionProtectionSummary {
        self.reasons
    }

    pub const fn counters(self) -> EvictionCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameProtectionReceipt {
    identity: ResidentFrameIdentity,
    reason: EvictionProtectionReason,
}

impl FrameProtectionReceipt {
    pub(crate) const fn new(
        identity: ResidentFrameIdentity,
        reason: EvictionProtectionReason,
    ) -> Self {
        Self { identity, reason }
    }

    pub const fn identity(self) -> ResidentFrameIdentity {
        self.identity
    }

    pub const fn reason(self) -> EvictionProtectionReason {
        self.reason
    }
}
